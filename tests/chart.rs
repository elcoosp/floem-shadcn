//! Integration tests for the floem-shadcn component library.
use floem_test::TestRoot;
// Tests for Chart component

use floem::prelude::*;
use floem_shadcn::components::chart::*;
use floem_test::prelude::*;

fn sample_data() -> ChartData {
    ChartData::new()
        .title("Monthly revenue")
        .description("Desktop vs mobile")
        .series(ChartSeries::new(
            "Desktop",
            vec![
                ChartPoint::new("Jan", 186.0),
                ChartPoint::new("Feb", 305.0),
                ChartPoint::new("Mar", 237.0),
            ],
        ))
        .series(ChartSeries::new(
            "Mobile",
            vec![ChartPoint::new("Jan", 80.0), ChartPoint::new("Feb", 200.0)],
        ))
}

#[test]
fn test_chart_data_construction() {
    let data = sample_data();
    assert_eq!(data.series.len(), 2);
    assert_eq!(data.series[0].name, "Desktop");
    assert_eq!(data.category_count(), 3, "longest series wins");
    assert!(
        (data.max_value() - 305.0).abs() < 1e-9,
        "max value should be 305"
    );
    assert_eq!(data.title.as_deref(), Some("Monthly revenue"));
    assert_eq!(data.description.as_deref(), Some("Desktop vs mobile"));
}

#[test]
fn test_chart_data_empty() {
    let data = ChartData::new();
    assert_eq!(data.category_count(), 0);
    assert_eq!(data.max_value(), 0.0, "no data means no maximum");
}

#[test]
fn test_bar_height_normalization() {
    // Half of the max -> half of the plot height
    assert!((bar_height(150.0, 300.0, 240.0) - 120.0).abs() < 1e-4);
    // Full value -> full plot height
    assert!((bar_height(300.0, 300.0, 240.0) - 240.0).abs() < 1e-4);
    // Zero value -> zero height
    assert!((bar_height(0.0, 300.0, 240.0)).abs() < 1e-6);
    // Negative values clamp to zero height
    assert!((bar_height(-5.0, 300.0, 240.0)).abs() < 1e-6);
    // Values above the max clamp to the plot height
    assert!((bar_height(900.0, 300.0, 240.0) - 240.0).abs() < 1e-4);
}

#[test]
fn test_bar_height_guards_against_bad_input() {
    // Non-positive maximum falls back to 1.0 instead of dividing by zero
    assert!((bar_height(0.5, 0.0, 100.0) - 50.0).abs() < 1e-4);
    assert!((bar_height(1.0, -3.0, 100.0) - 100.0).abs() < 1e-4);
    // NaN / infinite values are treated as zero
    assert!((bar_height(f64::NAN, 300.0, 240.0)).abs() < 1e-6);
    assert!((bar_height(f64::INFINITY, 300.0, 240.0)).abs() < 1e-6);
    assert!((bar_height(f64::NEG_INFINITY, 300.0, 240.0)).abs() < 1e-6);
}

#[test]
fn test_bar_width_divides_slots() {
    assert!((bar_width(400.0, 4, 2) - 50.0).abs() < 1e-4);
    // Never divides by zero
    assert!((bar_width(400.0, 0, 0) - 400.0).abs() < 1e-4);
}

#[test]
fn test_series_color_assignment() {
    let palette = chart_colors();
    assert_eq!(palette.len(), 6, "shadcn chart palette has 6 colors");
    // Index maps into the palette
    assert_eq!(series_color(0), palette[0]);
    assert_eq!(series_color(3), palette[3]);
    // Indexes wrap around
    assert_eq!(series_color(6), palette[0]);
    assert_eq!(series_color(7), palette[1]);
    // Explicit color wins over the palette
    let explicit = series_color(2);
    let series = ChartSeries::new("S", Vec::new()).with_color(explicit);
    assert_eq!(series.resolved_color(0), explicit);
    // Default uses the palette by index
    let series = ChartSeries::new("S", Vec::new());
    assert_eq!(series.resolved_color(4), palette[4]);
}

#[test]
fn test_chart_renders() {
    let chart = Chart::new(sample_data()).height(180.0).legend(true);
    let id = chart.view_id();

    let container = Stack::new((chart,)).style(|s| s.size(400.0, 400.0));
    let mut harness = HeadlessHarness::new_with_size(TestRoot::new(), container, 400.0, 400.0);
    harness.rebuild();

    let layout = id.get_layout().expect("Chart layout should exist");
    assert!(layout.size.width > 100.0, "Chart should have width");
    assert!(
        layout.size.height > 100.0,
        "Chart should fit plot + legend + labels"
    );
}

#[test]
fn test_chart_without_legend_and_grid() {
    let chart = Chart::new(sample_data()).legend(false).grid(false);
    let id = chart.view_id();

    let container = Stack::new((chart,)).style(|s| s.size(300.0, 300.0));
    let mut harness = HeadlessHarness::new_with_size(TestRoot::new(), container, 300.0, 300.0);
    harness.rebuild();

    let layout = id.get_layout().expect("Chart layout should exist");
    assert!(layout.size.width > 50.0);
}

#[test]
fn test_chart_legend_and_tooltip_construct() {
    let data = sample_data();
    let legend = ChartLegend::for_data(&data);
    let legend_id = legend.view_id();
    // The shadcn export-name aliases are the same types
    let legend: ChartLegendContent = legend;

    let tooltip = ChartTooltip::new("Desktop", 305.0).label("Feb");
    let tooltip_id = tooltip.view_id();
    let tooltip: ChartTooltipContent = tooltip;

    // Both pieces render as real views with measurable layout
    let container = Stack::vertical((legend, tooltip)).style(|s| s.size(300.0, 200.0));
    let mut harness = HeadlessHarness::new_with_size(TestRoot::new(), container, 300.0, 200.0);
    harness.rebuild();

    let legend_layout = legend_id
        .get_layout()
        .expect("ChartLegend layout should exist");
    assert!(
        legend_layout.size.width > 10.0,
        "legend should size to content"
    );
    let tooltip_layout = tooltip_id
        .get_layout()
        .expect("ChartTooltip layout should exist");
    assert!(
        tooltip_layout.size.width > 10.0,
        "tooltip should size to content"
    );
}
