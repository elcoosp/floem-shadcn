#[cfg(test)]
use super::KeyPress;
use super::{
    CURSOR_BLINK_INTERVAL_MS, Command, Document, Keymap, apply_styles_to_document, extract_padding,
    extract_text_styles, get_glyph_dimensions, is_cursor_visible,
};
use crate::theme::ShadcnThemeProp;
use floem::{
    IntoView, Renderer, View, ViewId,
    action::exec_after,
    context::{ComputeLayoutCx, PaintCx},
    event::{Event, EventListener, EventPropagation},
    kurbo::{Point, Rect, Size},
    reactive::{Effect, RwSignal, SignalGet, SignalTrack, SignalUpdate, SignalWith},
    style::{CursorStyle as StyleCursorStyle, Style},
    taffy::{Dimension, Overflow},
    views::{Decorators, Empty, Scroll},
};
use floem_editor_core::buffer::rope_text::RopeText;
use std::time::{Duration, Instant};
#[cfg(test)]
use ui_events::keyboard::NamedKey;
use ui_events::{
    keyboard::{Key, KeyState, KeyboardEvent, Modifiers},
    pointer::PointerEvent,
};

const RESIZE_HANDLE_SIZE: f64 = 16.0;

pub struct TextArea {
    id: ViewId,
    scroll_id: ViewId,
    dummy_child_id: ViewId,
    doc: RwSignal<Document>,
    padding: RwSignal<(f64, f64, f64, f64)>,
    viewport: RwSignal<Rect>,
    parent_size: RwSignal<Size>,
    child_height: RwSignal<f64>,
    last_cursor_action: RwSignal<Instant>,
    resize_enabled: RwSignal<bool>,
    resize_size: RwSignal<Option<Size>>,
    is_resizing: RwSignal<bool>,
    resize_start_pos: RwSignal<Point>,
    resize_start_size: RwSignal<Size>,
    min_size: RwSignal<Size>,
    max_size: RwSignal<Option<Size>>,
}

