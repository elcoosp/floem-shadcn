//! Field component – form layout primitive
//!
//! Port of shadcn/ui Field: pairs labels, descriptions, errors and controls
//! into accessible form rows. A `Field` is a flex container with an
//! `orientation` (vertical, horizontal or responsive); `FieldGroup`,
//! `FieldSet` and `FieldSeparator` provide the surrounding scaffolding.
//!
//! # Example
//!
//! ```
//! use floem::view::ParentView;
//! use floem_shadcn::components::field::*;
//!
//! Field::new()
//!     .orientation(FieldOrientation::Vertical)
//!     .child(
//!         FieldLabel::new("Name")
//!     )
//!     .child(
//!         FieldContent::new()
//!             .child(FieldDescription::new("We never share your data."))
//!             .child(FieldError::new("Name is required."))
//!     );
//! ```

use crate::styled::ShadcnStyleExt;
use crate::theme::ShadcnThemeExt;
use floem::prelude::*;
use floem::style::Style;
use floem::view::IntoViewIter;
use floem::view::ParentView;
use floem::views::Decorators;
use floem::{HasViewId, ViewId};
use floem_tailwind::TailwindExt;

// ============================================================================
// Field
// ============================================================================

/// Layout orientation of a [`Field`].
///
/// Mirrors the shadcn/ui `orientation` variants. The `Responsive` variant
/// stacks vertically in Floem (container queries are not supported) and is
/// provided for API parity.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum FieldOrientation {
    /// Stack label and control vertically (`flex-col`).
    #[default]
    Vertical,
    /// Place label and control on one row (`flex-row items-center`).
    Horizontal,
    /// Vertical at small sizes, horizontal at `@md` and above.
    ///
    /// In Floem this behaves like [`FieldOrientation::Vertical`] because
    /// container queries are unavailable.
    Responsive,
}

/// A form field container pairing labels, descriptions, errors and controls.
///
/// Children are attached with `.child(...)` (see [`floem::view::ParentView`]).
pub struct Field {
    id: ViewId,
    orientation: FieldOrientation,
    invalid: bool,
    optimistic: bool,
    children: Vec<floem::AnyView>,
}

impl Field {
    /// Create a new vertical field
    #[must_use]
    pub fn new() -> Self {
        Self {
            id: ViewId::new(),
            orientation: FieldOrientation::Vertical,
            invalid: false,
            optimistic: false,
            children: Vec::new(),
        }
    }

