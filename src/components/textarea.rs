use floem::prelude::*;
use floem::views::{TextArea, Decorators};
use floem::{HasViewId, ViewId};
use floem_tailwind::TailwindExt;
use crate::theme::ShadcnThemeExt;

pub struct Textarea {
    id: ViewId,
    rows: u32,
    placeholder: Option<String>,
    on_change: Option<Box<dyn Fn(&str)>>,
}

impl Textarea {
    pub fn new(initial: impl Into<String>) -> Self { Self { id: ViewId::new(), rows: 3, placeholder: Some(initial.into()), on_change: None } }
    pub fn rows(mut self, rows: u32) -> Self { self.rows = rows; self }
    pub fn placeholder(mut self, text: impl Into<String>) -> Self { self.placeholder = Some(text.into()); self }
    pub fn on_change(mut self, callback: impl Fn(&str) + 'static) -> Self { self.on_change = Some(Box::new(callback)); self }
    pub fn resizable(self, _: bool) -> Self { self }
    pub fn on_update(self, callback: impl Fn(&str) + 'static) -> Self { self.on_change(callback) }
}

impl HasViewId for Textarea { fn view_id(&self) -> ViewId { self.id } }

impl IntoView for Textarea {
    type V = Box<dyn View>;
    fn into_view(self) -> Self::V {
        let min_height = (self.rows as f64) * 24.0 + 16.0;
        let mut area = TextArea::new()
            .style(move |s| {
                s.min_height(min_height).w_full().rounded_md().border_1().px_3().py_2().text_sm()
                    .with_shadcn_theme(|s, t| {
                        let ring = t.ring;
                        s.border_color(t.input).background(t.background).color(t.foreground)
                         .focus(move |s| s.outline(2.0).outline_color(ring))
                    })
            });
        if let Some(callback) = self.on_change {
            area = area.on_update(move |text| callback(text));
        }
        Box::new(area)
    }
}