impl Default for TextArea {
    fn default() -> Self {
        Self::new()
    }
}
impl TextArea {
    pub fn new() -> Self {
        Self::with_text("")
    }
    pub fn with_text(text: impl Into<String>) -> Self {
        Self::with_text_and_id(text, ViewId::new())
    }
    pub fn with_text_and_id(text: impl Into<String>, id: ViewId) -> Self {
        Self::with_text_id_and_keymap(text, id, Keymap::multi_line())
    }
    pub fn with_text_id_and_keymap(text: impl Into<String>, id: ViewId, keymap: Keymap) -> Self {
        let child_height = RwSignal::new(0.0);
        let padding = RwSignal::new((0.0, 0.0, 0.0, 0.0));
        let viewport = RwSignal::new(Rect::ZERO);
        let parent_size = RwSignal::new(Size::ZERO);
        let doc = Document::new(text.into());
        let doc_signal = RwSignal::new(doc);
        let last_cursor_action = RwSignal::new(Instant::now());
        let resize_enabled = RwSignal::new(false);
        let resize_size = RwSignal::new(None);
        let is_resizing = RwSignal::new(false);
        let resize_start_pos = RwSignal::new(Point::ZERO);
        let resize_start_size = RwSignal::new(Size::ZERO);
        let min_size = RwSignal::new(Size::new(50.0, 30.0));
        let max_size = RwSignal::new(None);
        let cursor_signal = doc_signal.get_untracked().cursor();
        let dummy_child = Empty::new().style(|s| s.width(10.0).height(0.0));
        let dummy_child_id = dummy_child.id();
        let scroll_child_size = RwSignal::new(Size::ZERO);
        let scroll_view = Scroll::new(dummy_child)
            .style(move |s| {
                let p = padding.get();
                s.absolute().size_full().margin_top(-p.0).margin_left(-p.3);
            })
            .on_child_size(move |s| {
                scroll_child_size.set(s);
            })
            .ensure_visible(move || {
                let cursor = cursor_signal.get();
                let padding = padding.get_untracked();
                scroll_child_size.track();
                let offset = cursor.end;
                let doc = doc_signal.get_untracked();
                let pt = doc.text_layouts().borrow().point_of_offset(offset);
                Rect::from_origin_size(
                    (0.0, pt.line_top),
                    (1.0, pt.line_bottom - pt.line_top + padding.0 + padding.2),
                )
            })
            .on_scroll(move |v| {
                viewport.set(v);
            });
        let scroll_id = scroll_view.id();
        id.set_children_vec(vec![scroll_view.into_any()]);

        let keymap = std::sync::Arc::new(keymap);
        let keymap_clone = keymap.clone();

        id.add_event_listener(
            EventListener::PointerDown,
            Box::new(move |event| {
                if let Event::Pointer(PointerEvent::Down(pe)) = event {
                    let pos = pe.state.position;
                    if resize_enabled.get_untracked() {
                        let layout = id.get_layout().unwrap_or_default();
                        let w = layout.size.width as f64;
                        let h = layout.size.height as f64;
                        if pos.x >= w - RESIZE_HANDLE_SIZE && pos.y >= h - RESIZE_HANDLE_SIZE {
                            is_resizing.set(true);
                            resize_start_pos.set(Point::new(pos.x, pos.y));
                            resize_start_size.set(Size::new(w, h));
                            id.request_active();
                            return EventPropagation::Stop;
                        }
                    }
                    let padding = padding.get_untracked();
                    let vp = viewport.get_untracked();
                    let mut adj = pe.clone();
                    adj.state.position.x -= padding.3;
                    adj.state.position.y -= padding.0 - vp.y0;
                    id.request_active();
                    id.request_focus();
                    doc_signal.get_untracked().pointer_down(&adj);
                    last_cursor_action.set(Instant::now());
                }
                EventPropagation::Stop
            }),
        );

        id.add_event_listener(
            EventListener::PointerMove,
            Box::new(move |event| {
                if let Event::Pointer(PointerEvent::Move(pe)) = event {
                    if is_resizing.get_untracked() {
                        let cp = pe.current.position;
                        let sp = resize_start_pos.get_untracked();
                        let ss = resize_start_size.get_untracked();
                        let min = min_size.get_untracked();
                        let max: Option<Size> = max_size.get_untracked();
                        let dx = cp.x - sp.x;
                        let dy = cp.y - sp.y;
                        let mut nw = (ss.width + dx).max(min.width);
                        let mut nh = (ss.height + dy).max(min.height);
                        if let Some(mx) = max {
                            nw = nw.min(mx.width);
                            nh = nh.min(mx.height);
                        }
                        resize_size.set(Some(Size::new(nw, nh)));
                        id.request_layout();
                        return EventPropagation::Stop;
                    }
                    let padding = padding.get_untracked();
                    let vp = viewport.get_untracked();
                    let mut adj = pe.clone();
                    adj.current.position.x -= padding.3;
                    adj.current.position.y -= padding.0 - vp.y0;
                    doc_signal.get_untracked().pointer_move(&adj);
                    last_cursor_action.set(Instant::now());
                }
                EventPropagation::Stop
            }),
        );

        id.add_event_listener(
            EventListener::PointerUp,
            Box::new(move |event| {
                if let Event::Pointer(PointerEvent::Up(pe)) = event {
                    if is_resizing.get_untracked() {
                        is_resizing.set(false);
                        return EventPropagation::Stop;
                    }
                    let padding = padding.get_untracked();
                    let vp = viewport.get_untracked();
                    let mut adj = pe.clone();
                    adj.state.position.x -= padding.3;
                    adj.state.position.y -= padding.0 - vp.y0;
                    doc_signal.get_untracked().pointer_up(&adj);
                }
                EventPropagation::Stop
            }),
        );

        id.add_event_listener(
            EventListener::KeyDown,
            Box::new(move |event| {
                let Event::Key(KeyboardEvent {
                    state: KeyState::Down,
                    key,
                    modifiers,
                    ..
                }) = event
                else {
                    return EventPropagation::Continue;
                };
                let command = keymap_clone.get(key, modifiers);
                let document = doc_signal.get_untracked();
                if let Some(command) = command {
                    let shift_held = modifiers.shift();
                    match command {
                        Command::Edit(ec) => {
                            document.run_edit_command(ec);
                            id.request_layout();
                        }
                        Command::Move(mc) => {
                            document.run_move_command(mc, shift_held);
                            scroll_id.request_layout();
                        }
                        Command::SelectAll => document.select_all(),
                        Command::Copy => {
                            document.copy();
                        }
                        Command::Cut => {
                            if document.cut() {
                                id.request_layout();
                            }
                        }
                        Command::Paste => {
                            if document.paste(false) {
                                id.request_layout();
                            }
                        }
                    }
                    last_cursor_action.set(Instant::now());
                    return EventPropagation::Stop;
                }
                let mut mods = *modifiers;
                mods.set(Modifiers::SHIFT, false);
                #[cfg(target_os = "macos")]
                mods.set(Modifiers::ALT, false);
                if mods.is_empty() {
                    if !matches!(key, Key::Character(_)) {
                        return EventPropagation::Continue;
                    }
                    if let Key::Character(c) = key {
                        document.insert_text(c);
                        id.request_layout();
                        last_cursor_action.set(Instant::now());
                    }
                }
                EventPropagation::Stop
            }),
        );

        id.add_event_listener(
            EventListener::ImeCommit,
            Box::new(move |event| {
                if let Event::ImeCommit(text) = event {
                    doc_signal.get_untracked().insert_text(text);
                    id.request_layout();
                    last_cursor_action.set(Instant::now());
                }
                EventPropagation::Stop
            }),
        );

        Self {
            id,
            scroll_id,
            dummy_child_id,
            doc: doc_signal,
            padding,
            viewport,
            parent_size,
            child_height,
            last_cursor_action,
            resize_enabled,
            resize_size,
            is_resizing,
            resize_start_pos,
            resize_start_size,
            min_size,
            max_size,
        }
    }
    pub fn doc(&self) -> RwSignal<Document> {
        self.doc
    }
    pub fn scroll_id(&self) -> ViewId {
        self.scroll_id
    }
    pub fn child_height(&self) -> RwSignal<f64> {
        self.child_height
    }
    pub fn viewport(&self) -> RwSignal<Rect> {
        self.viewport
    }
    pub fn on_update(self, cb: impl Fn(&str) + 'static) -> Self {
        self.doc.get_untracked().on_update(cb);
        self
    }
    pub fn value(self, set_value: impl Fn() -> String + 'static) -> Self {
        let doc = self.doc;
        Effect::new(move |_| {
            let nv = set_value();
            let ct = doc.with_untracked(|d| d.text());
            if ct == nv {
                return;
            }
            doc.with_untracked(|d| {
                let end = d.buffer().with_untracked(|b| b.text().len());
                use floem_editor_core::{
                    cursor::CursorAffinity, editor::EditType, selection::SelRegion,
                };
                d.edit(
                    [(
                        SelRegion::new(0, end, CursorAffinity::Forward, None),
                        nv.as_str(),
                    )],
                    EditType::Other,
                );
            });
        });
        self
    }
    pub fn text(&self) -> String {
        self.doc.get_untracked().text()
    }
    pub fn resizable(self, enabled: bool) -> Self {
        self.resize_enabled.set(enabled);
        self
    }
    pub fn is_resizable(&self) -> bool {
        self.resize_enabled.get_untracked()
    }
    pub fn resize_size(&self) -> RwSignal<Option<Size>> {
        self.resize_size
    }
    pub fn min_resize_size(self, size: Size) -> Self {
        self.min_size.set(size);
        self
    }
    pub fn max_resize_size(self, size: Size) -> Self {
        self.max_size.set(Some(size));
        self
    }
    pub fn is_resizing(&self) -> bool {
        self.is_resizing.get_untracked()
    }
}

