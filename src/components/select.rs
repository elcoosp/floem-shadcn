//! Select component with builder-style API
//!
//! Based on shadcn/ui Select component - a dropdown for selecting from a list.
//!
//! # Example
//!
//! ```
//! use floem::reactive::RwSignal;
//! use floem_shadcn::components::select::*;
//!
//! let selected = RwSignal::new(Some("option1".to_string()));
//!
//! Select::new(selected)
//!     .placeholder("Select an option...")
//!     .items(vec![
//!         SelectItemData::new("option1", "Option 1"),
//!         SelectItemData::new("option2", "Option 2"),
//!         SelectItemData::new("option3", "Option 3"),
//!     ]);
//! ```

use floem::prelude::*;
use floem::reactive::{RwSignal, SignalGet, SignalUpdate};
use floem::style::CursorStyle;
use floem::views::{Decorators, Overlay};
use floem::{HasViewId, ViewId};
use floem_tailwind::TailwindExt;

use crate::theme::ShadcnThemeExt;

#[derive(Clone)]
pub struct SelectItemData {
    pub value: String,
    pub label: String,
    pub disabled: bool,
}

impl SelectItemData {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self { value: value.into(), label: label.into(), disabled: false }
    }
    pub fn disabled(mut self) -> Self { self.disabled = true; self }
}

pub struct Select {
    id: ViewId,
    selected: RwSignal<Option<String>>,
    placeholder: String,
    items: Vec<SelectItemData>,
    disabled: bool,
}

impl Select {
    pub fn new(selected: RwSignal<Option<String>>) -> Self {
        Self { id: ViewId::new(), selected, placeholder: "Select...".to_string(), items: Vec::new(), disabled: false }
    }
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self { self.placeholder = placeholder.into(); self }
    pub fn items(mut self, items: Vec<SelectItemData>) -> Self { self.items = items; self }
    pub fn disabled(mut self, disabled: bool) -> Self { self.disabled = disabled; self }
}

impl HasViewId for Select { fn view_id(&self) -> ViewId { self.id } }

impl IntoView for Select {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate { self.into_view() }

    fn into_view(self) -> Self::V {
        let selected = self.selected;
        let placeholder = self.placeholder;
        let items = self.items;
        let disabled = self.disabled;
        let is_open = RwSignal::new(false);

        let items_for_trigger = items.clone();

        let trigger = floem::views::Stack::horizontal((
            floem::views::Label::derived(move || {
                if let Some(val) = selected.get() {
                    items_for_trigger.iter().find(|i| i.value == val).map(|i| i.label.clone()).unwrap_or(val)
                } else { placeholder.clone() }
            })
            .style(move |s| {
                s.with_shadcn_theme(move |s, t| {
                    let has_value = selected.get().is_some();
                    s.flex_grow(1.0).text_sm().color(if has_value { t.foreground } else { t.muted_foreground })
                })
            }),
            floem::views::Label::new("▼").style(|s| {
                s.with_shadcn_theme(move |s, t| {
                    s.font_size(10.0).color(t.muted_foreground).flex_shrink(0.0)
                })
            }),
        ))
        .style(move |s| {
            s.with_shadcn_theme(move |s, t| {
                s.min_width(120.0)
                    .h_9()
                    .px_3()
                    .py_2()
                    .gap_2()
                    .items_center()
                    .border_1()
                    .border_color(t.input)
                    .rounded_md()
                    .background(t.background)
                    .shadow_sm()
                    .apply_if(disabled, |s| s.cursor(CursorStyle::Default))
                    .apply_if(!disabled, |s| {
                        s.cursor(CursorStyle::Pointer)
                            .hover(|s| s.border_color(t.ring))
                    })
            })
        });

        let trigger = if !disabled {
            trigger
                .on_click_stop(move |_| { is_open.update(|v| *v = !*v); })
                .into_any()
        } else {
            trigger.into_any()
        };

        let item_views: Vec<Box<dyn View>> = items.iter().enumerate().map(|(_idx, item)| {
            let value = item.value.clone();
            let label = item.label.clone();
            let d = item.disabled;
            let sel2 = selected;
            let io2 = is_open;
            let v0 = value.clone();
            let v1 = value.clone();
            let v2 = value.clone();
            Box::new(
                floem::views::Container::new(
                    floem::views::Stack::horizontal((
                        floem::views::Label::new(label).style(|s| s.text_sm().flex_grow(1.0)),
                        floem::views::Label::new("✓").style(move |s| {
                            let v = v0.clone();
                            s.with_shadcn_theme(move |s, t| {
                                let is_selected = sel2.get() == Some(v.clone());
                                s.size_4().text_sm().color(t.foreground).items_center().justify_center().flex_shrink(0.0)
                                    .apply_if(!is_selected, |s| s.display(floem::style::Display::None))
                            })
                        }),
                    )).style(|s| s.width_full().items_center().gap_2()),
                )
                .style({
                    let v = v1.clone();
                    move |s| s.with_shadcn_theme(|s, t| {
                        let is_sel = sel2.get() == Some(v.clone());
                        let base = s.width_full().padding_top(6.0).padding_bottom(6.0).padding_left(8.0).padding_right(8.0)
                            .items_center().rounded_sm().cursor(if d { CursorStyle::Default } else { CursorStyle::Pointer });
                        if is_sel { base.background(t.accent).color(t.accent_foreground) }
                        else if d { base.color(t.muted_foreground).opacity_50() }
                        else { base.color(t.foreground).hover(|s| s.background(t.accent).color(t.accent_foreground)) }
                    })
                })
                .on_click_stop(move |_| {
                    if !d { sel2.set(Some(v2.clone())); io2.set(false); }
                }),
            ) as Box<dyn View>
        }).collect();

        let items_container = floem::views::Stack::vertical_from_iter(item_views)
            .style(|s| s.width_full().max_height(300.0));

        let dropdown_overlay = Overlay::new(
            floem::views::Stack::new((
                floem::views::Empty::new()
                    .style(move |s| s.absolute().inset_0())
                    .on_click_stop(move |_| { is_open.set(false); }),
                floem::views::Container::new(items_container).style(move |s| {
                    s.with_shadcn_theme(move |s, t| {
                        s.position(floem::style::Position::Absolute)
                            .inset_top_pct(100.0).inset_left(0.0).inset_right(0.0)
                            .margin_top(6.0).p_1().min_width(120.0)
                            .background(t.popover).color(t.popover_foreground)
                            .border_1().border_color(t.border).rounded_md().shadow_md().z_index(100)
                            .flex_direction(floem::style::FlexDirection::Column)
                    })
                }),
            ))
            .style(move |s| {
                let open = is_open.get();
                s.fixed().inset_0().width_full().height_full()
                    .apply_if(!open, |s| s.hide())
            }),
        );

        Box::new(floem::views::Stack::new((trigger, dropdown_overlay)))
    }
}

