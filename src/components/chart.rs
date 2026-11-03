//! Chart component – lightweight bar charts
//!
//! Floem-native port of shadcn/ui Charts. shadcn charts are built on recharts
//! (React/DOM SVG); this port keeps the API shape (`ChartContainer`,
//! `ChartLegend`, series/points value types) but renders with regular Floem
//! flex views: the plot area is a container, grid lines and axes are thin
//! `Empty` views and each bar is a colored, rounded view whose height is
//! computed from the data. Hovering a bar highlights it via Floem's `hover`
//! state selector (the equivalent of a chart tooltip highlight).
//!
//! # Example
//!
//! ```
//! use floem_shadcn::components::chart::*;
//!
//! let data = ChartData::new()
//!     .title("Monthly revenue")
//!     .series(
//!         ChartSeries::new("Desktop", vec![
//!             ChartPoint::new("Jan", 186.0),
//!             ChartPoint::new("Feb", 305.0),
//!             ChartPoint::new("Mar", 237.0),
//!         ])
//!     );
//!
//! let chart = Chart::new(data).height(220.0).legend(true);
//! ```

use floem::peniko::Color;
use floem::peniko::color::{AlphaColor, Oklch, Srgb};
use floem::prelude::*;
use floem::views::Decorators;
use floem::{HasViewId, ViewId};
use floem_tailwind::TailwindExt;
use num_traits::ToPrimitive;

use crate::styled::ShadcnStyleExt;
use crate::theme::ShadcnThemeExt;

// ============================================================================
// Value types
// ============================================================================

/// A single data point in a [`ChartSeries`].
///
/// `label` is the category shown on the x axis, `value` is the numeric value
/// used to size the bar.
#[derive(Debug, Clone, PartialEq)]
pub struct ChartPoint {
    /// Category label rendered on the x axis
    pub label: String,
    /// Numeric value used to size the bar
    pub value: f64,
}

impl ChartPoint {
    /// Create a new data point
    pub fn new(label: impl Into<String>, value: f64) -> Self {
        Self {
            label: label.into(),
            value,
        }
    }
}

/// One named, colored series of [`ChartPoint`]s.
///
/// When no explicit color is set, a color is picked from the shadcn chart
/// palette based on the series index (see [`series_color`]).
#[derive(Debug, Clone, PartialEq)]
pub struct ChartSeries {
    /// Series name shown in the legend and tooltip
    pub name: String,
    /// Optional explicit color; when `None` the palette color is used
    pub color: Option<Color>,
    /// The data points of this series
    pub points: Vec<ChartPoint>,
}

impl ChartSeries {
    /// Create a new series with the default palette color
    pub fn new(name: impl Into<String>, points: Vec<ChartPoint>) -> Self {
        Self {
            name: name.into(),
            color: None,
            points,
        }
    }

    /// Assign an explicit color to this series
    #[must_use]
    pub fn with_color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// The effective color of this series: the explicit color if set,
    /// otherwise the palette color for `index`
    #[must_use]
    pub fn resolved_color(&self, index: usize) -> Color {
        self.color.unwrap_or_else(|| series_color(index))
    }
}

/// The full dataset rendered by a [`Chart`].
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ChartData {
    /// Optional chart title rendered above the plot
    pub title: Option<String>,
    /// Optional description rendered under the title
    pub description: Option<String>,
    /// The series to render
    pub series: Vec<ChartSeries>,
}

impl ChartData {
    /// Create an empty chart data set
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the chart title
    #[must_use]
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set the chart description
    #[must_use]
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Append a series
    #[must_use]
    pub fn series(mut self, series: ChartSeries) -> Self {
        self.series.push(series);
        self
    }

    /// Replace all series
    #[must_use]
    pub fn with_series(mut self, series: Vec<ChartSeries>) -> Self {
        self.series = series;
        self
    }

    /// The number of categories (x axis slots), i.e. the longest series length
    #[must_use]
    pub fn category_count(&self) -> usize {
        self.series
            .iter()
            .map(|s| s.points.len())
            .max()
            .unwrap_or(0)
    }

    /// The largest finite value across all series (ignoring negative and
    /// non-finite values); returns `0.0` when no positive values exist
    #[must_use]
    pub fn max_value(&self) -> f64 {
        self.series
            .iter()
            .flat_map(|s| s.points.iter())
            .map(|p| p.value)
            .filter(|v| v.is_finite() && *v > 0.0)
            .fold(0.0_f64, f64::max)
    }
}

