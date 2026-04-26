// Tests for sidebar menu button style cache (regression test)
use floem::prelude::*;
use floem::reactive::RwSignal;
use floem_shadcn::components::sidebar::*;
use floem_test::prelude::*;

#[test]
fn test_sidebar_button_active_state() {
    let active = RwSignal::new("none");
    let btn = SidebarMenuButton::new("First").is_active(move || active.get() == "first");
    let id = btn.view_id();
    let view = Stack::new((btn,)).style(|s| s.size(300.0, 100.0));
    let mut harness = HeadlessHarness::new_with_size(TestRoot::new(), view, 300.0, 100.0);
    harness.rebuild();
    // just ensure it builds and layout exists
    assert!(id.get_layout().is_some());
}