pub struct SelectTrigger<V> {
    id: ViewId,
    child: V,
    is_open: RwSignal<bool>,
}

impl<V: IntoView + 'static> SelectTrigger<V> {
    pub fn new(child: V, is_open: RwSignal<bool>) -> Self {
        Self { id: ViewId::new(), child, is_open }
    }
}

impl<V: IntoView + 'static> HasViewId for SelectTrigger<V> { fn view_id(&self) -> ViewId { self.id } }

impl<V: IntoView + 'static> IntoView for SelectTrigger<V> {
    type V = Box<dyn View>;
    fn into_view(self) -> Self::V {
        let is_open = self.is_open;
        Box::new(
            floem::views::Container::with_id(self.id, self.child)
                .style(|s| {
                    s.with_shadcn_theme(move |s, t| {
                        s.h_9().px_3().py_2().gap_2().items_center()
                            .border_1().border_color(t.input).rounded_md().background(t.background).shadow_sm()
                            .cursor(CursorStyle::Pointer).hover(|s| s.border_color(t.ring))
                    })
                })
                .on_click_stop(move |_| { is_open.update(|v| *v = !*v); }),
        )
    }
}

pub struct SelectContent<V> {
    id: ViewId,
    child: V,
    is_open: RwSignal<bool>,
}

impl<V: IntoView + 'static> SelectContent<V> {
    pub fn new(child: V, is_open: RwSignal<bool>) -> Self {
        Self { id: ViewId::new(), child, is_open }
    }
}

impl<V: IntoView + 'static> HasViewId for SelectContent<V> { fn view_id(&self) -> ViewId { self.id } }

impl<V: IntoView + 'static> IntoView for SelectContent<V> {
    type V = Box<dyn View>;
    fn into_view(self) -> Self::V {
        let is_open = self.is_open;
        Box::new(
            floem::views::Container::with_id(self.id, self.child).style(move |s| {
                s.with_shadcn_theme(move |s, t| {
                    let open = is_open.get();
                    let base = s
                        .position(floem::style::Position::Absolute)
                        .inset_top_pct(100.0).inset_left(0.0).inset_right(0.0)
                        .margin_top(6.0).p_1()
                        .background(t.popover).color(t.popover_foreground)
                        .border_1().border_color(t.border).rounded_md().shadow_md().z_index(100)
                        .flex_direction(floem::style::FlexDirection::Column);
                    if open { base } else { base.display(floem::style::Display::None) }
                })
            }),
        )
    }
}

