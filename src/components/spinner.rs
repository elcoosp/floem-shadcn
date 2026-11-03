//! Spinner component – animated loading indicator
//!
//! Based on shadcn/ui Spinner.
//!
//! # Example
//!
//! ```
//! use floem_shadcn::components::spinner::{Spinner, SpinnerSize};
//!
//! let spinner = Spinner::new().size(SpinnerSize::Md);
//! ```

use crate::theme::ShadcnThemeExt;
use floem::prelude::*;
use floem::views::Decorators;
use floem::{HasViewId, ViewId};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
/// The `SpinnerSize`.
pub enum SpinnerSize {
    /// The item.
    Sm,
    #[default]
    /// The item.
    Md,
    /// The item.
    Lg,
}
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
/// The `SpinnerColor`.
pub enum SpinnerColor {
    #[default]
    /// The item.
    Default,
    /// The item.
    Primary,
    /// The item.
    Secondary,
    /// The item.
    Destructive,
}

/// The Spinner.
pub struct Spinner {
    id: ViewId,
    size: SpinnerSize,
    color: SpinnerColor,
}
impl Spinner {
    #[must_use]
    /// Creates a new it.
    pub fn new() -> Self {
        Self {
            id: ViewId::new(),
            size: SpinnerSize::default(),
            color: SpinnerColor::default(),
        }
    }
    #[must_use]
    /// The size.
    pub fn size(mut self, size: SpinnerSize) -> Self {
        self.size = size;
        self
    }
    #[must_use]
    /// The color.
    pub fn color(mut self, color: SpinnerColor) -> Self {
        self.color = color;
        self
    }
}
impl Default for Spinner {
    fn default() -> Self {
        Self::new()
    }
}
impl HasViewId for Spinner {
    fn view_id(&self) -> ViewId {
        self.id
    }
}
impl IntoView for Spinner {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        let size = self.size;
        let color = self.color;
        let (w, h) = match size {
            SpinnerSize::Sm => (16.0, 16.0),
            SpinnerSize::Md => (20.0, 20.0),
            SpinnerSize::Lg => (24.0, 24.0),
        };
        let svg = floem::views::svg(move || SPINNER_SVG.to_string()).style(move |s| {
            s.width(w).height(h).with_shadcn_theme(move |s, t| {
                let c = match color {
                    SpinnerColor::Default => t.foreground,
                    SpinnerColor::Primary => t.primary,
                    SpinnerColor::Secondary => t.secondary,
                    SpinnerColor::Destructive => t.destructive,
                };
                s.color(c)
            })
        });
        // Attach the pre-created ViewId via Container so `view_id()` queries
        // resolve against the rendered view, not an orphan node. The container
        // itself is pinned to the spinner size so it cannot stretch in the
        // parent's cross axis.
        Box::new(
            floem::views::Container::with_id(self.id, svg).style(move |s| s.width(w).height(h)),
        )
    }
}

const SPINNER_SVG: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round"><path d="M21 12a9 9 0 1 1-6.219-8.56"/></svg>"#;
