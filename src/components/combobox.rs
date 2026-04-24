use floem::prelude::*;
use floem::reactive::{Context, RwSignal, Scope, SignalGet, SignalUpdate};
use floem::style::CursorStyle;
use floem::view::ParentView;
use floem::views::Decorators;
use floem::{HasViewId, ViewId};
use floem_tailwind::TailwindExt;
use crate::theme::ShadcnThemeExt;

#[derive(Clone, Copy)] pub struct ComboboxContext { pub selected: RwSignal<Option<String>>, pub search: RwSignal<String>, pub is_open: RwSignal<bool> }
pub struct Combobox { id: ViewId, selected: RwSignal<Option<String>>, search: RwSignal<String>, is_open: RwSignal<bool>, scope: Scope }
impl Combobox {
    pub fn new(selected: RwSignal<Option<String>>, search: RwSignal<String>) -> Self { let is_open = RwSignal::new(false); let scope = Scope::current().create_child(); scope.provide_context(ComboboxContext { selected, search, is_open }); Self { id: ViewId::new(), selected, search, is_open, scope } }
    pub fn is_open_signal(&self) -> RwSignal<bool> { self.is_open } pub fn selected_signal(&self) -> RwSignal<Option<String>> { self.selected } pub fn search_signal(&self) -> RwSignal<String> { self.search }
}
impl HasViewId for Combobox { fn view_id(&self) -> ViewId { self.id } }
impl IntoView for Combobox { type V = Container;
    type Intermediate = Container;
    fn into_intermediate(self) -> Self::Intermediate { self.into_view() }
    fn into_view(self) -> Self::V { let scope = self.scope; let id = self.id; scope.enter(move || Container::with_id(id, ())) } }
impl ParentView for Combobox { fn scope(&self) -> Option<Scope> { Some(self.scope) } }

pub struct ComboboxTrigger { id: ViewId, placeholder: String, items: Vec<(String, String)> }
impl ComboboxTrigger {
    pub fn new(p: impl Into<String>) -> Self { Self { id: ViewId::new(), placeholder: p.into(), items: vec![] } }
    pub fn items(mut self, items: impl IntoIterator<Item=(impl Into<String>, impl Into<String>)>) -> Self { self.items = items.into_iter().map(|(v,l)|(v.into(),l.into())).collect(); self }
}
impl HasViewId for ComboboxTrigger { fn view_id(&self) -> ViewId { self.id } }
impl IntoView for ComboboxTrigger {
    type V = Box<dyn View>;

    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate { self.into_view() }
    fn into_view(self) -> Self::V {
        let ctx = Context::get::<ComboboxContext>();
        if let Some(ctx) = ctx {
            let selected = ctx.selected; let is_open = ctx.is_open; let items = self.items.clone();
            Box::new(floem::views::Stack::horizontal((
                floem::views::Label::derived(move || if let Some(v)=selected.get(){items.iter().find(|(x,_)|x==&v).map(|(_,l)|l.clone()).unwrap_or(v)}else{self.placeholder.clone()}).style(move |s| s.with_shadcn_theme(move |s,t|{let hv=selected.get().is_some();s.flex_grow(1.0).text_sm().color(if hv{t.foreground}else{t.muted_foreground})})),
                floem::views::Label::new("▼").style(|s| s.with_shadcn_theme(move |s,t| s.font_size(10.0).color(t.muted_foreground).flex_shrink(0.0))),
            )).style(|s| s.with_shadcn_theme(move |s,t| s.min_width(200.0).h_9().px_3().py_2().gap_2().items_center().border_1().border_color(t.input).rounded_md().background(t.background).shadow_sm().cursor(CursorStyle::Pointer).hover(|s|s.border_color(t.ring)))).on_click_stop(move |_|{is_open.update(|v|*v=!*v);}))
        } else { Box::new(floem::views::Label::new(self.placeholder).style(|s| s.with_shadcn_theme(move |s,t| s.min_width(200.0).h_9().px_3().py_2().items_center().border_1().border_color(t.input).rounded_md().background(t.background).color(t.muted_foreground)))) }
    }
}

