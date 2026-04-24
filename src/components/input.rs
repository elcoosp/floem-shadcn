use floem::prelude::*;
use floem::views::{TextInput, Decorators};
use floem::{HasViewId, ViewId};
use floem_tailwind::TailwindExt;
use crate::theme::ShadcnThemeExt;

pub struct Input {
    id: ViewId,
    placeholder: Option<String>,
    on_update: Option<Box<dyn Fn(&str)>>,
}

impl Input {
    pub fn new() -> Self { Self { id: ViewId::new(), placeholder: None, on_update: None } }
    pub fn with_text(text: impl Into<String>) -> Self { Self { placeholder: Some(text.into()), ..Self::new() } }
    pub fn placeholder(mut self, text: impl Into<String>) -> Self { self.placeholder = Some(text.into()); self }
    pub fn on_update(mut self, callback: impl Fn(&str) + 'static) -> Self { self.on_update = Some(Box::new(callback)); self }
    pub fn value(self, _: impl Fn() -> String + 'static) -> Self { self }
    pub fn on_enter(self, _: impl Fn(&str) + 'static) -> Self { self }
}

impl Default for Input { fn default() -> Self { Self::new() } }

impl HasViewId for Input { fn view_id(&self) -> ViewId { self.id } }

impl IntoView for Input {
    type V = Box<dyn View>;
    fn into_view(self) -> Self::V {
        let placeholder = self.placeholder.unwrap_or_default();
        let mut input = TextInput::new()
            .placeholder(placeholder)
            .style(move |s| {
                s.h_10().w_full().rounded_md().border(1.0).px_3().py_2().font_size(14.0)
                    .with_shadcn_theme(|s, t| {
                        let ring = t.ring;
                        s.border_color(t.input).background(t.background).color(t.foreground)
                         .focus(move |s| s.outline(2.0).outline_color(ring))
                    })
            });
        if let Some(callback) = self.on_update {
            input = input.on_update(move |text| callback(text));
        }
        Box::new(input)
    }
}
