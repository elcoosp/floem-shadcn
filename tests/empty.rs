use floem::views::Decorators;
use floem::reactive::SignalGet;
use floem::reactive::SignalUpdate;
use floem_test::TestRoot;
//! Tests for Empty component

use floem::prelude::*;
use floem::view::ParentView;
use floem_shadcn::components::empty::*;
use floem_test::prelude::*;

#[test]
fn test_empty_renders_with_title() {
    let empty = Empty::new()
        .child(EmptyContent::new()
            .child(EmptyTitle::new("Nothing here"))
            .child(EmptyDescription::new("Add some content")));
    let id = empty.view_id();

    let container = Stack::new((empty,)).style(|s| s.size(400.0, 300.0));
    let mut harness = HeadlessHarness::new_with_size(TestRoot::new(), container,  400.0,  300.0);
    harness.rebuild();

    let layout = id.get_layout().expect("Empty layout should exist");
    assert!(layout.size.width > 100.0, "Empty should have width");
}

#[test]
fn test_empty_with_actions() {
    let empty = Empty::new()
        .child(EmptyContent::new()
            .child(EmptyTitle::new("Empty"))
            .child(EmptyActions::new()
                .child(floem::views::Label::new("Action"))));
    let id = empty.view_id();

    let container = Stack::new((empty,)).style(|s| s.size(400.0, 300.0));
    let mut harness = HeadlessHarness::new_with_size(TestRoot::new(), container,  400.0,  300.0);
    harness.rebuild();

    let layout = id.get_layout().expect("Layout should exist");
    assert!(layout.size.width > 100.0);
}
