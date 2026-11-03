//! Carousel component with builder-style API
//!
//! Based on shadcn/ui Carousel - a carousel for cycling through content.
//!
//! # Example
//!
//! ```
//! use floem::reactive::RwSignal;
//! use floem_shadcn::components::carousel::*;
//! # use floem::views::text;
//!
//! let current = RwSignal::new(0);
//! # let content_view = text("Slide 1");
//!
//! Carousel::new(current, 3)
//!     .items(content_view);
//! ```

use crate::theme::ShadcnThemeExt;
use floem::prelude::*;
use floem::reactive::{RwSignal, SignalUpdate};
use floem::style::CursorStyle;
use floem::views::Decorators;
use floem::{HasViewId, ViewId};
use floem_tailwind::TailwindExt;

/// Carousel orientation
#[derive(Clone, Copy, Default, PartialEq)]
pub enum CarouselOrientation {
    #[default]
    /// The item.
    Horizontal,
    /// The item.
    Vertical,
}

// ============================================================================
// Carousel
// ============================================================================

/// The Carousel.
pub struct Carousel<I> {
    id: ViewId,
    current: RwSignal<usize>,
    total: usize,
    items: I,
    orientation: CarouselOrientation,
    show_arrows: bool,
}

impl Carousel<()> {
    #[must_use]
    /// Creates a new it.
    pub fn new(current: RwSignal<usize>, total: usize) -> Self {
        Self {
            id: ViewId::new(),
            current,
            total,
            items: (),
            orientation: CarouselOrientation::Horizontal,
            show_arrows: true,
        }
    }
}

impl<I> Carousel<I> {
    /// The items.
    pub fn items<I2: IntoView + 'static>(self, items: I2) -> Carousel<I2> {
        Carousel {
            id: self.id,
            current: self.current,
            total: self.total,
            items,
            orientation: self.orientation,
            show_arrows: self.show_arrows,
        }
    }

    #[must_use]
    /// The orientation.
    pub fn orientation(mut self, orientation: CarouselOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    #[must_use]
    /// The horizontal.
    pub fn horizontal(mut self) -> Self {
        self.orientation = CarouselOrientation::Horizontal;
        self
    }

    #[must_use]
    /// The vertical.
    pub fn vertical(mut self) -> Self {
        self.orientation = CarouselOrientation::Vertical;
        self
    }

    #[must_use]
    /// The arrows.
    pub fn arrows(mut self, show: bool) -> Self {
        self.show_arrows = show;
        self
    }
}

impl<I: IntoView + 'static> HasViewId for Carousel<I> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<I: IntoView + 'static> IntoView for Carousel<I> {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }

    fn into_view(self) -> Self::V {
        let current = self.current;
        let total = self.total;
        let orientation = self.orientation;
        let show_arrows = self.show_arrows;

        let prev_button = if show_arrows {
            floem::views::Label::new(match orientation {
                CarouselOrientation::Horizontal => "<",
                CarouselOrientation::Vertical => "^",
            })
            .style(|s| {
                s.with_shadcn_theme(move |s, t| {
                    s.w_10()
                        .h_10()
                        .text_lg()
                        .color(t.foreground)
                        .background(t.background)
                        .border_1()
                        .border_color(t.border)
                        .rounded_md()
                        .cursor(CursorStyle::Pointer)
                        .flex()
                        .items_center()
                        .justify_center()
                        .hover(|s| s.background(t.accent))
                })
            })
            .on_event_stop(floem::event::listener::Click, move |_, ()| {
                current.update(|c| {
                    if *c > 0 {
                        *c -= 1;
                    } else {
                        *c = total.saturating_sub(1);
                    }
                });
            })
            .into_any()
        } else {
            floem::views::Empty::new().into_any()
        };

        let next_button = if show_arrows {
            floem::views::Label::new(match orientation {
                CarouselOrientation::Horizontal => ">",
                CarouselOrientation::Vertical => "v",
            })
            .style(|s| {
                s.with_shadcn_theme(move |s, t| {
                    s.w_10()
                        .h_10()
                        .text_lg()
                        .color(t.foreground)
                        .background(t.background)
                        .border_1()
                        .border_color(t.border)
                        .rounded_md()
                        .cursor(CursorStyle::Pointer)
                        .flex()
                        .items_center()
                        .justify_center()
                        .hover(|s| s.background(t.accent))
                })
            })
            .on_event_stop(floem::event::listener::Click, move |_, ()| {
                current.update(|c| {
                    if *c < total.saturating_sub(1) {
                        *c += 1;
                    } else {
                        *c = 0;
                    }
                });
            })
            .into_any()
        } else {
            floem::views::Empty::new().into_any()
        };

        let content = floem::views::Container::new(self.items)
            .style(|s| s.flex_grow(1.0_f32).flex().items_center().justify_center());

        let carousel_body = match orientation {
            CarouselOrientation::Horizontal => {
                floem::views::Stack::horizontal((prev_button, content, next_button))
                    .style(|s| s.w_full().items_center().gap_2())
                    .into_any()
            }
            CarouselOrientation::Vertical => {
                floem::views::Stack::vertical((prev_button, content, next_button))
                    .style(|s| s.h_full().items_center().gap_2())
                    .into_any()
            }
        };

        Box::new(carousel_body)
    }
}

