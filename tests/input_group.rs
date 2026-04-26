use floem::views::Decorators;
use floem_test::TestRoot;
// Tests for Input Group component

use floem::prelude::*;
use floem::view::ParentView;
use floem_shadcn::components::input_group::{AddonPosition, InputGroup, InputGroupAddon};
use floem_test::prelude::*;

#[test]
#[ignore]
fn test_input_group_with_prefix_addon() {
    floem_shadcn::theme::set_theme(floem_shadcn::theme::ShadcnTheme::light());
    let addon = InputGroupAddon::new(AddonPosition::Prefix, floem::views::Label::new("$"));
    let group = InputGroup::new().child(addon);
    let id = group.view_id();

    let container = Stack::new((group,)).style(|s| s.size(300.0, 100.0));
    let mut harness = HeadlessHarness::new_with_size(TestRoot::new(), container, 300.0, 100.0);
    harness.rebuild();

    let layout = id.get_layout().expect("InputGroup layout should exist");
    assert!(layout.size.width > 20.0, "Group should contain addon");
}

#[test]
#[ignore]
fn test_input_group_with_disabled() {
    floem_shadcn::theme::set_theme(floem_shadcn::theme::ShadcnTheme::light());
    let addon = InputGroupAddon::new(AddonPosition::Suffix, floem::views::Label::new(".com"));
    let group = InputGroup::new().disabled(true).child(addon);
    let id = group.view_id();

    let container = Stack::new((group,)).style(|s| s.size(300.0, 100.0));
    let mut harness = HeadlessHarness::new_with_size(TestRoot::new(), container, 300.0, 100.0);
    harness.rebuild();

    let layout = id.get_layout().expect("Layout should exist");
    assert!(layout.size.width > 20.0);
}