    /// Set the field orientation (default: vertical)
    #[must_use]
    pub fn orientation(mut self, orientation: FieldOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Mark the field as invalid; error styling is applied to children
    #[must_use]
    pub fn invalid(mut self, invalid: bool) -> Self {
        self.invalid = invalid;
        self
    }

    /// Render the field in an optimistic (in-flight) state: dimmed content
    #[must_use]
    pub fn optimistic(mut self, optimistic: bool) -> Self {
        self.optimistic = optimistic;
        self
    }
}

impl Default for Field {
    fn default() -> Self {
        Self::new()
    }
}

impl HasViewId for Field {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for Field {
    type V = Container;
    type Intermediate = Container;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        let orientation = self.orientation;
        let invalid = self.invalid;
        let optimistic = self.optimistic;
        let container = Container::with_id(self.id, ()).style(move |s| {
            let s = s.w_full().gap_3();
            let s = match orientation {
                FieldOrientation::Vertical | FieldOrientation::Responsive => s.flex_col(),
                FieldOrientation::Horizontal => s.flex_row().items_center(),
            };
            let s = if optimistic {
                field_optimistic_style(s)
            } else {
                s
            };
            if invalid {
                s.with_shadcn_theme(|s, t| s.color(t.destructive))
            } else {
                s
            }
        });
        self.id.set_children_vec(self.children);
        container
    }
}

impl ParentView for Field {
    fn child(mut self, child: impl IntoView + 'static) -> Self {
        self.children.push(child.into_any());
        self
    }
    fn children(mut self, children: impl IntoViewIter + 'static) -> Self {
        self.children.extend(children.into_view_iter());
        self
    }
}

/// Applies the dimmed "optimistic" style used while a form submission is
/// in flight (shadcn/ui applies the same treatment via `data-state=optimistic`).
#[must_use]
pub fn field_optimistic_style(s: Style) -> Style {
    s.opacity_50()
}

// ============================================================================
// FieldGroup
// ============================================================================

/// Groups multiple related [`Field`]s with vertical rhythm
pub struct FieldGroup {
    id: ViewId,
    children: Vec<floem::AnyView>,
}

impl FieldGroup {
    /// Create a new field group
    #[must_use]
    pub fn new() -> Self {
        Self {
            id: ViewId::new(),
            children: Vec::new(),
        }
    }
}

impl Default for FieldGroup {
    fn default() -> Self {
        Self::new()
    }
}

impl HasViewId for FieldGroup {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for FieldGroup {
    type V = Container;
    type Intermediate = Container;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        let container = Container::with_id(self.id, ()).style(|s| s.flex_col().w_full().gap_7());
        self.id.set_children_vec(self.children);
        container
    }
}

impl ParentView for FieldGroup {
    fn child(mut self, child: impl IntoView + 'static) -> Self {
        self.children.push(child.into_any());
        self
    }
    fn children(mut self, children: impl IntoViewIter + 'static) -> Self {
        self.children.extend(children.into_view_iter());
        self
    }
}

// ============================================================================
// FieldSet
// ============================================================================

/// Semantic grouping of fields (fieldset) with a legend
pub struct FieldSet {
    id: ViewId,
    children: Vec<floem::AnyView>,
}

impl FieldSet {
    /// Create a new field set
    #[must_use]
    pub fn new() -> Self {
        Self {
            id: ViewId::new(),
            children: Vec::new(),
        }
    }
}

impl Default for FieldSet {
    fn default() -> Self {
        Self::new()
    }
}

impl HasViewId for FieldSet {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for FieldSet {
    type V = Container;
    type Intermediate = Container;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        let container = Container::with_id(self.id, ()).style(|s| s.flex_col().w_full().gap_6());
        self.id.set_children_vec(self.children);
        container
    }
}

impl ParentView for FieldSet {
    fn child(mut self, child: impl IntoView + 'static) -> Self {
        self.children.push(child.into_any());
        self
    }
    fn children(mut self, children: impl IntoViewIter + 'static) -> Self {
        self.children.extend(children.into_view_iter());
        self
    }
}

// ============================================================================
// FieldLegend
// ============================================================================

/// Variant of a [`FieldLegend`]: real legend or label-styled legend
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum FieldLegendVariant {
    /// Larger legend text (`text-base font-medium`)
    #[default]
    Legend,
    /// Smaller, label-like legend text (`text-sm font-medium`)
    Label,
}

/// Legend for a [`FieldSet`]
pub struct FieldLegend {
    id: ViewId,
    text: String,
    variant: FieldLegendVariant,
}

impl FieldLegend {
    /// Create a new legend with the default variant
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            id: ViewId::new(),
            text: text.into(),
            variant: FieldLegendVariant::Legend,
        }
    }

    /// Create a label-styled legend
    pub fn label(text: impl Into<String>) -> Self {
        Self {
            id: ViewId::new(),
            text: text.into(),
            variant: FieldLegendVariant::Label,
        }
    }

    /// Set the legend variant
    #[must_use]
    pub fn variant(mut self, variant: FieldLegendVariant) -> Self {
        self.variant = variant;
        self
    }
}

impl HasViewId for FieldLegend {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for FieldLegend {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        let variant = self.variant;
        Box::new(
            floem::views::Label::with_id(self.id, self.text).style(move |s| {
                let s = s.mb_3().font_medium();
                match variant {
                    FieldLegendVariant::Legend => s.text_base(),
                    FieldLegendVariant::Label => s.text_sm(),
                }
            }),
        )
    }
}

// ============================================================================
// FieldLabel
// ============================================================================

/// Label for a [`Field`]
pub struct FieldLabel {
    id: ViewId,
    text: String,
}

impl FieldLabel {
    /// Create a new field label
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            id: ViewId::new(),
            text: text.into(),
        }
    }
}

impl HasViewId for FieldLabel {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for FieldLabel {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        Box::new(floem::views::Label::with_id(self.id, self.text).style(|s| {
            s.flex()
                .gap_2()
                .items_center()
                .text_sm()
                .leading_snug()
                .font_medium()
        }))
    }
}

// ============================================================================
// FieldTitle
// ============================================================================

/// Bold title used inside [`FieldContent`] or [`FieldLabel`]
pub struct FieldTitle {
    id: ViewId,
    text: String,
}

impl FieldTitle {
    /// Create a new field title
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            id: ViewId::new(),
            text: text.into(),
        }
    }
}

impl HasViewId for FieldTitle {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for FieldTitle {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        Box::new(
            floem::views::Label::with_id(self.id, self.text)
                .style(|s| s.text_sm().leading_snug().font_medium()),
        )
    }
}

// ============================================================================
// FieldContent
// ============================================================================

/// Holds the title/description stack of a [`Field`]
pub struct FieldContent {
    id: ViewId,
    children: Vec<floem::AnyView>,
}

impl FieldContent {
    /// Create a new field content container
    #[must_use]
    pub fn new() -> Self {
        Self {
            id: ViewId::new(),
            children: Vec::new(),
        }
    }
}

impl Default for FieldContent {
    fn default() -> Self {
        Self::new()
    }
}

impl HasViewId for FieldContent {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for FieldContent {
    type V = Container;
    type Intermediate = Container;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        let container =
            Container::with_id(self.id, ()).style(|s| s.flex_col().flex_grow(1.0_f32).gap_1p5());
        self.id.set_children_vec(self.children);
        container
    }
}

