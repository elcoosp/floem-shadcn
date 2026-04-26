use floem::reactive::RwSignal;
use lapce_xi_rope::Rope;

pub struct Document {
    buffer: RwSignal<Rope>,
    // many fields… we stub
}
impl Document {
    pub fn new(text: String) -> Self { Self { buffer: RwSignal::new(Rope::from(text)) } }
    pub fn cursor(&self) -> RwSignal<floem_editor_core::cursor::Cursor> { todo!() }
    pub fn text(&self) -> String { self.buffer.with_untracked(|b| b.to_string()) }
    pub fn buffer(&self) -> RwSignal<Rope> { self.buffer }
    pub fn text_layouts(&self) -> std::cell::RefCell<super::TextLayoutLines> { todo!() }
    pub fn on_update(&self, _: impl Fn(&str) + 'static) {}
}