// ============================================================================
// Palette
// ============================================================================

#[inline]
fn oklch(l: f32, c: f32, h: f32) -> Color {
    AlphaColor::<Oklch>::new([l, c, h, 1.0]).convert::<Srgb>()
}

/// The default shadcn chart palette (`--chart-1` … `--chart-5` plus a sixth
/// red used for overflow series)
#[must_use]
pub fn chart_colors() -> [Color; 6] {
    [
        oklch(0.646, 0.222, 41.116),
        oklch(0.6, 0.118, 184.704),
        oklch(0.398, 0.07, 227.392),
        oklch(0.828, 0.189, 84.429),
        oklch(0.769, 0.188, 70.08),
        oklch(0.577, 0.245, 27.325),
    ]
}

/// The palette color for a series `index`; indexes beyond the palette wrap
/// around
#[must_use]
pub fn series_color(index: usize) -> Color {
    let palette = chart_colors();
    palette[index % palette.len()]
}

// ============================================================================
// Layout math
// ============================================================================

/// Horizontal grid lines layered behind the bars.
fn chart_grid_lines(show_grid: bool) -> floem::AnyView {
    if !show_grid {
        return floem::views::Empty::new().into_any();
    }
    let mut lines: Vec<Box<dyn View>> = Vec::new();
    for i in 0..4 {
        lines.push(Box::new(
            floem::views::Empty::new()
                .style(move |s| {
                    let s = s.w_full().h_px();
                    if i == 0 { s.flex_grow(1.0_f32) } else { s }
                })
                .style(|s| s.with_shadcn_theme(|s, t| s.background(t.border))),
        ));
    }
    floem::views::Stack::vertical_from_iter(lines)
        .style(|s| {
            s.absolute()
                .inset_left(0.0)
                .inset_right(0.0)
                .inset_top(0.0)
                .inset_bottom(0.0)
                .justify_between()
        })
        .into_any()
}

/// One group of bars per category, one bar per series.
#[must_use]
pub fn chart_bar_groups(
    data: &ChartData,
    category_count: usize,
    max_value: f64,
    plot_height: f32,
) -> Vec<Box<dyn View>> {
    let mut groups: Vec<Box<dyn View>> = Vec::new();
    for category in 0..category_count.max(1) {
        let mut bars: Vec<Box<dyn View>> = Vec::new();
        for (series_index, series) in data.series.iter().enumerate() {
            let value = series.points.get(category).map_or(0.0, |p| p.value);
            let height = bar_height(value, max_value, plot_height);
            let color = series.resolved_color(series_index);
            bars.push(Box::new(floem::views::Empty::new().style(move |s| {
                s.flex_grow(1.0_f32)
                    .min_width(8.0)
                    .height(height)
                    .rounded_sm()
                    .with_shadcn_theme(move |s, t| {
                        s.background(color)
                            .hover(move |s| s.background(t.hover_color(color)))
                    })
            })));
        }
        groups.push(Box::new(
            floem::views::Stack::horizontal_from_iter(bars)
                .style(|s| s.flex_grow(1.0_f32).items_end().justify_center().gap_1()),
        ));
    }
    groups
}

/// Height of the bar for `value` inside a plot of `plot_height` px whose
/// largest value is `max_value`.
///
/// Defensive by design: non-finite values are treated as `0.0`, a non-positive
/// `max_value` is treated as `1.0`, and the result is clamped to
/// `[0.0, plot_height]` so the function never divides by zero nor panics.
#[must_use]
pub fn bar_height(value: f64, max_value: f64, plot_height: f32) -> f32 {
    let max = if max_value.is_finite() && max_value > 0.0 {
        max_value
    } else {
        1.0
    };
    let v = if value.is_finite() { value } else { 0.0 };
    let fraction = (v / max).clamp(0.0, 1.0);
    plot_height * fraction.to_f32().unwrap_or(0.0)
}

/// Width of a single bar: the plot width divided evenly between every
/// category × series slot, keeping at least one slot so it never divides by
/// zero.
#[must_use]
pub fn bar_width(plot_width: f32, category_count: usize, series_count: usize) -> f32 {
    let slots = category_count.max(1) * series_count.max(1);
    plot_width / slots.to_f32().unwrap_or(1.0)
}

// ============================================================================
// Chart
// ============================================================================

