//! Integration tests for the floem-shadcn component library.
use floem_test::TestRoot;
// Tests for Field component

use floem::prelude::*;
use floem::view::ParentView;
use floem_shadcn::components::field::*;
use floem_test::prelude::*;

#[test]
fn test_field_orientation_default() {
    assert_eq!(FieldOrientation::default(), FieldOrientation::Vertical);
    assert_ne!(FieldOrientation::Horizontal, FieldOrientation::Vertical);
    assert_ne!(FieldOrientation::Responsive, FieldOrientation::Horizontal);
}

#[test]
fn test_field_error_from_errors() {
    assert!(
        FieldError::from_errors(Vec::new()).is_none(),
        "no errors should render nothing"
    );
    let single = FieldError::from_errors(vec!["Required".to_string()]);
    assert!(single.is_some(), "one error should render");
    let many = FieldError::from_errors(vec!["A".to_string(), "B".to_string()]);
    assert!(many.is_some(), "multiple errors should render");
}

#[test]
fn test_field_pieces_construct() {
    // Every Field sub-part should construct without panicking.
    let _label = FieldLabel::new("Name");
    let _title = FieldTitle::new("Name");
    let _description = FieldDescription::new("Your full name");
    let _error = FieldError::new("Name is required.");
    let _legend = FieldLegend::new("Profile").variant(FieldLegendVariant::Label);
    let _legend_label = FieldLegend::label("Profile");
    let _separator = FieldSeparator::new();
    let _separator_or = FieldSeparator::with_content("OR");
    let _group = FieldGroup::new();
    let _set = FieldSet::new();
    let _content = FieldContent::new();
    let _action = FieldAction::new();
    let _field = Field::new()
        .orientation(FieldOrientation::Horizontal)
        .invalid(true)
        .optimistic(true);
}

#[test]
fn test_field_renders_with_children() {
    let field = Field::new().child(FieldLabel::new("Email")).child(
        FieldContent::new()
            .child(FieldDescription::new("We never share your email."))
            .child(FieldError::new("Email is invalid.")),
    );
    let id = field.view_id();

    let container = Stack::new((field,)).style(|s| s.size(300.0, 120.0));
    let mut harness = HeadlessHarness::new_with_size(TestRoot::new(), container, 300.0, 120.0);
    harness.rebuild();

    let layout = id.get_layout().expect("Field layout should exist");
    assert!(layout.size.width > 100.0, "Field should stretch wide");
    assert!(
        layout.size.height > 20.0,
        "Field should fit label + content"
    );
}

#[test]
fn test_field_separator_renders_content() {
    let separator = FieldSeparator::with_content("OR");
    let id = separator.view_id();

    let container = Stack::new((separator,)).style(|s| s.size(300.0, 40.0));
    let mut harness = HeadlessHarness::new_with_size(TestRoot::new(), container, 300.0, 40.0);
    harness.rebuild();

    let layout = id.get_layout().expect("FieldSeparator layout should exist");
    assert!(layout.size.width > 100.0, "Separator should span its row");
}
