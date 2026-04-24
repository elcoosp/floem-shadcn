use floem::prelude::*;
use floem::reactive::RwSignal;
use floem::{HasViewId, ViewId};
pub struct Slider { id: ViewId, #[allow(dead_code)] value: RwSignal<f64> }
impl Slider { pub fn new(v: RwSignal<f64>) -> Self { Self{id:ViewId::new(),value:v} } pub fn min(self, _: f64) -> Self { self } pub fn max(self, _: f64) -> Self { self } pub fn step(self, _: f64) -> Self { self } pub fn disabled(self, _: bool) -> Self { self } pub fn build(self) -> impl IntoView { floem::views::Empty::new() } }
impl HasViewId for Slider { fn view_id(&self) -> ViewId { self.id } }
impl IntoView for Slider { type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate { self.into_view() }
 fn into_view(self) -> Self::V { Box::new(self.build().into_view()) } }