impl View for TextArea {
    fn id(&self) -> ViewId {
        self.id
    }
    fn view_style(&self) -> Option<Style> {
        let rs = self.resize_size;
        Some(
            Style::new()
                .cursor(StyleCursorStyle::Text)
                .focusable(true)
                .set(floem::style::OverflowX, Overflow::Hidden)
                .set(floem::style::OverflowY, Overflow::Scroll)
                .apply_if(rs.get().is_some(), move |s| {
                    let sz = rs.get().unwrap();
                    s.width(sz.width).height(sz.height)
                }),
        )
    }
    fn compute_layout(&mut self, cx: &mut ComputeLayoutCx) -> Option<Rect> {
        let layout = self.id.get_layout().unwrap_or_default();
        let style = self.id.get_combined_style();
        let builtin_style = style.builtin();
        let padding = extract_padding(&builtin_style, layout.size.width as f64);
        if padding != self.padding.get_untracked() {
            self.padding.set(padding);
        }
        let width = layout.size.width as f64 - padding.3 - padding.1;
        let height = layout.size.height as f64 - padding.0 - padding.2;
        let parent_size = Size::new(width, height);
        let text_styles = extract_text_styles(&builtin_style);
        let doc = self.doc.get_untracked();
        apply_styles_to_document(&doc, &text_styles);
        doc.set_width(width);
        let child_height = {
            let lines = doc.text_layouts().borrow();
            lines.point_of_offset(lines.utf8_len()).line_bottom + padding.0 + padding.2
        };
        if child_height != self.child_height.get_untracked() {
            self.child_height.set(child_height);
            let node = self.dummy_child_id.taffy_node();
            let mut sty = self
                .id
                .taffy()
                .borrow()
                .style(node)
                .cloned()
                .unwrap_or_default();
            sty.size.height = Dimension::length(child_height as f32);
            self.id.set_taffy_style(node, sty);
        }
        if self.parent_size.get_untracked() != parent_size {
            self.parent_size.set(parent_size);
        }
        cx.compute_view_layout(self.scroll_id);
        None
    }
    fn paint(&mut self, cx: &mut PaintCx) {
        let padding = self.padding.get_untracked();
        let viewport = self.viewport.get_untracked();
        let style = self.id.get_combined_style();
        let text_styles = extract_text_styles(&style.builtin());
        let theme = style.get(ShadcnThemeProp);
        let selection_color = theme.primary.multiply_alpha(0.2);
        cx.save();
        cx.clip(
            &self
                .parent_size
                .get_untracked()
                .to_rect()
                .with_origin(Point::new(padding.3, padding.0))
                .inflate(2.0, 0.0),
        );
        let doc = self.doc.get_untracked();
        let lines = doc.text_layouts().borrow();
        let min_vline = lines.vline_of_height(viewport.y0).saturating_sub(1);
        let max_vline = lines.vline_of_height(viewport.y1) + 1;
        if cx.is_focused(self.id) {
            let cursor = doc.cursor().get_untracked();
            if cursor.is_caret() {
                let elapsed = self
                    .last_cursor_action
                    .get_untracked()
                    .elapsed()
                    .as_millis();
                if is_cursor_visible(elapsed) {
                    let p = lines.point_of_offset(cursor.end);
                    let (ct, ch) = get_glyph_dimensions(
                        p.glyph_top,
                        p.glyph_bottom,
                        lines.default_glyph_top(),
                        lines.default_glyph_height(),
                    );
                    let rect = Rect::from_origin_size(
                        (p.x + padding.3 - 1.0, ct + padding.0 - viewport.y0),
                        (2.0, ch),
                    );
                    cx.fill(&rect, text_styles.text_color, 0.0);
                }
                let id = self.id;
                exec_after(Duration::from_millis(CURSOR_BLINK_INTERVAL_MS), move |_| {
                    id.request_paint();
                });
            } else {
                let sv = lines.vline_of_offset(cursor.min());
                let ev = lines.vline_of_offset(cursor.max());
                let so = lines.offset_of_vline(sv.max(min_vline));
                let eo = lines.offset_of_vline(ev.min(max_vline));
                for line in lines.visual_lines(so..eo + 1) {
                    let x0 = if line.line_i == sv {
                        lines.point_of_offset(cursor.min()).x
                    } else {
                        0.0
                    };
                    let x1 = if line.line_i == ev {
                        lines.point_of_offset(cursor.max()).x
                    } else {
                        line.line_w as f64
                    };
                    let rect = Rect::from_origin_size(
                        (
                            x0 + padding.3,
                            line.line_top as f64 + padding.0 - viewport.y0,
                        ),
                        (x1 - x0, line.line_height as f64),
                    );
                    cx.fill(&rect, selection_color, 0.0);
                }
            }
        }
        let mo = lines.offset_of_vline(min_vline);
        let mx = lines.offset_of_vline(max_vline + 1);
        let layout = lines.visual_lines(mo..mx + 1);
        cx.draw_text_with_layout(layout, Point::new(padding.3, padding.0 - viewport.y0));
        cx.restore();
        cx.paint_view(self.scroll_id);
        if self.resize_enabled.get_untracked() {
            let layout = self.id.get_layout().unwrap_or_default();
            let w = layout.size.width as f64;
            let h = layout.size.height as f64;
            let hc = theme.muted_foreground.multiply_alpha(0.5);
            let ds = 3.0;
            let dot = 2.0;
            let bx = w - 4.0;
            let by = h - 4.0;
            for row in 0..3 {
                for col in 0..=row {
                    let x = bx - row as f64 * ds + col as f64 * ds;
                    let y = by - col as f64 * ds;
                    cx.fill(
                        &Rect::from_origin_size((x - dot / 2., y - dot / 2.), (dot, dot)),
                        hc,
                        0.0,
                    );
                }
            }
        }
    }
}