pub struct SelectItem {
    id: ViewId,
    value: String,
    label: String,
    disabled: bool,
    selected: Option<RwSignal<Option<String>>>,
    is_open: Option<RwSignal<bool>>,
}
impl SelectItem {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self { id: ViewId::new(), value: value.into(), label: label.into(), disabled: false, selected: None, is_open: None }
    }
    pub fn disabled(mut self, disabled: bool) -> Self { self.disabled = disabled; self }
    pub fn bind(mut self, selected: RwSignal<Option<String>>) -> Self { self.selected = Some(selected); self }
    pub fn auto_close(mut self, is_open: RwSignal<bool>) -> Self { self.is_open = Some(is_open); self }
}
impl HasViewId for SelectItem { fn view_id(&self) -> ViewId { self.id } }
impl IntoView for SelectItem {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate { self.into_view() }

    fn into_view(self) -> Self::V {
        let value = self.value;
        let label = self.label;
        let disabled = self.disabled;
        let selected = self.selected;
        let is_open = self.is_open;
        let v_for_style = value.clone();
        let v_for_click = value.clone();
        Box::new(
            floem::views::Container::new(
                floem::views::Stack::horizontal((
                    floem::views::Label::new(label).style(|s| s.text_sm().flex_grow(1.0)),
                    floem::views::Label::new("✓").style(move |s| {
                        let v = value.clone();
                        s.with_shadcn_theme(move |s, t| {
                            let is_selected = selected.map(|sig| sig.get() == Some(v.clone())).unwrap_or(false);
                            s.size_4().text_sm().color(t.foreground).items_center().justify_center().flex_shrink(0.0)
                                .apply_if(!is_selected, |s| s.display(floem::style::Display::None))
                        })
                    }),
                )).style(|s| s.width_full().items_center().gap_2()),
            )
            .style(move |s| {
                let v = v_for_style.clone();
                s.with_shadcn_theme(move |s, t| {
                    let is_selected = selected.map(|sig| sig.get() == Some(v.clone())).unwrap_or(false);
                    let base = s
                        .width_full().padding_top(6.0).padding_bottom(6.0).padding_left(8.0).padding_right(8.0)
                        .gap_2().items_center().rounded_sm()
                        .cursor(if disabled { CursorStyle::Default } else { CursorStyle::Pointer });
                    if is_selected { base.background(t.accent).color(t.accent_foreground) }
                    else if disabled { base.color(t.muted_foreground).opacity_50() }
                    else { base.color(t.foreground).hover(|s| s.background(t.accent).color(t.accent_foreground)) }
                })
            })
            .on_click_stop(move |_| {
                if !disabled {
                    if let Some(s) = selected { s.set(Some(v_for_click.clone())); }
                    if let Some(io) = is_open { io.set(false); }
                }
            }),
        )
    }
}

pub struct SelectLabel { id: ViewId, text: String }
impl SelectLabel { pub fn new(text: impl Into<String>) -> Self { Self { id: ViewId::new(), text: text.into() } } }
impl HasViewId for SelectLabel { fn view_id(&self) -> ViewId { self.id } }
impl IntoView for SelectLabel {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate { self.into_view() }

    fn into_view(self) -> Self::V {
        Box::new(floem::views::Label::with_id(self.id, self.text).style(|s| {
            s.with_shadcn_theme(move |s, t| {
                s.px_2().padding_top(6.0).padding_bottom(6.0).text_xs().color(t.muted_foreground)
            })
        }))
    }
}

pub struct SelectSeparator;
impl SelectSeparator { pub fn new() -> Self { Self } }
impl Default for SelectSeparator { fn default() -> Self { Self::new() } }
impl HasViewId for SelectSeparator { fn view_id(&self) -> ViewId { ViewId::new() } }
impl IntoView for SelectSeparator {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate { self.into_view() }

    fn into_view(self) -> Self::V {
        Box::new(floem::views::Empty::new().style(|s| {
            s.with_shadcn_theme(move |s, t| {
                s.width_full().height(1.0).background(t.border).margin_left(-4.0).margin_right(-4.0).margin_top(4.0).margin_bottom(4.0)
            })
        }))
    }
}

pub struct SelectGroup<V> { id: ViewId, label: String, child: V }
impl<V: IntoView + 'static> SelectGroup<V> {
    pub fn new(label: impl Into<String>, child: V) -> Self { Self { id: ViewId::new(), label: label.into(), child } }
}
impl<V: IntoView + 'static> HasViewId for SelectGroup<V> { fn view_id(&self) -> ViewId { self.id } }
impl<V: IntoView + 'static> IntoView for SelectGroup<V> {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate { self.into_view() }

    fn into_view(self) -> Self::V {
        Box::new(floem::views::Stack::vertical((SelectLabel::new(self.label), self.child)))
    }
}
