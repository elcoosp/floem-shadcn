use crate::theme::ShadcnThemeExt;
use floem::context::LayoutChanged;
use floem::prelude::*;
use floem::reactive::{RwSignal, SignalGet, SignalUpdate};
use floem::style::CursorStyle;
use floem::views::Decorators;
use floem::{HasViewId, ViewId};

#[derive(Clone)]
/// The `SelectItemData`.
pub struct SelectItemData {
    /// The item.
    pub value: String,
    /// The item.
    pub label: String,
    /// The item.
    pub disabled: bool,
}
impl SelectItemData {
    /// Creates a new it.
    pub fn new(v: impl Into<String>, l: impl Into<String>) -> Self {
        Self {
            value: v.into(),
            label: l.into(),
            disabled: false,
        }
    }
    #[must_use]
    /// The disabled.
    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}
/// The Select.
pub struct Select {
    id: ViewId,
    selected: RwSignal<Option<String>>,
    placeholder: String,
    items: Vec<SelectItemData>,
    disabled: bool,
    trigger_origin: RwSignal<floem::kurbo::Point>,
    trigger_size: RwSignal<floem::kurbo::Size>,
}

/// Build the closed-select trigger chip (label + caret + layout tracking).
fn select_trigger(
    sel: RwSignal<Option<String>>,
    placeholder: String,
    items: Vec<SelectItemData>,
    disabled: bool,
    trigger_origin: RwSignal<floem::kurbo::Point>,
    trigger_size: RwSignal<floem::kurbo::Size>,
) -> impl IntoView {
    floem::views::Stack::horizontal((
        floem::views::Label::derived(move || {
            if let Some(v) = sel.get() {
                items
                    .iter()
                    .find(|i| i.value == v)
                    .map(|i| i.label.clone())
                    .unwrap_or(v)
            } else {
                placeholder.clone()
            }
        })
        .style(move |s| {
            s.with_shadcn_theme(move |s, t| {
                let hv = sel.get().is_some();
                s.flex_grow(1.0_f32).font_size(14.0).color(if hv {
                    t.foreground
                } else {
                    t.muted_foreground
                })
            })
        }),
        floem::views::Label::new("▼").style(|s| {
            s.with_shadcn_theme(move |s, t| {
                s.font_size(10.0)
                    .color(t.muted_foreground)
                    .flex_shrink(0.0_f32)
            })
        }),
    ))
    .style(move |s| {
        s.with_shadcn_theme(move |s, t| {
            s.min_width(120.0)
                .height(36.0)
                .padding_left(12.0)
                .padding_right(12.0)
                .padding_top(8.0)
                .padding_bottom(8.0)
                .gap(8.0)
                .items_center()
                .border(1.0)
                .border_color(t.input)
                .border_radius(6.0)
                .background(t.background)
                .box_shadow_blur(2.0)
                .box_shadow_color(floem::peniko::Color::from_rgba8(0, 0, 0, 25))
                .apply_if(disabled, |s| s.cursor(CursorStyle::Default))
                .apply_if(!disabled, |s| {
                    s.cursor(CursorStyle::Pointer)
                        .hover(|s| s.border_color(t.ring))
                })
        })
    })
    .on_event_stop(
        LayoutChanged::listener(),
        move |_cx, event: &LayoutChanged| {
            trigger_origin.set(event.new_window_origin);
            trigger_size.set(event.new_box.size());
        },
    )
}