// ============================================================================
// CarouselItem
// ============================================================================

/// The `CarouselItem`.
pub struct CarouselItem<V> {
    id: ViewId,
    child: V,
}

impl<V: IntoView + 'static> CarouselItem<V> {
    /// Creates a new it.
    pub fn new(child: V) -> Self {
        Self {
            id: ViewId::new(),
            child,
        }
    }
}

impl<V: IntoView + 'static> HasViewId for CarouselItem<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<V: IntoView + 'static> IntoView for CarouselItem<V> {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }

    fn into_view(self) -> Self::V {
        Box::new(
            floem::views::Container::with_id(self.id, self.child)
                .style(|s| s.w_full().flex().items_center().justify_center()),
        )
    }
}

// ============================================================================
// CarouselContent - Shows content based on current index using visibility
// ============================================================================

/// The `CarouselContent`.
pub struct CarouselContent;

impl CarouselContent {
    #[must_use]
    /// The usage hint.
    pub fn usage_hint() -> &'static str {
        "Use Label::reactive or conditional rendering based on current signal"
    }
}

// ============================================================================
// CarouselPrevious / CarouselNext (standalone buttons)
// ============================================================================

/// The `CarouselPrevious`.
pub struct CarouselPrevious {
    id: ViewId,
    current: RwSignal<usize>,
    total: usize,
    wrap: bool,
}

impl CarouselPrevious {
    #[must_use]
    /// Creates a new it.
    pub fn new(current: RwSignal<usize>, total: usize) -> Self {
        Self {
            id: ViewId::new(),
            current,
            total,
            wrap: true,
        }
    }

    #[must_use]
    /// The wrap.
    pub fn wrap(mut self, wrap: bool) -> Self {
        self.wrap = wrap;
        self
    }
}

impl HasViewId for CarouselPrevious {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for CarouselPrevious {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }

    fn into_view(self) -> Self::V {
        let current = self.current;
        let total = self.total;
        let wrap = self.wrap;

        Box::new(
            floem::views::Label::new("<")
                .style(|s| {
                    s.with_shadcn_theme(move |s, t| {
                        s.w_10()
                            .h_10()
                            .text_lg()
                            .color(t.foreground)
                            .background(t.background)
                            .border_1()
                            .border_color(t.border)
                            .rounded_full()
                            .cursor(CursorStyle::Pointer)
                            .flex()
                            .items_center()
                            .justify_center()
                            .hover(|s| s.background(t.accent))
                    })
                })
                .on_event_stop(floem::event::listener::Click, move |_, ()| {
                    current.update(|c| {
                        if *c > 0 {
                            *c -= 1;
                        } else if wrap {
                            *c = total.saturating_sub(1);
                        }
                    });
                }),
        )
    }
}

/// The `CarouselNext`.
pub struct CarouselNext {
    id: ViewId,
    current: RwSignal<usize>,
    total: usize,
    wrap: bool,
}

impl CarouselNext {
    #[must_use]
    /// Creates a new it.
    pub fn new(current: RwSignal<usize>, total: usize) -> Self {
        Self {
            id: ViewId::new(),
            current,
            total,
            wrap: true,
        }
    }

    #[must_use]
    /// The wrap.
    pub fn wrap(mut self, wrap: bool) -> Self {
        self.wrap = wrap;
        self
    }
}

impl HasViewId for CarouselNext {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for CarouselNext {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }

    fn into_view(self) -> Self::V {
        let current = self.current;
        let total = self.total;
        let wrap = self.wrap;

        Box::new(
            floem::views::Label::new(">")
                .style(|s| {
                    s.with_shadcn_theme(move |s, t| {
                        s.w_10()
                            .h_10()
                            .text_lg()
                            .color(t.foreground)
                            .background(t.background)
                            .border_1()
                            .border_color(t.border)
                            .rounded_full()
                            .cursor(CursorStyle::Pointer)
                            .flex()
                            .items_center()
                            .justify_center()
                            .hover(|s| s.background(t.accent))
                    })
                })
                .on_event_stop(floem::event::listener::Click, move |_, ()| {
                    current.update(|c| {
                        if *c < total.saturating_sub(1) {
                            *c += 1;
                        } else if wrap {
                            *c = 0;
                        }
                    });
                }),
        )
    }
}
