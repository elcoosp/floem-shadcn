use floem_test::TestRoot;
// Tests for Spinner component

use floem::prelude::*;
use floem_shadcn::components::spinner::*;
use floem_test::prelude::*;

#[test]
#[ignore]
fn test_spinner_default_md_size() {
    floem_shadcn::theme::set_theme(floem_shadcn::theme::ShadcnTheme::light());
    let spinner = Spinner::new();
    let id = spinner.view_id();

    let container = Stack::new((spinner,)).style(|s| s.size(100.0, 100.0));
    let mut harness = HeadlessHarness::new_with_size(TestRoot::new(), container, 100.0, 100.0);
    harness.rebuild();

    let layout = id.get_layout().expect("Spinner layout should exist");
    // Default Md size: 20x20
    assert!((layout.size.width - 20.0).abs() < 0.1, "Width should be 20, got {}", layout.size.width);
    assert!((layout.size.height - 20.0).abs() < 0.1, "Height should be 20, got {}", layout.size.height);
}

#[test]
#[ignore]
fn test_spinner_sm_size() {
    floem_shadcn::theme::set_theme(floem_shadcn::theme::ShadcnTheme::light());
    let spinner = Spinner::new().size(SpinnerSize::Sm);
    let id = spinner.view_id();

    let container = Stack::new((spinner,)).style(|s| s.size(100.0, 100.0));
    let mut harness = HeadlessHarness::new_with_size(TestRoot::new(), container, 100.0, 100.0);
    harness.rebuild();

    let layout = id.get_layout().expect("Layout should exist");
    assert!((layout.size.width - 16.0).abs() < 0.1, "Width should be 16, got {}", layout.size.width);
}

#[test]
#[ignore]
fn test_spinner_lg_size() {
    floem_shadcn::theme::set_theme(floem_shadcn::theme::ShadcnTheme::light());
    let spinner = Spinner::new().size(SpinnerSize::Lg);
    let id = spinner.view_id();

    let container = Stack::new((spinner,)).style(|s| s.size(100.0, 100.0));
    let mut harness = HeadlessHarness::new_with_size(TestRoot::new(), container, 100.0, 100.0);
    harness.rebuild();

    let layout = id.get_layout().expect("Layout should exist");
    assert!((layout.size.width - 24.0).abs() < 0.1, "Width should be 24, got {}", layout.size.width);
}
