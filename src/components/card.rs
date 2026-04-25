//! Card component with builder-style API
//!
//! Based on shadcn/ui Card component with header, content, and footer sections.
//!
//! # Example
//!
//! ```rust
//! use floem_shadcn::components::card::{Card, CardHeader, CardContent, CardFooter};
//!
//! let card = Card::new((
//!     CardHeader::new()
//!         .title("Create project")
//!         .description("Deploy your new project in one-click."),
//!     CardContent::new(content),
//!     CardFooter::new(buttons),
//! ));
//! ```

use floem::prelude::*;
use floem::view::IntoViewIter;
use floem::views::Decorators;
use floem::{HasViewId, ViewId};

use crate::theme::ShadcnThemeExt;
use crate::styled::ShadcnStyleExt;

// ============================================================================
// Card
// ============================================================================

/// Card container builder
pub struct Card<C> {
    id: ViewId,
    children: C,
}

impl<C: IntoViewIter> Card<C> {
    /// Create a new card with the given children
    pub fn new(children: C) -> Self {
        Self {
            id: ViewId::new(),
            children,
        }
    }

    /// Build the card view with reactive styling
    pub fn build(self) -> impl IntoView {
        floem::views::Stack::vertical_from_iter(self.children.into_view_iter()).style(|s| {
            s.gap(24.0)
                .border_radius(12.0)
                .border(1.0)
                .padding_top(24.0).padding_bottom(24.0)
                // Note: shadow-sm not available in floem-tailwind yet
                .with_shadcn_theme(|s, t| {
                    s.border_color(t.border)
                        .background(t.card)
                        .color(t.card_foreground)
                })
        })
    }
}

impl<C: IntoViewIter> HasViewId for Card<C> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<C: IntoViewIter> IntoView for Card<C> {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate { self.into_view() }



    fn into_view(self) -> Self::V {
        Box::new(self.build().into_view())
    }
}

// ============================================================================
// CardHeader
// ============================================================================

/// Card header builder
pub struct CardHeader {
    id: ViewId,
    title: Option<String>,
    description: Option<String>,
}

impl CardHeader {
    /// Create a new card header
    pub fn new() -> Self {
        Self {
            id: ViewId::new(),
            title: None,
            description: None,
        }
    }

    /// Set the header title
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set the header description
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }
}

impl Default for CardHeader {
    fn default() -> Self {
        Self::new()
    }
}

impl HasViewId for CardHeader {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for CardHeader {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate { self.into_view() }



    fn into_view(self) -> Self::V {
        let mut children: Vec<Box<dyn View>> = Vec::new();

        if let Some(title) = self.title {
            children
                .push(Box::new(Label::derived(move || title.clone()).style(|s| {
                    s.font_size(18.0).line_height(1.0)
                })));
        }

        if let Some(description) = self.description {
            children.push(Box::new(Label::derived(move || description.clone()).style(
                |s| {
                    s.font_size(14.0)
                        .line_height(1.43) // 20px / 14px
                        .text_muted_foreground()
                },
            )));
        }

        Box::new(floem::views::Stack::vertical_from_iter(children).style(|s| s.gap(8.0).padding_left(24.0).padding_right(24.0))) // gap-2 px-6
    }
}

// ============================================================================
// CardContent
// ============================================================================

/// Card content section builder
pub struct CardContent<V> {
    id: ViewId,
    child: V,
}

impl<V: IntoView + 'static> CardContent<V> {
    /// Create a new card content section
    pub fn new(child: V) -> Self {
        Self {
            id: ViewId::new(),
            child,
        }
    }
}

impl<V: IntoView + 'static> HasViewId for CardContent<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<V: IntoView + 'static> IntoView for CardContent<V> {
    type V = Box<dyn View>;

    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate { self.into_view() }


    fn into_view(self) -> Self::V {
        Box::new(floem::views::Container::with_id(self.id, self.child).style(|s| s.padding_left(24.0).padding_right(24.0)))
    }
}

// ============================================================================
// CardFooter
// ============================================================================

/// Card footer section builder
pub struct CardFooter<V> {
    id: ViewId,
    child: V,
}

impl<V: IntoView + 'static> CardFooter<V> {
    /// Create a new card footer section
    pub fn new(child: V) -> Self {
        Self {
            id: ViewId::new(),
            child,
        }
    }
}

impl<V: IntoView + 'static> HasViewId for CardFooter<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<V: IntoView + 'static> IntoView for CardFooter<V> {
    type V = Box<dyn View>;

    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate { self.into_view() }


    fn into_view(self) -> Self::V {
        Box::new(
            floem::views::Container::with_id(self.id, self.child).style(|s| {
                s.flex().items_center().padding_left(24.0).padding_right(24.0) // flex items-center px-6
            }),
        )
    }
}

// ============================================================================
// CardTitle (standalone, for custom headers)
// ============================================================================

/// Standalone card title builder
pub struct CardTitle<V> {
    id: ViewId,
    child: V,
}

impl<V: IntoView + 'static> CardTitle<V> {
    /// Create a new card title
    pub fn new(child: V) -> Self {
        Self {
            id: ViewId::new(),
            child,
        }
    }
}

impl<V: IntoView + 'static> HasViewId for CardTitle<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<V: IntoView + 'static> IntoView for CardTitle<V> {
    type V = Box<dyn View>;

    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate { self.into_view() }


    fn into_view(self) -> Self::V {
        Box::new(
            floem::views::Container::with_id(self.id, self.child)
                .style(|s| s.font_size(18.0).line_height(1.0)),
        )
    }
}

// ============================================================================
// CardDescription (standalone, for custom headers)
// ============================================================================

/// Standalone card description builder
pub struct CardDescription<V> {
    id: ViewId,
    child: V,
}

impl<V: IntoView + 'static> CardDescription<V> {
    /// Create a new card description
    pub fn new(child: V) -> Self {
        Self {
            id: ViewId::new(),
            child,
        }
    }
}

impl<V: IntoView + 'static> HasViewId for CardDescription<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<V: IntoView + 'static> IntoView for CardDescription<V> {
    type V = Box<dyn View>;

    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate { self.into_view() }


    fn into_view(self) -> Self::V {
        Box::new(
            floem::views::Container::with_id(self.id, self.child).style(|s| {
                s.font_size(14.0)
                    .line_height(1.43) // 20px / 14px
                    .text_muted_foreground()
            }),
        )
    }
}
