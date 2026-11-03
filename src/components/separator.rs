//! Separator component with builder-style API
//!
//! Based on shadcn/ui Separator - a visual divider between content.
//!
//! # Example
//!
//! ```
//! use floem_shadcn::components::separator::Separator;
//!
//! // Horizontal separator (default)
//! let sep = Separator::new();
//!
//! // Vertical separator
//! let sep = Separator::new().vertical();
//! ```

use floem::prelude::*;
use floem::views::Decorators;
use floem::{HasViewId, ViewId};
use floem_tailwind::TailwindExt;

use crate::theme::ShadcnThemeExt;

/// Separator orientation
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum SeparatorOrientation {
    #[default]
    /// The item.
    Horizontal,
    /// The item.
    Vertical,
}

/// A styled separator (divider) builder
pub struct Separator {
    id: ViewId,
    orientation: SeparatorOrientation,
}

impl Separator {
    #[must_use]
    /// Create a new horizontal separator
    pub fn new() -> Self {
        Self {
            id: ViewId::new(),
            orientation: SeparatorOrientation::Horizontal,
        }
    }

    #[must_use]
    /// Make the separator vertical
    pub fn vertical(mut self) -> Self {
        self.orientation = SeparatorOrientation::Vertical;
        self
    }

    #[must_use]
    /// Make the separator horizontal (default)
    pub fn horizontal(mut self) -> Self {
        self.orientation = SeparatorOrientation::Horizontal;
        self
    }

    #[must_use]
    /// Build the separator view
    pub fn build(self) -> impl IntoView {
        let orientation = self.orientation;

        floem::views::Empty::new().style(move |s| {
            let base = s.flex_shrink(0.0_f32);
            let base = match orientation {
                SeparatorOrientation::Horizontal => base.w_full().h_px(),
                SeparatorOrientation::Vertical => base.h_full().w_px(),
            };
            base.with_shadcn_theme(|s, t| s.background(t.border))
        })
    }
}

impl Default for Separator {
    fn default() -> Self {
        Self::new()
    }
}

impl HasViewId for Separator {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for Separator {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }

    fn into_view(self) -> Self::V {
        Box::new(self.build().into_view())
    }
}
