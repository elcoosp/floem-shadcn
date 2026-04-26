//! Tests for the Slider component

use floem::prelude::*;
use floem::reactive::RwSignal;
use floem::view::ViewId;
use floem::views::Decorators;
use floem_shadcn::components::slider::Slider;
use floem_test::TestRoot;
use floem_test::prelude::*;

/// Create a simple test slider for coordinate calculation tests
fn create_test_slider(value: RwSignal<f64>, width: f64) -> (ViewId, impl IntoView) {
    let container_id = ViewId::new();
    let padding = 8.0;

    let view = floem::views::Container::with_id(
        container_id,
        floem::views::Empty::new().style(move |s| {
            let percent = value.get();
            s.height(6.0)
                .width_pct(percent)
                .background(peniko::Color::from_rgb8(14, 165, 233))
        }),
    )
    .style(move |s| {
        s.width(width)
            .height(16.0)
            .items_center()
            .background(peniko::Color::from_rgb8(229, 229, 229))
            .padding_left(padding)
            .padding_right(padding)
            .cursor(floem::style::CursorStyle::Pointer)
    })
    .on_event_stop(floem::event::listener::PointerDown, move |_cx, event| {
        let content_rect = container_id.get_content_rect();
        let track_width = content_rect.width();
        let click_x = event.state.logical_point().x - content_rect.x0;
        let percent = (click_x / track_width).clamp(0.0, 1.0);
        value.set(percent * 100.0);
    });

    (container_id, view)
}

#[test]
fn test_slider_creation() {
    let value = RwSignal::new(50.0);
    let slider = Slider::new(value).build();
    let root = TestRoot::new();
    let mut harness = HeadlessHarness::new_with_size(root, slider, 200.0, 40.0);
    harness.rebuild();
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

#[test]
fn test_slider_layout_with_padding() {
    let value = RwSignal::new(50.0);
    let (container_id, view) = create_test_slider(value, 300.0);
    let mut harness = HeadlessHarness::new_with_size(TestRoot::new(), view, 400.0, 100.0);
    harness.rebuild();

    let content_rect = harness.get_content_rect(container_id);

    // With 8px padding on each side, content should be 284px (300 - 16)
    assert!(
        (content_rect.width() - 284.0).abs() < 1.0,
        "Content width should be 284 (300 - 2*8 padding), got {}",
        content_rect.width()
    );
}

#[test]
fn test_slider_click_at_start() {
    let value = RwSignal::new(50.0);
    let (_container_id, view) = create_test_slider(value, 300.0);
    let mut harness = HeadlessHarness::new_with_size(TestRoot::new(), view, 400.0, 100.0);
    harness.rebuild();

    harness.pointer_down(8.0, 8.0);
    let new_value = value.get();
    assert!(
        new_value < 5.0,
        "Clicking at start should give ~0%, got {}%",
        new_value
    );
}

#[test]
fn test_slider_click_at_middle() {
    let value = RwSignal::new(0.0);
    let (_container_id, view) = create_test_slider(value, 300.0);
    let mut harness = HeadlessHarness::new_with_size(TestRoot::new(), view, 400.0, 100.0);
    harness.rebuild();

    harness.pointer_down(150.0, 8.0);
    let new_value = value.get();
    assert!(
        (new_value - 50.0).abs() < 10.0,
        "Clicking at middle should give ~50%, got {}%",
        new_value
    );
}

#[test]
fn test_slider_click_at_end() {
    let value = RwSignal::new(0.0);
    let (_container_id, view) = create_test_slider(value, 300.0);
    let mut harness = HeadlessHarness::new_with_size(TestRoot::new(), view, 400.0, 100.0);
    harness.rebuild();

    harness.pointer_down(292.0, 8.0);
    let new_value = value.get();
    assert!(
        new_value > 95.0,
        "Clicking at end should give ~100%, got {}%",
        new_value
    );
}

#[test]
fn test_slider_click_at_quarter() {
    let value = RwSignal::new(0.0);
    let (_container_id, view) = create_test_slider(value, 300.0);
    let mut harness = HeadlessHarness::new_with_size(TestRoot::new(), view, 400.0, 100.0);
    harness.rebuild();

    harness.pointer_down(79.0, 8.0);
    let new_value = value.get();
    assert!(
        (new_value - 25.0).abs() < 10.0,
        "Clicking at 25% should give ~25%, got {}%",
        new_value
    );
}

#[test]
fn test_slider_click_at_three_quarters() {
    let value = RwSignal::new(0.0);
    let (_container_id, view) = create_test_slider(value, 300.0);
    let mut harness = HeadlessHarness::new_with_size(TestRoot::new(), view, 400.0, 100.0);
    harness.rebuild();

    harness.pointer_down(221.0, 8.0);
    let new_value = value.get();
    assert!(
        (new_value - 75.0).abs() < 10.0,
        "Clicking at 75% should give ~75%, got {}%",
        new_value
    );
}

#[test]
fn test_actual_slider_component_value_persists() {
    let value = RwSignal::new(0.0);
    let view = Slider::new(value).build();
    let mut harness = HeadlessHarness::new_with_size(TestRoot::new(), view, 400.0, 100.0);
    harness.rebuild();

    harness.pointer_down(200.0, 8.0);
    let value_after_down = value.get();
    assert!(
        value_after_down > 0.0,
        "Value should be set after pointer down"
    );
}