/// Default plot height in px
pub const DEFAULT_CHART_HEIGHT: f32 = 240.0;

/// Bar chart renderer (the Floem-native equivalent of shadcn/ui
/// `ChartContainer`).
///
/// Renders an optional title/description, a legend, axis + grid lines and one
/// bar per series per category. Bars highlight on hover via a Floem state
/// selector (the tooltip-highlight equivalent of shadcn/ui charts).
pub struct Chart {
    id: ViewId,
    data: ChartData,
    height: f32,
    show_legend: bool,
    show_grid: bool,
}

impl Chart {
    /// Create a new chart from a data set
    #[must_use]
    pub fn new(data: ChartData) -> Self {
        Self {
            id: ViewId::new(),
            data,
            height: DEFAULT_CHART_HEIGHT,
            show_legend: true,
            show_grid: true,
        }
    }

    /// Set the plot height in px (default: 240)
    #[must_use]
    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    /// Show or hide the legend (default: shown)
    #[must_use]
    pub fn legend(mut self, show: bool) -> Self {
        self.show_legend = show;
        self
    }

    /// Show or hide horizontal grid lines (default: shown)
    #[must_use]
    pub fn grid(mut self, show: bool) -> Self {
        self.show_grid = show;
        self
    }

    /// Build the chart view
    ///
    /// The root [`floem::views::Stack`] carries the chart's [`ViewId`] so that
    /// callers can query layout via [`HasViewId::view_id`].
    #[must_use]
    pub fn build(self) -> impl IntoView {
        let chart_id = self.id;
        let data = self.data;
        let plot_height = self.height;
        let category_count = data.category_count();
        let max_value = data.max_value();

        let header: Option<Box<dyn View>> = if let (None, None) = (&data.title, &data.description) {
            None
        } else {
            let mut parts: Vec<Box<dyn View>> = Vec::new();
            if let Some(title) = data.title.clone() {
                parts.push(Box::new(
                    floem::views::Label::new(title).style(|s| s.text_base().font_medium()),
                ));
            }
            if let Some(description) = data.description.clone() {
                parts.push(Box::new(
                    floem::views::Label::new(description)
                        .style(|s| s.text_sm().text_muted_foreground()),
                ));
            }
            Some(Box::new(
                floem::views::Stack::vertical_from_iter(parts)
                    .style(floem_tailwind::TailwindExt::gap_1),
            ))
        };

        let legend_view = if self.show_legend && !data.series.is_empty() {
            ChartLegend::for_data(&data).into_any()
        } else {
            floem::views::Empty::new().into_any()
        };

        // Category labels come from the first series that has them.
        let x_labels: Vec<String> = data
            .series
            .iter()
            .find(|s| !s.points.is_empty())
            .map(|s| s.points.iter().map(|p| p.label.clone()).collect())
            .unwrap_or_default();

        // Plot area: grid lines at the bottom layer, bars on top.
        let grid_lines = chart_grid_lines(self.show_grid);

        let groups = chart_bar_groups(&data, category_count, max_value, plot_height);

        let x_axis_labels = if x_labels.is_empty() {
            floem::views::Empty::new().into_any()
        } else {
            let labels: Vec<Box<dyn View>> = x_labels
                .into_iter()
                .map(|l| {
                    Box::new(
                        floem::views::Label::new(l).style(|s| s.text_xs().text_muted_foreground()),
                    ) as Box<dyn View>
                })
                .collect();
            floem::views::Stack::horizontal_from_iter(labels)
                .style(floem_tailwind::TailwindExt::w_full)
                .into_any()
        };

        let mut plot_children: Vec<Box<dyn View>> = vec![grid_lines.into_any()];
        plot_children.extend(groups);
        let plot = floem::views::Stack::horizontal_from_iter(plot_children).style(move |s| {
            s.relative()
                .w_full()
                .height(plot_height)
                .items_end()
                .gap_2()
        });

        let with_axes = floem::views::Stack::new((
            floem::views::Empty::new()
                .style(|s| s.w_px().flex_grow(0.0_f32))
                .style(|s| s.with_shadcn_theme(|s, t| s.background(t.border))),
            plot,
        ))
        .style(|s| s.w_full().flex_row());

        let mut body: Vec<Box<dyn View>> = Vec::new();
        if let Some(header) = header {
            body.push(header);
        }
        body.push(Box::new(legend_view));
        body.push(Box::new(with_axes));
        body.push(Box::new(x_axis_labels));

        floem::views::Stack::from_iter_with_id(
            chart_id,
            body,
            Some(floem::style::FlexDirection::Column),
        )
        .style(move |s| {
            s.w_full()
                .gap_4()
                .p_4()
                .border_1()
                .rounded_lg()
                .with_shadcn_theme(|s, t| {
                    s.background(t.card)
                        .color(t.card_foreground)
                        .border_color(t.border)
                })
        })
    }
}

