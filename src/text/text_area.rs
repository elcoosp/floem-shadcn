use floem::{View, ViewId};
use floem::style::Style;

pub struct TextArea { id: ViewId }
impl TextArea {
    pub fn with_text_and_id(_text: impl Into<String>, id: ViewId) -> Self { Self { id } }
    pub fn resizable(self, _enabled: bool) -> Self { self }
    pub fn on_update(self, _cb: impl Fn(&str) + 'static) -> Self { self }
}
impl View for TextArea {
    fn id(&self) -> ViewId { self.id }
    fn view_style(&self) -> Option<Style> { None }
}