pub struct ComboboxContent { id: ViewId }
impl ComboboxContent { pub fn new() -> Self { Self{id:ViewId::new()} } }
impl Default for ComboboxContent { fn default() -> Self { Self::new() } }
impl HasViewId for ComboboxContent { fn view_id(&self) -> ViewId { self.id } }
impl IntoView for ComboboxContent {
    type V = Box<dyn View>;

    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate { self.into_view() }
    fn into_view(self) -> Self::V {
        let ctx = Context::get::<ComboboxContext>();
        let stem = Container::with_id(self.id, ()).style(|s| s.flex_col().width_full());
        if let Some(ctx) = ctx { Box::new(floem::views::Container::new(stem).style(move |s| s.with_shadcn_theme(move |s,t|{s.position(floem::style::Position::Absolute).inset_top_pct(100.0).inset_left(0.0).inset_right(0.0).margin_top(6.0).min_width(200.0).flex_col().background(t.popover).color(t.popover_foreground).border_1().border_color(t.border).rounded_md().shadow_lg().z_index(100).apply_if(!ctx.is_open.get(),|s|s.display(floem::style::Display::None))}))) } else { Box::new(stem) }
    }
}
impl ParentView for ComboboxContent {}

pub struct ComboboxInput { id: ViewId, placeholder: String }
impl ComboboxInput { pub fn new() -> Self { Self{id:ViewId::new(),placeholder:"Search...".into()} } pub fn placeholder(mut self, p: impl Into<String>) -> Self { self.placeholder = p.into(); self } }
impl Default for ComboboxInput { fn default() -> Self { Self::new() } }
impl HasViewId for ComboboxInput { fn view_id(&self) -> ViewId { self.id } }
impl IntoView for ComboboxInput {
    type V = Box<dyn View>;

    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate { self.into_view() }
    fn into_view(self) -> Self::V { Box::new(floem::views::TextInput::new(RwSignal::new(String::new())).placeholder(self.placeholder).style(|s| s.with_shadcn_theme(move |s,t| s.width_full().h_8().px_3().text_sm().border(0.0).border_bottom(1.0).border_color(t.border).background(floem::peniko::Color::TRANSPARENT).color(t.foreground)))) }
}

pub struct ComboboxList { id: ViewId, max_height: f64 }
impl ComboboxList { pub fn new() -> Self { Self{id:ViewId::new(),max_height:300.0} } pub fn max_height(mut self, h: f64) -> Self { self.max_height=h; self } }
impl Default for ComboboxList { fn default() -> Self { Self::new() } }
impl HasViewId for ComboboxList { fn view_id(&self) -> ViewId { self.id } }
impl IntoView for ComboboxList { type V = floem::views::Scroll;
    type Intermediate = floem::views::Scroll;
    fn into_intermediate(self) -> Self::Intermediate { self.into_view() }
    fn into_view(self) -> Self::V { let mh=self.max_height; let c=Container::with_id(self.id,()).style(|s|s.flex_col().width_full().p_1()); floem::views::Scroll::new(c).style(move |s|s.max_height(mh).width_full()) } }
impl ParentView for ComboboxList {}

