use floem::prelude::*;
use floem::views::Decorators;
use floem::{HasViewId, ViewId};
use floem_tailwind::TailwindExt;

use crate::text::TextInput;
use crate::theme::ShadcnThemeExt;

pub struct Input {
    id: ViewId,
    initial_text: String,
    placeholder_text: Option<String>,
    on_enter: Option<Box<dyn Fn(&str)>>,
    on_update: Option<Box<dyn Fn(&str)>>,
    value_fn: Option<Box<dyn Fn() -> String>>,
}

impl Input {
    pub fn new() -> Self { Self { id: ViewId::new(), initial_text: String::new(), placeholder_text: None, on_enter: None, on_update: None, value_fn: None } }
    pub fn with_text(text: impl Into<String>) -> Self { Self { id: ViewId::new(), initial_text: text.into(), placeholder_text: None, on_enter: None, on_update: None, value_fn: None } }
    pub fn placeholder(mut self, text: impl Into<String>) -> Self { self.placeholder_text = Some(text.into()); self }
    pub fn on_enter(mut self, callback: impl Fn(&str) + 'static) -> Self { self.on_enter = Some(Box::new(callback)); self }
    pub fn on_update(mut self, callback: impl Fn(&str) + 'static) -> Self { self.on_update = Some(Box::new(callback)); self }
    pub fn value(mut self, getter: impl Fn() -> String + 'static) -> Self { self.value_fn = Some(Box::new(getter)); self }

    pub fn build(self) -> impl IntoView {
        let mut input = TextInput::with_text_and_id(self.initial_text, self.id);
        if let Some(placeholder) = self.placeholder_text { input = input.placeholder(placeholder); }
        if let Some(callback) = self.on_update { input = input.on_update(callback); }
        if let Some(getter) = self.value_fn { input = input.value(getter); }
        if let Some(callback) = self.on_enter { input = input.on_enter(callback); }
        input.style(move |s| {
            s.h_10().w_full().rounded_md().border(1.0).px_3().py_2().font_size(14.0)
                .with_shadcn_theme(|s, t| {
                    let ring = t.ring;
                    s.border_color(t.input).background(t.background).color(t.foreground)
                        .focus(move |s| s.outline(2.0).outline_color(ring))
                })
        })
    }
}

impl Default for Input { fn default() -> Self { Self::new() } }
impl HasViewId for Input { fn view_id(&self) -> ViewId { self.id } }
impl IntoView for Input { type V = Box<dyn View>; type Intermediate = Self; fn into_intermediate(self) -> Self::Intermediate { self } fn into_view(self) -> Self::V { Box::new(self.build().into_view()) } }
