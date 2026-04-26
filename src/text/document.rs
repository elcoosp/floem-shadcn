use std::{cell::RefCell, rc::Rc};

use floem::{
    kurbo::Point,
    peniko::Color,
    reactive::{RwSignal, SignalGet, SignalUpdate, SignalWith},
    text::{Attrs, AttrsList, FamilyOwned, LineHeightValue, TextLayout, FontWeight},
};
use floem_editor_core::{
    buffer::{
        Buffer, InvalLines,
        rope_text::{RopeText, RopeTextRef},
    },
    command::{EditCommand, MoveCommand},
    cursor::{ColPosition, CursorAffinity},
    editor::EditType,
    mode::Mode,
    selection::{InsertDrift, SelRegion, Selection},
};
use lapce_xi_rope::{Delta, Rope, RopeDelta};
use ui_events::pointer::{PointerButton, PointerButtonEvent, PointerState, PointerUpdate};

use super::TextLayoutLines;

#[derive(Clone)]
#[allow(clippy::type_complexity)]
pub struct Document {
    buffer: RwSignal<Buffer>,
    text_layouts: Rc<RefCell<TextLayoutLines>>,
    width: RwSignal<f64>,
    active: RwSignal<bool>,
    cursor: RwSignal<SelRegion>,
    horiz: RwSignal<Option<ColPosition>>,
    text_color: RwSignal<Color>,
    font_size: RwSignal<f32>,
    line_height: RwSignal<LineHeightValue>,
    font_weight: RwSignal<FontWeight>,
    font_family: RwSignal<Vec<FamilyOwned>>,
    on_update: Rc<RefCell<Vec<Box<dyn Fn(&str)>>>>,
}

impl Document {
    pub fn new(text: impl Into<Rope>) -> Self {
        let text = text.into();
        let buffer = RwSignal::new(Buffer::new(text));
        let cursor = RwSignal::new(SelRegion::caret(0, CursorAffinity::Forward));
        let width = RwSignal::new(10.0);
        let horiz = RwSignal::new(None);
        let active = RwSignal::new(false);
        let text_color = RwSignal::new(Color::BLACK);
        let font_size = RwSignal::new(14.0);
        let line_height = RwSignal::new(LineHeightValue::Normal(1.5));
        let font_weight = RwSignal::new(FontWeight::NORMAL);
        let font_family = RwSignal::new(Vec::new());

        Self {
            buffer,
            text_layouts: Rc::new(RefCell::new(TextLayoutLines::builder().build())),
            cursor,
            active,
            width,
            horiz,
            text_color,
            font_size,
            line_height,
            font_weight,
            font_family,
            on_update: Rc::new(RefCell::new(Vec::new())),
        }
    }

    pub fn text(&self) -> String {
        self.buffer
            .with_untracked(|b| b.text().slice_to_cow(..).into_owned())
    }

    pub fn buffer(&self) -> RwSignal<Buffer> {
        self.buffer
    }

    pub fn cursor(&self) -> RwSignal<SelRegion> {
        self.cursor
    }

    pub fn width(&self) -> RwSignal<f64> {
        self.width
    }

    pub fn text_layouts(&self) -> &Rc<RefCell<TextLayoutLines>> {
        &self.text_layouts
    }

    pub fn on_update(&self, f: impl Fn(&str) + 'static) {
        self.on_update.borrow_mut().push(Box::new(f));
    }

    pub fn set_width(&self, width: f64) {
        if self.width.get_untracked() == width { return; }
        self.width.set(width);
        self.rebuild_layouts(width);
    }

    pub fn set_text_color(&self, color: Color) {
        if self.text_color.get_untracked() == color { return; }
        self.text_color.set(color);
        let width = self.width.get_untracked();
        if width > 0.0 { self.rebuild_layouts(width); }
    }

    pub fn set_font_size(&self, size: f32) {
        if self.font_size.get_untracked() == size { return; }
        self.font_size.set(size);
        let width = self.width.get_untracked();
        if width > 0.0 { self.rebuild_layouts(width); }
    }