pub struct ComboboxItem { id: ViewId, value: String, label: String, disabled: bool }
impl ComboboxItem { pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self { Self{id:ViewId::new(),value:value.into(),label:label.into(),disabled:false} } pub fn disabled(mut self, d: bool) -> Self { self.disabled=d; self } }
impl HasViewId for ComboboxItem { fn view_id(&self) -> ViewId { self.id } }
impl IntoView for ComboboxItem {
    type V = Box<dyn View>;

    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate { self.into_view() }
    fn into_view(self) -> Self::V {
        let ctx = Context::get::<ComboboxContext>();
        if let Some(ctx) = ctx {
            let selected = ctx.selected; let is_open = ctx.is_open;
            let value = self.value; let label = self.label; let disabled = self.disabled;
            let v0 = value.clone(); let v1 = value.clone(); let v2 = value.clone();
            Box::new(
                floem::views::Container::new(floem::views::Stack::horizontal((
                    floem::views::Label::new(label).style(|s|s.text_sm().flex_grow(1.0)),
                    floem::views::Label::new("✓").style(move |s| { let v = v0.clone(); s.with_shadcn_theme(move |s,t| { let is_sel = selected.get() == Some(v.clone()); s.size_4().text_sm().color(t.foreground).items_center().justify_center().flex_shrink(0.0).apply_if(!is_sel,|s|s.display(floem::style::Display::None)) }) }),
                )).style(|s|s.width_full().items_center().gap_2()))
                .style(move |s| { let v = v1.clone(); s.with_shadcn_theme(move |s,t| { let is_sel = selected.get() == Some(v.clone()); let base = s.width_full().padding_top(6.0).padding_bottom(6.0).padding_left(8.0).padding_right(8.0).items_center().rounded_sm().cursor(if disabled{CursorStyle::Default}else{CursorStyle::Pointer}); if is_sel { base.background(t.accent).color(t.accent_foreground) } else if disabled { base.color(t.muted_foreground).opacity_50() } else { base.color(t.foreground).hover(|s|s.background(t.accent).color(t.accent_foreground)) } }) })
                .on_click_stop(move |_| { if !disabled { selected.set(Some(v2.clone())); is_open.set(false); } })
            )
        } else {
            Box::new(floem::views::Label::new(self.label).style(|s|s.with_shadcn_theme(move |s,t|s.width_full().padding(6.0).text_sm().color(t.foreground))))
        }
    }
}

pub struct ComboboxEmpty { id: ViewId, text: String }
impl ComboboxEmpty { pub fn new(t: impl Into<String>) -> Self { Self{id:ViewId::new(),text:t.into()} } }
impl Default for ComboboxEmpty { fn default() -> Self { Self::new("No results found.") } }
impl HasViewId for ComboboxEmpty { fn view_id(&self) -> ViewId { self.id } }
impl IntoView for ComboboxEmpty { type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate { self.into_view() }
    fn into_view(self) -> Self::V { Box::new(floem::views::Label::new(self.text).style(|s|s.with_shadcn_theme(move |s,t|s.width_full().padding_top(8.0).padding_bottom(8.0).text_sm().color(t.muted_foreground).justify_center()))) } }

pub struct ComboboxGroup { id: ViewId }
impl ComboboxGroup { pub fn new() -> Self { Self{id:ViewId::new()} } }
impl Default for ComboboxGroup { fn default() -> Self { Self::new() } }
impl HasViewId for ComboboxGroup { fn view_id(&self) -> ViewId { self.id } }
impl IntoView for ComboboxGroup { type V = Container;
    type Intermediate = Container;
    fn into_intermediate(self) -> Self::Intermediate { self.into_view() }
    fn into_view(self) -> Self::V { Container::with_id(self.id,()).style(|s|s.flex_col().width_full()) } }
impl ParentView for ComboboxGroup {}

pub struct ComboboxLabel { id: ViewId, text: String }
impl ComboboxLabel { pub fn new(t: impl Into<String>) -> Self { Self{id:ViewId::new(),text:t.into()} } }
impl HasViewId for ComboboxLabel { fn view_id(&self) -> ViewId { self.id } }
impl IntoView for ComboboxLabel { type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate { self.into_view() }
    fn into_view(self) -> Self::V { Box::new(floem::views::Label::new(self.text).style(|s|s.with_shadcn_theme(move |s,t|s.px_2().padding_top(6.0).padding_bottom(6.0).text_xs().font_medium().color(t.muted_foreground)))) } }

pub struct ComboboxSeparator;
impl ComboboxSeparator { pub fn new() -> Self { Self } }
impl Default for ComboboxSeparator { fn default() -> Self { Self::new() } }
impl HasViewId for ComboboxSeparator { fn view_id(&self) -> ViewId { ViewId::new() } }
impl IntoView for ComboboxSeparator { type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate { self.into_view() }
    fn into_view(self) -> Self::V { Box::new(floem::views::Empty::new().style(|s|s.with_shadcn_theme(move |s,t|s.width_full().height(1.0).background(t.border).margin_left(-4.0).margin_right(-4.0).margin_top(4.0).margin_bottom(4.0)))) } }
