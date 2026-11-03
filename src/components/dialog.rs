//! Dialog component with builder-style API
//!
//! Based on shadcn/ui Dialog component for modal dialogs.
//!
//! # Example (recommended)
//!
//! ```
//! use floem_shadcn::components::dialog::{Dialog, DialogTrigger, DialogContent, DialogHeader, DialogFooter, DialogClose};
//! use floem_shadcn::components::button::Button;
//! # use floem::prelude::*;
//!
//! Dialog::new((
//!     DialogTrigger::new(Button::new("Open Dialog")),
//!     DialogContent::new((
//!         DialogHeader::new()
//!             .title("Are you sure?")
//!             .description("This action cannot be undone."),
//!         DialogFooter::new((
//!             DialogClose::new(Button::new("Cancel").outline()),
//!             Button::new("Continue").on_event_stop(floem::event::listener::Click, move |_, _| {
//!                 // do something
//!             }),
//!         )),
//!     )),
//! ));
//! ```
//!
//! # Components
//!
//! - `Dialog` - Root component that provides context
//! - `DialogTrigger` - Opens the dialog when clicked
//! - `DialogContent` - The modal content (includes overlay/backdrop automatically)
//! - `DialogHeader` - Container for title and description
//! - `DialogFooter` - Container for action buttons
//! - `DialogClose` - Closes the dialog when clicked
//!
//! # External state control
//!
//! Use `dialog.open_signal()` to get the signal for programmatic control:
//!
//! ```
//! # use floem_shadcn::components::dialog::Dialog;
//! # use floem::views::text;
//! # let trigger = text("trigger");
//! # let content = text("content");
//! let dialog = Dialog::new((trigger, content));
//! let open = dialog.open_signal();
//! // Later: open.set(true) to open programmatically
//! ```

use floem::prelude::*;
use floem::reactive::{Context, RwSignal, Scope, SignalGet, SignalUpdate};
use floem::views::Decorators;
use floem::{HasViewId, ViewId};
use floem_tailwind::TailwindExt;

use crate::theme::ShadcnThemeExt;

// ============================================================================
// Dialog Context - passes open signal to children via reactive Context
// ============================================================================

#[derive(Clone, Copy)]
/// The `DialogContext`.
pub struct DialogContext {
    /// The item.
    pub open: RwSignal<bool>,
}

// ============================================================================
// Dialog
// ============================================================================

/// The Dialog.
pub struct Dialog<V> {
    id: ViewId,
    open: RwSignal<bool>,
    child: V,
    scope: Scope,
}

impl<V: IntoView + 'static> Dialog<V> {
    /// Creates a new it.
    pub fn new(child: V) -> Self {
        let open = RwSignal::new(false);
        let scope = Scope::current().create_child();
        scope.provide_context(DialogContext { open });
        Self {
            id: ViewId::new(),
            open,
            child,
            scope,
        }
    }

    /// The open signal.
    pub fn open_signal(&self) -> RwSignal<bool> {
        self.open
    }
}

impl<V: IntoView + 'static> HasViewId for Dialog<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<V: IntoView + 'static> IntoView for Dialog<V> {
    type V = Box<dyn View>;

    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }

    fn into_view(self) -> Self::V {
        let scope = self.scope;
        let child = self.child;
        let id = self.id;
        Box::new(scope.enter(move || {
            floem::views::Container::with_id(id, child).style(|s| s.w_full().h_full())
        }))
    }
}

// ============================================================================
// DialogTrigger
// ============================================================================

/// The `DialogTrigger`.
pub struct DialogTrigger<V> {
    id: ViewId,
    child: V,
}

impl<V: IntoView + 'static> DialogTrigger<V> {
    /// Creates a new it.
    pub fn new(child: V) -> Self {
        Self {
            id: ViewId::new(),
            child,
        }
    }
}

impl<V: IntoView + 'static> HasViewId for DialogTrigger<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<V: IntoView + 'static> IntoView for DialogTrigger<V> {
    type V = Box<dyn View>;

    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }

    fn into_view(self) -> Self::V {
        let ctx = Context::get::<DialogContext>();
        Box::new(
            floem::views::Container::with_id(self.id, self.child).on_event_stop(
                floem::event::listener::Click,
                move |_, ()| {
                    if let Some(ctx) = ctx {
                        ctx.open.set(true);
                    }
                },
            ),
        )
    }
}

// ============================================================================
// DialogClose
// ============================================================================

/// The `DialogClose`.
pub struct DialogClose<V> {
    id: ViewId,
    child: V,
}

impl<V: IntoView + 'static> DialogClose<V> {
    /// Creates a new it.
    pub fn new(child: V) -> Self {
        Self {
            id: ViewId::new(),
            child,
        }
    }
}

impl<V: IntoView + 'static> HasViewId for DialogClose<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<V: IntoView + 'static> IntoView for DialogClose<V> {
    type V = Box<dyn View>;

    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }

    fn into_view(self) -> Self::V {
        let ctx = Context::get::<DialogContext>();
        Box::new(
            floem::views::Container::with_id(self.id, self.child).on_event_stop(
                floem::event::listener::Click,
                move |_, ()| {
                    if let Some(ctx) = ctx {
                        ctx.open.set(false);
                    }
                },
            ),
        )
    }
}

// ============================================================================
// DialogContent
// ============================================================================

/// The `DialogContent`.
pub struct DialogContent {
    id: ViewId,
    children: Vec<Box<dyn View>>,
}

impl DialogContent {
    /// Creates a new it.
    pub fn new(children: impl floem::view::IntoViewIter) -> Self {
        Self {
            id: ViewId::new(),
            children: children.into_view_iter().collect(),
        }
    }
}