/// Build the dropdown row for one select item.
fn select_item_view(
    item: &SelectItemData,
    selected_signal: RwSignal<Option<String>>,
    open_signal: RwSignal<bool>,
) -> Box<dyn View> {
    let item_value = item.value.clone();
    let item_label = item.label.clone();
    let item_disabled = item.disabled;
    let check_value = item_value.clone();
    let hover_value = item_value.clone();
    let click_value = item_value;
    Box::new(
        floem::views::Container::new(
            floem::views::Stack::horizontal((
                floem::views::Label::new(item_label)
                    .style(|s| s.font_size(14.0).flex_grow(1.0_f32)),
                floem::views::Label::new("✓").style(move |s| {
                    let v = check_value.clone();
                    s.with_shadcn_theme(move |s, t| {
                        let is_sel = selected_signal.get() == Some(v.clone());
                        s.size(16.0, 16.0)
                            .font_size(14.0)
                            .color(t.foreground)
                            .items_center()
                            .justify_center()
                            .flex_shrink(0.0_f32)
                            .apply_if(!is_sel, |s| s.display(floem::style::Display::None))
                    })
                }),
            ))
            .style(|s| s.width_full().items_center().gap(8.0)),
        )
        .style(move |s| {
            let v = hover_value.clone();
            s.with_shadcn_theme(move |s, t| {
                let is_sel = selected_signal.get() == Some(v.clone());
                let base = s
                    .width_full()
                    .padding_top(6.0)
                    .padding_bottom(6.0)
                    .padding_left(8.0)
                    .padding_right(8.0)
                    .items_center()
                    .border_radius(3.0)
                    .cursor(if item_disabled {
                        CursorStyle::Default
                    } else {
                        CursorStyle::Pointer
                    });
                if is_sel {
                    base.background(t.accent).color(t.accent_foreground)
                } else if item_disabled {
                    base.color(t.muted_foreground).opacity(0.5_f32)
                } else {
                    base.color(t.foreground)
                        .hover(|s| s.background(t.accent).color(t.accent_foreground))
                }
            })
        })
        .on_event_stop(floem::event::listener::Click, move |_, ()| {
            if !item_disabled {
                selected_signal.set(Some(click_value.clone()));
                open_signal.set(false);
            }
        }),
    ) as _
}
impl Select {
    #[must_use]
    /// Creates a new it.
    pub fn new(s: RwSignal<Option<String>>) -> Self {
        Self {
            id: ViewId::new(),
            selected: s,
            placeholder: "Select...".into(),
            items: vec![],
            disabled: false,
            trigger_origin: RwSignal::new(floem::kurbo::Point::ZERO),
            trigger_size: RwSignal::new(floem::kurbo::Size::ZERO),
        }
    }
    #[must_use]
    /// The placeholder.
    pub fn placeholder(mut self, p: impl Into<String>) -> Self {
        self.placeholder = p.into();
        self
    }
    #[must_use]
    /// The items.
    pub fn items(mut self, i: Vec<SelectItemData>) -> Self {
        self.items = i;
        self
    }
    #[must_use]
    /// The disabled.
    pub fn disabled(mut self, d: bool) -> Self {
        self.disabled = d;
        self
    }
}
impl HasViewId for Select {
    fn view_id(&self) -> ViewId {
        self.id
    }
}
impl IntoView for Select {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        let sel = self.selected;
        let ph = self.placeholder;
        let items = self.items;
        let disabled = self.disabled;
        let trigger_origin = self.trigger_origin;
        let trigger_size = self.trigger_size;
        let is_open = RwSignal::new(false);
        let items_for_trigger = items.clone();
        let trigger = select_trigger(
            sel,
            ph,
            items_for_trigger,
            disabled,
            trigger_origin,
            trigger_size,
        );
        let trigger = if disabled {
            trigger.into_any()
        } else {
            trigger
                .on_event_stop(floem::event::listener::Click, move |_, ()| {
                    is_open.update(|v| *v = !*v);
                })
                .into_any()
        };
        let to = trigger_origin;
        let ts = trigger_size;
        let item_views: Vec<Box<dyn View>> = items
            .iter()
            .map(|item| select_item_view(item, self.selected, is_open))
            .collect();
        let items_container = floem::views::Stack::vertical_from_iter(item_views)
            .style(|s| s.width_full().max_height(300.0));
        let dropdown = floem::views::Container::new(items_container).style(move |s| {
            s.with_shadcn_theme(move |s, t| {
                let origin = to.get();
                let tsize = ts.get();
                s.position(floem::style::Position::Absolute)
                    .inset_left(origin.x)
                    .inset_top(origin.y + tsize.height + 6.0)
                    .min_width(tsize.width.max(120.0))
                    .margin_top(6.0)
                    .padding(4.0)
                    .background(t.popover)
                    .color(t.popover_foreground)
                    .border(1.0)
                    .border_color(t.border)
                    .border_radius(6.0)
                    .box_shadow_blur(4.0)
                    .box_shadow_color(floem::peniko::Color::from_rgba8(0, 0, 0, 40))
                    .z_index(100)
                    .flex_direction(floem::style::FlexDirection::Column)
                    .apply_if(!is_open.get(), |s| s.display(floem::style::Display::None))
            })
        });
        Box::new(floem::views::Stack::new((trigger, dropdown)))
    }
}
/// The `SelectTrigger`.
pub struct SelectTrigger<V> {
    id: ViewId,
    child: V,
    is_open: RwSignal<bool>,
}
impl<V: IntoView + 'static> SelectTrigger<V> {
    /// Creates a new it.
    pub fn new(c: V, io: RwSignal<bool>) -> Self {
        Self {
            id: ViewId::new(),
            child: c,
            is_open: io,
        }
    }
}
impl<V: IntoView + 'static> HasViewId for SelectTrigger<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}
impl<V: IntoView + 'static> IntoView for SelectTrigger<V> {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        let io = self.is_open;
        Box::new(
            floem::views::Container::with_id(self.id, self.child)
                .style(|s| {
                    s.with_shadcn_theme(move |s, t| {
                        s.height(36.0)
                            .padding_left(12.0)
                            .padding_right(12.0)
                            .padding_top(8.0)
                            .padding_bottom(8.0)
                            .gap(8.0)
                            .items_center()
                            .border(1.0)
                            .border_color(t.input)
                            .border_radius(6.0)
                            .background(t.background)
                            .box_shadow_blur(2.0)
                            .box_shadow_color(floem::peniko::Color::from_rgba8(0, 0, 0, 25))
                            .cursor(CursorStyle::Pointer)
                            .hover(|s| s.border_color(t.ring))
                    })
                })
                .on_event_stop(floem::event::listener::Click, move |_, ()| {
                    io.update(|v| *v = !*v);
                }),
        )
    }
}
/// The `SelectContent`.
pub struct SelectContent<V> {
    id: ViewId,
    child: V,
    is_open: RwSignal<bool>,
}
impl<V: IntoView + 'static> SelectContent<V> {
    /// Creates a new it.
    pub fn new(c: V, io: RwSignal<bool>) -> Self {
        Self {
            id: ViewId::new(),
            child: c,
            is_open: io,
        }
    }
}
impl<V: IntoView + 'static> HasViewId for SelectContent<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}
impl<V: IntoView + 'static> IntoView for SelectContent<V> {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        let io = self.is_open;
        Box::new(
            floem::views::Container::with_id(self.id, self.child).style(move |s| {
                s.with_shadcn_theme(move |s, t| {
                    let base = s
                        .position(floem::style::Position::Absolute)
                        .inset_top_pct(100.0)
                        .inset_left(0.0)
                        .inset_right(0.0)
                        .margin_top(6.0)
                        .padding(4.0)
                        .background(t.popover)
                        .color(t.popover_foreground)
                        .border(1.0)
                        .border_color(t.border)
                        .border_radius(6.0)
                        .box_shadow_blur(4.0)
                        .box_shadow_color(floem::peniko::Color::from_rgba8(0, 0, 0, 40))
                        .z_index(100)
                        .flex_direction(floem::style::FlexDirection::Column);
                    if io.get() {
                        base
                    } else {
                        base.display(floem::style::Display::None)
                    }
                })
            }),
        )
    }
}
/// The `SelectItem`.
pub struct SelectItem;
impl SelectItem {
    #[must_use]
    /// Creates a new it.
    pub fn new(_: impl Into<String>, _: impl Into<String>) -> Self {
        Self
    }
    #[must_use]
    /// The disabled.
    pub fn disabled(self, _: bool) -> Self {
        self
    }
    #[must_use]
    /// The bind.
    pub fn bind(self, _: RwSignal<Option<String>>) -> Self {
        self
    }
    #[must_use]
    /// The auto close.
    pub fn auto_close(self, _: RwSignal<bool>) -> Self {
        self
    }
}
impl HasViewId for SelectItem {
    fn view_id(&self) -> ViewId {
        ViewId::new()
    }
}
impl IntoView for SelectItem {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        Box::new(floem::views::Empty::new())
    }
}
/// The `SelectLabel`.
pub struct SelectLabel;
impl SelectLabel {
    #[must_use]
    /// Creates a new it.
    pub fn new(_: impl Into<String>) -> Self {
        Self
    }
}
impl HasViewId for SelectLabel {
    fn view_id(&self) -> ViewId {
        ViewId::new()
    }
}
impl IntoView for SelectLabel {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        Box::new(floem::views::Empty::new())
    }
}
/// The `SelectSeparator`.
pub struct SelectSeparator;
impl SelectSeparator {
    #[must_use]
    /// Creates a new it.
    pub fn new() -> Self {
        Self
    }
}
impl Default for SelectSeparator {
    fn default() -> Self {
        Self::new()
    }
}
impl HasViewId for SelectSeparator {
    fn view_id(&self) -> ViewId {
        ViewId::new()
    }
}
impl IntoView for SelectSeparator {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        Box::new(floem::views::Empty::new())
    }
}
/// The `SelectGroup`.
pub struct SelectGroup<V> {
    id: ViewId,
    child: V,
}
impl<V: IntoView + 'static> SelectGroup<V> {
    /// Creates a new it.
    pub fn new(_: impl Into<String>, child: V) -> Self {
        Self {
            id: ViewId::new(),
            child,
        }
    }
}
impl<V: IntoView + 'static> HasViewId for SelectGroup<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}
impl<V: IntoView + 'static> IntoView for SelectGroup<V> {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        Box::new(self.child.into_view())
    }
}
