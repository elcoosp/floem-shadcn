use floem_test::TestRoot;
// Tests for Kbd component

use floem::prelude::*;
use floem_shadcn::components::kbd::*;
use floem_test::prelude::*;

#[test]
fn test_kbd_renders() {
    floem_shadcn::theme::set_theme(floem_shadcn::theme::ShadcnTheme::light());
    let kbd = Kbd::new("⌘K");
    let id = kbd.view_id();

    let container = Stack::new((kbd,)).style(|s| s.size(100.0, 100.0));
    let mut harness = HeadlessHarness::new_with_size(TestRoot::new(), container, 100.0, 100.0);
    harness.rebuild();

    let layout = id.get_layout().expect("Kbd layout should exist");
    // Kbd should be small
    assert!(layout.size.width > 10.0, "Kbd should have some width");
    assert!(layout.size.height > 10.0, "Kbd should have some height");
}

#[test]
fn test_kbd_group() {
    floem_shadcn::theme::set_theme(floem_shadcn::theme::ShadcnTheme::light());
    let group = KbdGroup::new(vec!["⌘".into(), "K".into()]);
    let id = group.view_id();

    let container = Stack::new((group,)).style(|s| s.size(200.0, 100.0));
    let mut harness = HeadlessHarness::new_with_size(TestRoot::new(), container, 200.0, 100.0);
    harness.rebuild();

    let layout = id.get_layout().expect("KbdGroup layout should exist");
    assert!(
        layout.size.width > 20.0,
        "KbdGroup should have width for multiple keys"
    );
}