    pub fn set_line_height(&self, line_height: LineHeightValue) {
        if self.line_height.get_untracked() == line_height { return; }
        self.line_height.set(line_height);
        let width = self.width.get_untracked();
        if width > 0.0 { self.rebuild_layouts(width); }
    }

    pub fn set_font_weight(&self, weight: FontWeight) {
        if self.font_weight.get_untracked() == weight { return; }
        self.font_weight.set(weight);
        let width = self.width.get_untracked();
        if width > 0.0 { self.rebuild_layouts(width); }
    }

    pub fn set_font_family(&self, family: Vec<FamilyOwned>) {
        if self.font_family.get_untracked() == family { return; }
        self.font_family.set(family);
        let width = self.width.get_untracked();
        if width > 0.0 { self.rebuild_layouts(width); }
    }

    fn rebuild_layouts(&self, width: f64) {
        let text_color = self.text_color.get_untracked();
        let font_size = self.font_size.get_untracked();
        let line_height = self.line_height.get_untracked();
        let font_weight = self.font_weight.get_untracked();
        let font_family = self.font_family.get_untracked();

        let mut attrs = Attrs::default()
            .font_size(font_size)
            .line_height(line_height)
            .weight(font_weight)
            .color(text_color);

        if !font_family.is_empty() { attrs = attrs.family(&font_family); }

        let attrs = AttrsList::new(attrs);
        let mut builder = TextLayoutLines::builder();
        let reference_layout = TextLayout::new_with_text(" ", attrs.clone(), None);
        builder.set_default_from_layout(&reference_layout);

        self.buffer.with_untracked(|buffer| {
            for line in buffer.text().lines_raw(0..buffer.text().len()) {
                let mut text_layout = TextLayout::new_with_text(&line, attrs.clone(), None);
                text_layout.set_size(width as f32, f32::MAX);
                builder.push_text_layout(&text_layout);
            }
        });
        *self.text_layouts.borrow_mut() = builder.build();
    }

    pub fn insert_text(&self, text: &str) {
        self.edit([(self.cursor.get_untracked(), text)], EditType::InsertChars);
    }

    pub fn edit<'a, I>(&self, edits: I, edit_type: EditType)
    where I: IntoIterator<Item = (SelRegion, &'a str)> {
        let edits = edits.into_iter().map(|(region, s)| {
            (Selection::region(region.start, region.end, CursorAffinity::Forward), s)
        });
        let delta = self.buffer.try_update(|b| b.edit(edits, edit_type)).unwrap();
        self.apply_delta(&delta);
    }

    fn apply_delta(&self, delta: &(Rope, RopeDelta, InvalLines)) {
        let width = self.width.get_untracked();
        let text_color = self.text_color.get_untracked();
        let font_size = self.font_size.get_untracked();
        let line_height = self.line_height.get_untracked();
        let font_weight = self.font_weight.get_untracked();
        let font_family = self.font_family.get_untracked();

        let mut attrs = Attrs::default()
            .font_size(font_size)
            .line_height(line_height)
            .weight(font_weight)
            .color(text_color);
        if !font_family.is_empty() { attrs = attrs.family(&font_family); }
        let attrs = AttrsList::new(attrs);

        let (rope, rope_delta, inval_lines) = delta;
        {
            let mut text_layouts = self.text_layouts.borrow_mut();
            let mut builder = TextLayoutLines::builder();
            self.buffer.with_untracked(|buffer| {
                let start = buffer.offset_of_line(inval_lines.start_line);
                let end = buffer.offset_of_line(inval_lines.start_line + inval_lines.new_count);
                for line in buffer.text().lines_raw(start..end) {
                    let mut text_layout = TextLayout::new_with_text(&line, attrs.clone(), None);
                    text_layout.set_size(width as f32, f32::MAX);
                    builder.push_text_layout(&text_layout);
                }
            });
            let new = builder.build();
            let rope_ref = RopeTextRef::new(rope);
            let start = rope_ref.offset_of_line(inval_lines.start_line);
            let end = rope_ref.offset_of_line(inval_lines.start_line + inval_lines.inval_count);
            let lines_delta = Delta::simple_edit(start..end, new.tree, rope_ref.len());
            text_layouts.apply_delta(lines_delta);
        }
        self.cursor.update(|c| {
            let selection = Selection::region(c.start, c.end, c.affinity);
            let new_selection = selection.apply_delta(rope_delta, true, InsertDrift::Default);
            if let Some(region) = new_selection.regions().first() { *c = *region; }
        });
        self.buffer.with_untracked(|buffer| {
            let text = &buffer.text().slice_to_cow(..);
            for on_update in self.on_update.borrow().iter() { on_update(text); }
        });
    }

