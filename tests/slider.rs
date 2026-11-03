//! Tests for the Slider component
use floem::reactive::RwSignal;
use floem_shadcn::components::slider::Slider;
use floem_test::prelude::*;

#[test]
fn test_slider_creation() {
    let value = RwSignal::new(50.0);
    let slider = Slider::new(value).build();
    let root = TestRoot::new();
    let mut harness = HeadlessHarness::new_with_size(root, slider, 200.0, 40.0);
    harness.rebuild();
    // just verify it builds
}

#[test]
fn test_slider_min_max() {
    let value = RwSignal::new(0.0);
    let slider = Slider::new(value).min(10.0).max(50.0).build();
    let root = TestRoot::new();
    let _harness = HeadlessHarness::new_with_size(root, slider, 200.0, 40.0);
}

#[test]
fn test_slider_disabled() {
    let value = RwSignal::new(50.0);
    let slider = Slider::new(value).disabled(true).build();
    let root = TestRoot::new();
    let _harness = HeadlessHarness::new_with_size(root, slider, 200.0, 40.0);
}
