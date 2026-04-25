use floem::prelude::*;
use floem::reactive::RwSignal;
use floem::views::{TextInput, Decorators};
use floem::{HasViewId, ViewId};
use crate::theme::ShadcnThemeExt;
pub struct Input { id: ViewId, placeholder: Option<String> }
impl Input { pub fn new() -> Self { Self{id:ViewId::new(),placeholder:None} } pub fn with_text(t: impl Into<String>) -> Self { Self{placeholder:Some(t.into()),..Self::new()} } pub fn placeholder(mut self, t: impl Into<String>) -> Self { self.placeholder=Some(t.into()); self } pub fn on_update(self, _: impl Fn(&str)+'static) -> Self { self } pub fn value(self, _: impl Fn() -> String + 'static) -> Self { self } pub fn on_enter(self, _: impl Fn(&str)+'static) -> Self { self } }
impl Default for Input { fn default() -> Self { Self::new() } }
impl HasViewId for Input { fn view_id(&self) -> ViewId { self.id } }
impl IntoView for Input { type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate { self.into_view() }
    fn into_view(self) -> Self::V { let buffer = RwSignal::new(String::new()); Box::new(TextInput::new(buffer).placeholder(self.placeholder.unwrap_or_default()).style(move |s| s.height(40.0).width_full().border_radius(6.0).border(1.0).padding_left(12.0).padding_right(12.0).padding_top(8.0).padding_bottom(8.0).font_size(14.0).with_shadcn_theme(|s,t|{ let ring=t.ring; s.border_color(t.input).background(t.background).color(t.foreground).focus(move |s| s.outline(2.0).outline_color(ring)) }))) } }
