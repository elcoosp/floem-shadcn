use floem::views::Decorators;
use floem_test::TestRoot;
// Tests for Button Group component

use floem::prelude::*;
use floem::view::ParentView;
use floem_shadcn::components::button::Button;
use floem_shadcn::components::button_group::{ButtonGroup, ButtonGroupSeparator};
use floem_test::prelude::*;

#[test]
fn test_button_group_renders_three_buttons() {
    floem_shadcn::theme::set_theme(floem_shadcn::theme::ShadcnTheme::light());
    floem_shadcn::theme::set_theme(floem_shadcn::theme::ShadcnTheme::light());
    let btn1 = Button::new("Left");
    let btn2 = Button::new("Center");
    let btn3 = Button::new("Right");
    let group = ButtonGroup::new().child(btn1).child(btn2).child(btn3);
    let id = group.view_id();

    let container = Stack::new((group,)).style(|s| s.size(500.0, 100.0));
    let mut harness = HeadlessHarness::new_with_size(TestRoot::new(), container, 500.0, 100.0);
    harness.rebuild();

    let layout = id.get_layout().expect("ButtonGroup layout should exist");
    assert!(layout.size.width > 100.0, "Group should contain buttons");
    assert!(layout.size.height > 30.0, "Group should have button height");
}

#[test]
fn test_button_group_with_separator() {
    floem_shadcn::theme::set_theme(floem_shadcn::theme::ShadcnTheme::light());
    floem_shadcn::theme::set_theme(floem_shadcn::theme::ShadcnTheme::light());
    let btn1 = Button::new("Left");
    let btn2 = Button::new("Right");
    let group = ButtonGroup::new()
        .child(btn1)
        .child(ButtonGroupSeparator::new())
        .child(btn2);
    let id = group.view_id();

    let container = Stack::new((group,)).style(|s| s.size(400.0, 100.0));
    let mut harness = HeadlessHarness::new_with_size(TestRoot::new(), container, 400.0, 100.0);
    harness.rebuild();

    let layout = id.get_layout().expect("Layout should exist");
    assert!(layout.size.width > 100.0);
}
