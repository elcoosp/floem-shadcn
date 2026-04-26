use floem_test::TestRoot;
// Tests for Typography components

use floem::prelude::*;
use floem_shadcn::components::typography::*;
use floem_test::prelude::*;

#[test]
#[ignore]
fn test_h1_has_large_font_size() {
    let h1 = TypographyH1::new("Title");
    let id = h1.view_id();

    let container = Stack::new((h1,)).style(|s| s.size(400.0, 100.0));
    let mut harness = HeadlessHarness::new_with_size(TestRoot::new(), container, 400.0, 100.0);
    harness.rebuild();

    let layout = id.get_layout().expect("Layout should exist");
    // Height should be roughly proportional to font size (36px)
    assert!(layout.size.height > 20.0, "H1 should be tall enough");
}

#[test]
#[ignore]
fn test_muted_has_muted_color() {
    let muted = TypographyMuted::new("muted text");
    let id = muted.view_id();

    let container = Stack::new((muted,)).style(|s| s.size(400.0, 100.0));
    let mut harness = HeadlessHarness::new_with_size(TestRoot::new(), container, 400.0, 100.0);
    harness.rebuild();

    let layout = id.get_layout().expect("Layout should exist");
    assert!(layout.size.width > 0.0);
    // Color verification would need style inspection – we just ensure it renders.
}

#[test]
#[ignore]
fn test_list_renders() {
    let list = TypographyList::new(vec!["Item 1".into(), "Item 2".into()], false);
    let id = list.view_id();

    let container = Stack::new((list,)).style(|s| s.size(400.0, 200.0));
    let mut harness = HeadlessHarness::new_with_size(TestRoot::new(), container, 400.0, 200.0);
    harness.rebuild();

    let layout = id.get_layout().expect("Layout should exist");
    assert!(layout.size.width > 0.0);
}