    pub fn run_move_command(&self, command: &MoveCommand, modify: bool) {
        match command {
            MoveCommand::Left => {
                let region = self.cursor.get_untracked();
                let new_offset = if modify || region.is_caret() {
                    self.buffer.with_untracked(|b| b.move_left(region.end, Mode::Insert, 1))
                } else { region.min() };
                self.set_offset(new_offset, modify);
                self.horiz.set(None);
            }
            MoveCommand::Right => {
                let region = self.cursor.get_untracked();
                let new_offset = if modify || region.is_caret() {
                    self.buffer.with_untracked(|b| b.move_right(region.end, Mode::Insert, 1))
                } else { region.max() };
                self.set_offset(new_offset, modify);
                self.horiz.set(None);
            }
            MoveCommand::Up => {
                let region = self.cursor.get_untracked();
                let offset = region.end;
                let lines = self.text_layouts.borrow();
                let vline = lines.vline_of_offset(offset);
                let horiz = if let Some(horiz) = self.horiz.get_untracked() {
                    horiz
                } else {
                    let point = lines.point_of_offset(offset);
                    self.horiz.set(Some(ColPosition::Col(point.x)));
                    ColPosition::Col(point.x)
                };
                let new_offset = if vline == 0 {
                    0
                } else {
                    let vline = vline - 1;
                    match horiz {
                        ColPosition::FirstNonBlank => 0,
                        ColPosition::Start => lines.offset_of_vline(vline),
                        ColPosition::End => {
                            let next_vline_offset = lines.offset_of_vline(vline + 1);
                            if next_vline_offset > 0 { next_vline_offset - 1 } else { 0 }
                        }
                        ColPosition::Col(x) => {
                            let offset = lines.offset_of_vline(vline);
                            let point = lines.point_of_offset(offset);
                            lines.offset_of_point(Point::new(x, point.glyph_top))
                        }
                    }
                };
                drop(lines);
                self.set_offset(new_offset, modify);
            }
            MoveCommand::Down => {
                let region = self.cursor.get_untracked();
                let offset = region.end;
                let lines = self.text_layouts.borrow();
                let vline = lines.vline_of_offset(offset);
                let horiz = if let Some(horiz) = self.horiz.get_untracked() {
                    horiz
                } else {
                    let point = lines.point_of_offset(offset);
                    self.horiz.set(Some(ColPosition::Col(point.x)));
                    ColPosition::Col(point.x)
                };
                let last_vline = lines.vline_of_offset(lines.utf8_len());
                let new_offset = if last_vline == vline {
                    lines.utf8_len()
                } else {
                    let vline = vline + 1;
                    match horiz {
                        ColPosition::FirstNonBlank => lines.offset_of_vline(vline),
                        ColPosition::Start => lines.offset_of_vline(vline),
                        ColPosition::End => {
                            let next_vline_offset = lines.offset_of_vline(vline + 1);
                            if next_vline_offset > 0 { next_vline_offset - 1 } else { lines.utf8_len() }
                        }
                        ColPosition::Col(x) => {
                            let offset = lines.offset_of_vline(vline);
                            let point = lines.point_of_offset(offset);
                            lines.offset_of_point(Point::new(x, point.glyph_top))
                        }
                    }
                };
                drop(lines);
                self.set_offset(new_offset, modify);
            }
            MoveCommand::LineStart => {
                let region = self.cursor.get_untracked();
                let lines = self.text_layouts.borrow();
                let vline = lines.vline_of_offset(region.end);
                let new_offset = lines.offset_of_vline(vline);
                drop(lines);
                self.set_offset(new_offset, modify);
                self.horiz.set(Some(ColPosition::Start));
            }
            MoveCommand::LineEnd => {
                let region = self.cursor.get_untracked();
                let lines = self.text_layouts.borrow();
                let vline = lines.vline_of_offset(region.end);
                let last_vline = lines.vline_of_offset(lines.utf8_len());
                let doc_len = lines.utf8_len();
                let new_offset = if vline >= last_vline {
                    doc_len
                } else {
                    let next_vline_offset = lines.offset_of_vline(vline + 1);
                    next_vline_offset.saturating_sub(1).min(doc_len)
                };
                drop(lines);
                self.set_offset(new_offset, modify);
                self.horiz.set(Some(ColPosition::End));
            }
            MoveCommand::DocumentStart => {
                self.set_offset(0, modify);
                self.horiz.set(None);
            }
            MoveCommand::DocumentEnd => {
                let doc_len = self.text_layouts.borrow().utf8_len();
                self.set_offset(doc_len, modify);
                self.horiz.set(None);
            }
            MoveCommand::WordBackward => {
                let region = self.cursor.get_untracked();
                let new_offset = self.buffer.with_untracked(|b| b.move_word_backward(region.end, Mode::Insert));
                self.set_offset(new_offset, modify);
                self.horiz.set(None);
            }
            MoveCommand::WordForward => {
                let region = self.cursor.get_untracked();
                let new_offset = self.buffer.with_untracked(|b| b.move_word_forward(region.end));
                self.set_offset(new_offset, modify);
                self.horiz.set(None);
            }
            _ => {}
        }
    }