impl HasViewId for Chart {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for Chart {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        Box::new(self.build().into_view())
    }
}

/// Alias matching the official shadcn/ui export name
pub type ChartContainer = Chart;

// ============================================================================
// ChartLegend
// ============================================================================

/// Legend listing series names with their color swatches
pub struct ChartLegend {
    id: ViewId,
    entries: Vec<(String, Color)>,
}

impl ChartLegend {
    /// Create a legend from explicit `(name, color)` entries
    #[must_use]
    pub fn new(entries: Vec<(String, Color)>) -> Self {
        Self {
            id: ViewId::new(),
            entries,
        }
    }

    /// Create a legend for a data set, resolving palette colors by index
    #[must_use]
    pub fn for_data(data: &ChartData) -> Self {
        Self {
            id: ViewId::new(),
            entries: data
                .series
                .iter()
                .enumerate()
                .map(|(i, s)| (s.name.clone(), s.resolved_color(i)))
                .collect(),
        }
    }
}

impl HasViewId for ChartLegend {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for ChartLegend {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        let items: Vec<Box<dyn View>> = self
            .entries
            .into_iter()
            .map(|(name, color)| {
                Box::new(
                    floem::views::Stack::horizontal((
                        floem::views::Empty::new().style(move |s| {
                            s.size_2()
                                .rounded_full()
                                .with_shadcn_theme(move |s, _| s.background(color))
                        }),
                        floem::views::Label::new(name)
                            .style(|s| s.text_sm().text_muted_foreground()),
                    ))
                    .style(|s| s.flex_row().items_center().gap_2()),
                ) as Box<dyn View>
            })
            .collect();
        Box::new(
            floem::views::Stack::from_iter_with_id(
                self.id,
                items,
                Some(floem::style::FlexDirection::Row),
            )
            .style(|s| s.flex_wrap(floem::style::FlexWrap::Wrap).gap_4()),
        )
    }
}

/// Alias matching the official shadcn/ui export name
pub type ChartLegendContent = ChartLegend;

// ============================================================================
// ChartTooltip
// ============================================================================

/// A small value box styled like the shadcn/ui chart tooltip.
///
/// shadcn/ui charts show a live tooltip that follows the pointer; Floem has no
/// pointer-tracked overlays for arbitrary views, so this renders the same
/// `series: value` box as a regular view. For pointer-driven tooltips combine
/// it with [`crate::components::tooltip::TooltipExt`] on the bar group.
pub struct ChartTooltip {
    id: ViewId,
    series_name: String,
    value: f64,
    label: Option<String>,
}

impl ChartTooltip {
    /// Create a tooltip box for a series value
    pub fn new(series_name: impl Into<String>, value: f64) -> Self {
        Self {
            id: ViewId::new(),
            series_name: series_name.into(),
            value,
            label: None,
        }
    }

    /// Set the category label shown above the value
    #[must_use]
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }
}

impl HasViewId for ChartTooltip {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for ChartTooltip {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        let category = self.label.unwrap_or_default();
        let value_line = format!("{}: {}", self.series_name, self.value);
        Box::new(
            floem::views::Stack::with_id(
                self.id,
                (
                    floem::views::Label::new(category)
                        .style(|s| s.text_xs().text_muted_foreground()),
                    floem::views::Label::new(value_line).style(|s| s.text_sm().font_medium()),
                ),
            )
            .style(|s| {
                s.flex_col()
                    .gap_0p5()
                    .px_3()
                    .py_1p5()
                    .rounded_md()
                    .border_1()
                    .box_shadow_blur(8.0)
                    .with_shadcn_theme(|s, t| {
                        s.background(t.popover)
                            .color(t.popover_foreground)
                            .border_color(t.border)
                            .box_shadow_color(t.foreground.with_alpha(0.1))
                    })
            }),
        )
    }
}

/// Alias matching the official shadcn/ui export name
pub type ChartTooltipContent = ChartTooltip;
