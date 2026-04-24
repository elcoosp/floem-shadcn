use floem::prelude::*;
use floem::reactive::RwSignal;
use floem::views::Decorators;
use floem::{HasViewId, ViewId};
use floem_tailwind::TailwindExt;
use crate::theme::ShadcnThemeExt;

pub struct Textarea {
    id: ViewId,
    placeholder: Option<String>,
}

impl Textarea {
    pub fn new(initial: impl Into<String>) -> Self {
        Self { id: ViewId::new(), placeholder: Some(initial.into()) }
    }
    pub fn rows(self, _: u32) -> Self { self }
    pub fn placeholder(mut self, text: impl Into<String>) -> Self { self.placeholder = Some(text.into()); self }
    pub fn on_change(self, _: impl Fn(&str) + 'static) -> Self { self }
    pub fn resizable(self, _: bool) -> Self { self }
    pub fn on_update(self, _: impl Fn(&str) + 'static) -> Self { self }
}

impl HasViewId for Textarea { fn view_id(&self) -> ViewId { self.id } }

impl IntoView for Textarea {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate { self.into_view() }

    fn into_view(self) -> Self::V {
        let buffer = RwSignal::new(String::new());
        let area = floem::views::TextInput::new(buffer)
            .style(|s| {
                s.w_full().rounded_md().border_1().px_3().py_2().text_sm()
                    .with_shadcn_theme(|s, t| {
                        let ring = t.ring;
                        s.border_color(t.input).background(t.background).color(t.foreground)
                         .focus(move |s| s.outline(2.0).outline_color(ring))
                    })
            });
        Box::new(area)
    }
}