    pub fn run_edit_command(&self, command: &EditCommand) {
        match command {
            EditCommand::InsertNewLine => { self.edit([(self.cursor.get_untracked(), "\n")], EditType::InsertNewline); }
            EditCommand::InsertTab => { self.edit([(self.cursor.get_untracked(), "\t")], EditType::InsertChars); }
            EditCommand::DeleteBackward => {
                let region = self.cursor.get_untracked();
                let region = if region.is_caret() {
                    let new_offset = self.buffer.with_untracked(|b| b.move_left(region.start, Mode::Insert, 1));
                    SelRegion::new(region.start, new_offset, CursorAffinity::Forward, None)
                } else { region };
                self.edit([(region, "")], EditType::Delete);
            }
            EditCommand::DeleteForward => {
                let region = self.cursor.get_untracked();
                let region = if region.is_caret() {
                    let new_offset = self.buffer.with_untracked(|b| b.move_right(region.start, Mode::Insert, 1));
                    SelRegion::new(region.start, new_offset, CursorAffinity::Forward, None)
                } else { region };
                self.edit([(region, "")], EditType::Delete);
            }
            EditCommand::DeleteWordBackward => {
                let region = self.cursor.get_untracked();
                let region = if region.is_caret() {
                    let new_offset = self.buffer.with_untracked(|b| b.move_word_backward(region.start, Mode::Insert));
                    SelRegion::new(region.start, new_offset, CursorAffinity::Forward, None)
                } else { region };
                self.edit([(region, "")], EditType::Delete);
            }
            EditCommand::DeleteWordForward => {
                let region = self.cursor.get_untracked();
                let region = if region.is_caret() {
                    let new_offset = self.buffer.with_untracked(|b| b.move_word_forward(region.start));
                    SelRegion::new(region.start, new_offset, CursorAffinity::Forward, None)
                } else { region };
                self.edit([(region, "")], EditType::Delete);
            }
            EditCommand::DeleteToBeginningOfLine => {
                let region = self.cursor.get_untracked();
                let lines = self.text_layouts.borrow();
                let vline = lines.vline_of_offset(region.end);
                let line_start = lines.offset_of_vline(vline);
                drop(lines);
                if region.end > line_start {
                    let delete_region = SelRegion::new(line_start, region.end, CursorAffinity::Forward, None);
                    self.edit([(delete_region, "")], EditType::Delete);
                }
            }
            _ => {}
        }
    }