impl HasViewId for DialogContent {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for DialogContent {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }

    fn into_view(self) -> Self::V {
        let id = self.id;
        let children = self.children;
        let ctx = Context::get::<DialogContext>();
        if let Some(ctx) = ctx {
            let open = ctx.open;
            // The fixed wrapper IS the backdrop: it fills the viewport, dims the
            // page and closes the dialog on outside clicks. The content panel
            // swallows clicks so interacting inside the dialog never closes it.
            // (A separate absolute-positioned backdrop child does not survive
            // understory box-tree hit-testing reliably; the wrapper-as-backdrop
            // shape does, including after hide/show cycles.)
            Box::new(
                floem::views::Container::with_id(
                    id,
                    floem::views::Stack::vertical_from_iter(children)
                        .style(move |s| {
                            s.absolute()
                                .inset_left_pct(50.0)
                                .inset_top_pct(50.0)
                                .z_index(10)
                                .max_width(512.0)
                                .rounded_lg()
                                .p_6()
                                .gap_4()
                                .box_shadow_blur(8.0)
                                .box_shadow_color(floem::peniko::Color::from_rgba8(0, 0, 0, 60))
                        })
                        .style(move |s| {
                            s.with_shadcn_theme(move |s, t| {
                                s.background(t.background).border_1().border_color(t.border)
                            })
                        })
                        .on_event_stop(floem::event::listener::Click, move |_, ()| {
                            // Swallow clicks inside the dialog content.
                        }),
                )
                .on_event_stop(floem::event::listener::Click, move |_, ()| {
                    open.set(false);
                })
                .style(move |s| {
                    let is_open = open.get();
                    s.fixed()
                        .inset_0()
                        .w_full()
                        .h_full()
                        .z_index(1)
                        .background(floem::peniko::Color::from_rgba8(0, 0, 0, 128))
                        .apply_if(!is_open, floem::style::Style::hide)
                }),
            )
        } else {
            Box::new(
                floem::views::Stack::vertical_from_iter(children)
                    .style(floem_tailwind::TailwindExt::w_full),
            )
        }
    }
}

// ============================================================================
// DialogHeader
// ============================================================================

/// The `DialogHeader`.
pub struct DialogHeader {
    id: ViewId,
    title: Option<String>,
    description: Option<String>,
}

impl DialogHeader {
    #[must_use]
    /// Creates a new it.
    pub fn new() -> Self {
        Self {
            id: ViewId::new(),
            title: None,
            description: None,
        }
    }
    #[must_use]
    /// The title.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }
    #[must_use]
    /// The description.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }
}

impl Default for DialogHeader {
    fn default() -> Self {
        Self::new()
    }
}

impl HasViewId for DialogHeader {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for DialogHeader {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }

    fn into_view(self) -> Self::V {
        let mut children: Vec<Box<dyn View>> = Vec::new();
        if let Some(title) = self.title {
            children.push(Box::new(floem::views::Label::new(title).style(|s| {
                s.with_shadcn_theme(|s, t| {
                    s.text_lg()
                        .leading_none()
                        .font_semibold()
                        .color(t.foreground)
                })
            })));
        }
        if let Some(description) = self.description {
            children.push(Box::new(floem::views::Label::new(description).style(|s| {
                s.with_shadcn_theme(|s, t| s.text_sm().color(t.muted_foreground))
            })));
        }
        Box::new(
            floem::views::Stack::vertical_from_iter(children)
                .style(floem_tailwind::TailwindExt::gap_2),
        )
    }
}

// ============================================================================
// DialogFooter
// ============================================================================

/// The `DialogFooter`.
pub struct DialogFooter<V> {
    id: ViewId,
    child: V,
}

impl<V: IntoView + 'static> DialogFooter<V> {
    /// Creates a new it.
    pub fn new(child: V) -> Self {
        Self {
            id: ViewId::new(),
            child,
        }
    }
}

impl<V: IntoView + 'static> HasViewId for DialogFooter<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<V: IntoView + 'static> IntoView for DialogFooter<V> {
    type V = Box<dyn View>;

    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }

    fn into_view(self) -> Self::V {
        Box::new(
            floem::views::Container::with_id(self.id, self.child)
                .style(|s| s.flex().flex_row().justify_end().gap_2()),
        )
    }
}

// ============================================================================
// DialogTitle / DialogDescription (standalone)
// ============================================================================

/// The `DialogTitle`.
pub struct DialogTitle<V> {
    id: ViewId,
    child: V,
}

impl<V: IntoView + 'static> DialogTitle<V> {
    /// Creates a new it.
    pub fn new(child: V) -> Self {
        Self {
            id: ViewId::new(),
            child,
        }
    }
}

impl<V: IntoView + 'static> HasViewId for DialogTitle<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<V: IntoView + 'static> IntoView for DialogTitle<V> {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        Box::new(
            floem::views::Container::with_id(self.id, self.child)
                .style(|s| s.with_shadcn_theme(|s, t| s.text_lg().color(t.foreground))),
        )
    }
}

/// The `DialogDescription`.
pub struct DialogDescription<V> {
    id: ViewId,
    child: V,
}

impl<V: IntoView + 'static> DialogDescription<V> {
    /// Creates a new it.
    pub fn new(child: V) -> Self {
        Self {
            id: ViewId::new(),
            child,
        }
    }
}

impl<V: IntoView + 'static> HasViewId for DialogDescription<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<V: IntoView + 'static> IntoView for DialogDescription<V> {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        Box::new(
            floem::views::Container::with_id(self.id, self.child)
                .style(|s| s.with_shadcn_theme(|s, t| s.text_sm().color(t.muted_foreground))),
        )
    }
}