impl ParentView for FieldContent {
    fn child(mut self, child: impl IntoView + 'static) -> Self {
        self.children.push(child.into_any());
        self
    }
    fn children(mut self, children: impl IntoViewIter + 'static) -> Self {
        self.children.extend(children.into_view_iter());
        self
    }
}

// ============================================================================
// FieldDescription
// ============================================================================

/// Helper text rendered below a field title or control
pub struct FieldDescription {
    id: ViewId,
    text: String,
}

impl FieldDescription {
    /// Create a new field description
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            id: ViewId::new(),
            text: text.into(),
        }
    }
}

impl HasViewId for FieldDescription {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for FieldDescription {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        Box::new(
            floem::views::Label::with_id(self.id, self.text)
                .style(|s| s.text_sm().leading_normal().text_muted_foreground()),
        )
    }
}

// ============================================================================
// FieldError
// ============================================================================

/// Validation error text rendered in the destructive color
pub struct FieldError {
    id: ViewId,
    errors: Vec<String>,
}

impl FieldError {
    /// Create a new field error with a single message
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            id: ViewId::new(),
            errors: vec![text.into()],
        }
    }

    /// Create a field error from a list of messages; returns `None` when the
    /// list is empty (mirroring shadcn/ui rendering `null` without errors)
    #[must_use]
    pub fn from_errors(errors: Vec<String>) -> Option<Self> {
        if errors.is_empty() {
            None
        } else {
            Some(Self {
                id: ViewId::new(),
                errors,
            })
        }
    }
}

impl HasViewId for FieldError {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for FieldError {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        let single = self.errors.len() == 1;
        let children: Vec<Box<dyn View>> = if single {
            vec![Box::new(floem::views::Label::new(
                self.errors.first().cloned().unwrap_or_default(),
            ))]
        } else {
            self.errors
                .iter()
                .map(|e| Box::new(floem::views::Label::new(e.clone())) as Box<dyn View>)
                .collect()
        };
        Box::new(
            floem::views::Stack::vertical_from_iter(children)
                .style(move |s| {
                    let s = s.text_sm();
                    if single { s } else { s.gap_1() }
                })
                .style(|s| s.with_shadcn_theme(|s, t| s.color(t.destructive))),
        )
    }
}

// ============================================================================
// FieldSeparator
// ============================================================================

/// Divider between field groups, optionally carrying content such as "OR"
pub struct FieldSeparator {
    id: ViewId,
    content: Option<String>,
}

impl FieldSeparator {
    /// Create a plain separator line
    #[must_use]
    pub fn new() -> Self {
        Self {
            id: ViewId::new(),
            content: None,
        }
    }

    /// Create a separator with centered content (e.g. "OR")
    pub fn with_content(content: impl Into<String>) -> Self {
        Self {
            id: ViewId::new(),
            content: Some(content.into()),
        }
    }
}

impl Default for FieldSeparator {
    fn default() -> Self {
        Self::new()
    }
}

impl HasViewId for FieldSeparator {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for FieldSeparator {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        let content = self.content;
        Box::new(
            floem::views::Stack::with_id(
                self.id,
                (
                    floem::views::Empty::new()
                        .style(|s| s.flex_grow(1.0_f32).h_px())
                        .style(|s| s.with_shadcn_theme(|s, t| s.background(t.border))),
                    if let Some(text) = content {
                        floem::views::Label::new(text)
                            .style(|s| s.px_2().text_xs().text_muted_foreground())
                            .into_any()
                    } else {
                        floem::views::Empty::new().into_any()
                    },
                    floem::views::Empty::new()
                        .style(|s| s.flex_grow(1.0_f32).h_px())
                        .style(|s| s.with_shadcn_theme(|s, t| s.background(t.border))),
                ),
            )
            .style(|s| s.flex_row().items_center().w_full().gap_2().h_5()),
        )
    }
}

// ============================================================================
// FieldAction
// ============================================================================

/// Right-aligned action slot inside a horizontal [`Field`]
pub struct FieldAction {
    id: ViewId,
    children: Vec<floem::AnyView>,
}

impl FieldAction {
    /// Create a new field action slot
    #[must_use]
    pub fn new() -> Self {
        Self {
            id: ViewId::new(),
            children: Vec::new(),
        }
    }
}

impl Default for FieldAction {
    fn default() -> Self {
        Self::new()
    }
}

impl HasViewId for FieldAction {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for FieldAction {
    type V = Container;
    type Intermediate = Container;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        let container =
            Container::with_id(self.id, ()).style(|s| s.ml_auto().px_3().flex_row().items_center());
        self.id.set_children_vec(self.children);
        container
    }
}

impl ParentView for FieldAction {
    fn child(mut self, child: impl IntoView + 'static) -> Self {
        self.children.push(child.into_any());
        self
    }
    fn children(mut self, children: impl IntoViewIter + 'static) -> Self {
        self.children.extend(children.into_view_iter());
        self
    }
}