    pub fn set_offset(&self, offset: usize, modify: bool) {
        let region = self.cursor.get_untracked();
        let region = if modify {
            SelRegion::new(region.start, offset, CursorAffinity::Forward, None)
        } else {
            SelRegion::caret(offset, CursorAffinity::Forward)
        };
        self.cursor.set(region);
    }

    pub fn select_all(&self) {
        let doc_len = self.text_layouts.borrow().utf8_len();
        let region = SelRegion::new(0, doc_len, CursorAffinity::Forward, None);
        self.cursor.set(region);
    }

    pub fn copy(&self) -> bool {
        let region = self.cursor.get_untracked();
        if region.is_caret() { return false; }
        let start = region.min(); let end = region.max();
        let text = self.buffer.with_untracked(|b| b.text().slice_to_cow(start..end).into_owned());
        if text.is_empty() { return false; }
        floem::Clipboard::set_contents(text).is_ok()
    }

    pub fn cut(&self) -> bool {
        let region = self.cursor.get_untracked();
        if region.is_caret() { return false; }
        let start = region.min(); let end = region.max();
        let text = self.buffer.with_untracked(|b| b.text().slice_to_cow(start..end).into_owned());
        if text.is_empty() { return false; }
        if floem::Clipboard::set_contents(text).is_ok() { self.edit([(region, "")], EditType::Delete); true } else { false }
    }

    pub fn paste(&self, filter_newlines: bool) -> bool {
        let mut content = match floem::Clipboard::get_contents() { Ok(c) => c, Err(_) => return false };
        if filter_newlines { content.retain(|c| c != '\n' && c != '\r'); }
        if content.is_empty() { return false; }
        let region = self.cursor.get_untracked();
        self.edit([(region, content.as_str())], EditType::InsertChars);
        true
    }

    pub fn pointer_down(&self, event: &PointerButtonEvent) {
        if event.button == Some(PointerButton::Primary) { self.left_click(&event.state); }
        else if event.button == Some(PointerButton::Secondary) { self.double_click(&event.state); }
    }

    fn left_click(&self, state: &PointerState) {
        match state.count {
            1 => { self.active.set(true); self.single_click(state); }
            2 => self.double_click(state),
            3 => self.triple_click(state),
            _ => {}
        }
    }

    fn single_click(&self, state: &PointerState) {
        let lines = self.text_layouts.borrow();
        let pos = state.logical_point();
        let new_offset = lines.offset_of_point(pos);
        let shift = state.modifiers.shift();
        self.set_offset(new_offset, shift);
        self.horiz.set(None);
    }

    fn double_click(&self, state: &PointerState) {
        let lines = self.text_layouts.borrow();
        let pos = state.logical_point();
        let mouse_offset = lines.offset_of_point(pos);
        let (start, end) = self.buffer.with_untracked(|b| b.select_word(mouse_offset));
        self.cursor.set(SelRegion::new(start, end, CursorAffinity::Forward, None));
        self.horiz.set(None);
    }

    fn triple_click(&self, state: &PointerState) {
        let lines = self.text_layouts.borrow();
        let pos = state.logical_point();
        let mouse_offset = lines.offset_of_point(pos);
        let vline = lines.vline_of_offset(mouse_offset);
        let start = lines.offset_of_vline(vline);
        let end = lines.offset_of_vline(vline + 1);
        self.cursor.set(SelRegion::new(start, end, CursorAffinity::Forward, None));
        self.horiz.set(None);
    }

    pub fn pointer_move(&self, event: &PointerUpdate) {
        if self.active.get_untracked() {
            let lines = self.text_layouts.borrow();
            let pos = event.current.logical_point();
            let offset = lines.offset_of_point(pos);
            let cursor = self.cursor.get_untracked();
            if cursor.end != offset {
                self.cursor.set(SelRegion::new(cursor.start, offset, CursorAffinity::Forward, None));
            }
        }
    }

    pub fn pointer_up(&self, _event: &PointerButtonEvent) { self.active.set(false); }
}
