//! Sidebar component with builder-style API
//!
//! Based on shadcn/ui Sidebar component for navigation.
//! Active state is controlled via the `is_active` prop on `SidebarMenuButton`,
//! following the same pattern as shadcn/ui.
//!
//! # Example
//!
//! ```rust
//! use floem::reactive::RwSignal;
//! use floem::view::ParentView;
//! use floem_shadcn::components::sidebar::*;
//! # use floem::prelude::*;
//!
//! let active = RwSignal::new("buttons");
//!
//! let sidebar = Sidebar::new()
//!     .header(
//!         SidebarHeader::new()
//!             .child(label(|| "My App"))
//!     )
//!     .content(
//!         SidebarContent::new().child(
//!             SidebarGroup::new()
//!                 .child(SidebarGroupLabel::new("Components"))
//!                 .child(
//!                     SidebarGroupContent::new().child(
//!                         SidebarMenu::new()
//!                             .child(
//!                                 SidebarMenuItem::new().child(
//!                                     SidebarMenuButton::with_label("Buttons")
//!                                         .is_active(move || active.get() == "buttons")
//!                                         .on_event_stop(floem::event::listener::Click, move |_, _| active.set("buttons"))
//!                                 )
//!                             )
//!                             .child(
//!                                 SidebarMenuItem::new().child(
//!                                     SidebarMenuButton::with_label("Cards")
//!                                         .is_active(move || active.get() == "cards")
//!                                         .on_event_stop(floem::event::listener::Click, move |_, _| active.set("cards"))
//!                                 )
//!                             )
//!                     )
//!                 )
//!         )
//!     )
//!     .footer(
//!         SidebarFooter::new()
//!             .child(label(|| "v1.0.0"))
//!     );
//! ```

use std::rc::Rc;

use floem::prelude::*;
use floem::style::CursorStyle;
use floem::view::ParentView;
use floem::views::Scroll;
use floem::{HasViewId, ViewId};

use crate::theme::ShadcnThemeExt;
use floem_tailwind::TailwindExt;
// ============================================================================
// Sidebar - Main container
// ============================================================================

/// Sidebar container builder with ergonomic API.
///
/// # Example
/// ```rust
/// # use floem::prelude::*;
/// # use floem_shadcn::components::sidebar::*;
/// Sidebar::new()
///     .header(SidebarHeader::new().child(label(|| "My App")))
///     .content(SidebarContent::new().child(label(|| "menu items")))
///     .footer(SidebarFooter::new().child(label(|| "v1.0")));
/// ```
pub struct Sidebar {
    id: ViewId,
    header: Option<Box<dyn View>>,
    content: Option<Box<dyn View>>,
    footer: Option<Box<dyn View>>,
    width: f32,
}

impl Sidebar {
    #[must_use]
    /// Create a new empty sidebar builder
    pub fn new() -> Self {
        Self {
            id: ViewId::new(),
            header: None,
            content: None,
            footer: None,
            width: 240.0,
        }
    }

    #[must_use]
    /// Set the sidebar header
    pub fn header(mut self, header: impl IntoView + 'static) -> Self {
        self.header = Some(Box::new(header.into_view()));
        self
    }

    #[must_use]
    /// Set the sidebar content (scrollable area)
    pub fn content(mut self, content: impl IntoView + 'static) -> Self {
        self.content = Some(Box::new(content.into_view()));
        self
    }

    #[must_use]
    /// Set the sidebar footer
    pub fn footer(mut self, footer: impl IntoView + 'static) -> Self {
        self.footer = Some(Box::new(footer.into_view()));
        self
    }

    #[must_use]
    /// Set the sidebar width (default: 240px)
    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }
}

impl Default for Sidebar {
    fn default() -> Self {
        Self::new()
    }
}

impl HasViewId for Sidebar {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for Sidebar {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }

    fn into_view(self) -> Self::V {
        let width = self.width;
        let header = self.header;
        let content = self.content;
        let footer = self.footer;

        // Build view with optional header, content, footer
        let header_view: Box<dyn View> = header.unwrap_or_else(|| ().into_any());
        let content_view: Box<dyn View> = content.unwrap_or_else(|| ().into_any());
        let footer_view: Box<dyn View> = footer.unwrap_or_else(|| ().into_any());

        Box::new(
            Stack::vertical((header_view, content_view, footer_view)).style(move |s| {
                s.with_shadcn_theme(move |s, t| {
                    s.width(width)
                        .min_width(width)
                        .flex_shrink(0_f32)
                        .h_full()
                        .border_right(1.0)
                        .border_color(t.border)
                        .background(t.background)
                })
            }),
        )
    }
}

// ============================================================================
// SidebarHeader - Header section at top
// ============================================================================

/// Sidebar header component (for logo/title at top)
pub struct SidebarHeader {
    id: ViewId,
    children: Vec<floem::AnyView>,
}

impl SidebarHeader {
    #[must_use]
    /// Create a new sidebar header
    pub fn new() -> Self {
        Self {
            id: ViewId::new(),
            children: Vec::new(),
        }
    }
}

impl Default for SidebarHeader {
    fn default() -> Self {
        Self::new()
    }
}

impl HasViewId for SidebarHeader {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for SidebarHeader {
    type V = Container;
    type Intermediate = Container;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }

    fn into_view(self) -> Self::V {
        let container = {
            Container::with_id(self.id, ()).style(|s| {
                s.with_shadcn_theme(|s, t| {
                    s.w_full().p_4().border_bottom(1.0).border_color(t.border)
                })
            })
        };
        self.id.set_children_vec(self.children);
        container
    }
}

impl ParentView for SidebarHeader {
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
// SidebarContent - Main scrollable content area
// ============================================================================

/// Sidebar content area (scrollable).
pub struct SidebarContent {
    id: ViewId,
    children: Vec<floem::AnyView>,
}

impl SidebarContent {
    #[must_use]
    /// Create a new sidebar content area
    pub fn new() -> Self {
        Self {
            id: ViewId::new(),
            children: Vec::new(),
        }
    }
}

impl Default for SidebarContent {
    fn default() -> Self {
        Self::new()
    }
}

impl HasViewId for SidebarContent {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for SidebarContent {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }

    fn into_view(self) -> Self::V {
        let container = {
            let inner = Container::with_id(self.id, ()).style(|s| {
                s.w_full()
                    .flex_direction(floem::style::FlexDirection::Column)
                    .padding(8.0)
                    .gap_2()
            });

            Box::new(Scroll::new(inner).style(|s| {
                s.flex_grow(1.0_f32)
                    .flex_basis(0.0)
                    .min_height(0.0)
                    .w_full()
            }))
        };
        self.id.set_children_vec(self.children);
        container
    }
}

impl ParentView for SidebarContent {
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
// SidebarFooter - Footer section at bottom
// ============================================================================

/// Sidebar footer component
pub struct SidebarFooter {
    id: ViewId,
    children: Vec<floem::AnyView>,
}

impl SidebarFooter {
    #[must_use]
    /// Create a new sidebar footer
    pub fn new() -> Self {
        Self {
            id: ViewId::new(),
            children: Vec::new(),
        }
    }
}

impl Default for SidebarFooter {
    fn default() -> Self {
        Self::new()
    }
}

impl HasViewId for SidebarFooter {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for SidebarFooter {
    type V = Container;
    type Intermediate = Container;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }

    fn into_view(self) -> Self::V {
        let container = {
            Container::with_id(self.id, ()).style(|s| {
                s.with_shadcn_theme(|s, t| s.w_full().p_4().border_top(1.0).border_color(t.border))
            })
        };
        self.id.set_children_vec(self.children);
        container
    }
}

impl ParentView for SidebarFooter {
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
// SidebarGroup - Groups related menu items
// ============================================================================

/// Sidebar group container.
pub struct SidebarGroup {
    id: ViewId,
    children: Vec<floem::AnyView>,
}

impl SidebarGroup {
    #[must_use]
    /// Create a new sidebar group
    pub fn new() -> Self {
        Self {
            id: ViewId::new(),
            children: Vec::new(),
        }
    }
}

impl Default for SidebarGroup {
    fn default() -> Self {
        Self::new()
    }
}

impl HasViewId for SidebarGroup {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for SidebarGroup {
    type V = Container;
    type Intermediate = Container;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }

    fn into_view(self) -> Self::V {
        let container = {
            Container::with_id(self.id, ()).style(|s| {
                s.flex_direction(floem::style::FlexDirection::Column)
                    .w_full()
                    .gap_1()
            })
        };
        self.id.set_children_vec(self.children);
        container
    }
}

impl ParentView for SidebarGroup {
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
// SidebarGroupLabel - Label for a group
// ============================================================================

/// Sidebar group label
pub struct SidebarGroupLabel {
    id: ViewId,
    text: String,
}

impl SidebarGroupLabel {
    /// Create a new group label
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            id: ViewId::new(),
            text: text.into(),
        }
    }
}

impl HasViewId for SidebarGroupLabel {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for SidebarGroupLabel {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }

    fn into_view(self) -> Self::V {
        let text = self.text.to_uppercase();
        Box::new(
            floem::views::Label::derived(move || text.clone()).style(|s| {
                s.with_shadcn_theme(|s, t| {
                    s.font_size(11.0)
                        .color(t.muted_foreground)
                        .padding_left(8.0)
                        .padding_right(8.0)
                        .padding_top(8.0)
                        .padding_bottom(4.0)
                })
            }),
        )
    }
}

// ============================================================================
// SidebarGroupContent - Content container within a group
// ============================================================================

/// Sidebar group content container
pub struct SidebarGroupContent {
    id: ViewId,
    children: Vec<floem::AnyView>,
}

impl SidebarGroupContent {
    #[must_use]
    /// Create a new group content container
    pub fn new() -> Self {
        Self {
            id: ViewId::new(),
            children: Vec::new(),
        }
    }
}

impl Default for SidebarGroupContent {
    fn default() -> Self {
        Self::new()
    }
}

impl HasViewId for SidebarGroupContent {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for SidebarGroupContent {
    type V = Container;
    type Intermediate = Container;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }

    fn into_view(self) -> Self::V {
        let container = {
            Container::with_id(self.id, ()).style(|s| {
                s.flex_direction(floem::style::FlexDirection::Column)
                    .w_full()
            })
        };
        self.id.set_children_vec(self.children);
        container
    }
}

impl ParentView for SidebarGroupContent {
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
// SidebarGroupAction - Optional action button in group header
// ============================================================================

/// Sidebar group action button (appears next to group label)
pub struct SidebarGroupAction {
    id: ViewId,
    children: Vec<floem::AnyView>,
}

impl SidebarGroupAction {
    #[must_use]
    /// Create a new group action
    pub fn new() -> Self {
        Self {
            id: ViewId::new(),
            children: Vec::new(),
        }
    }
}

impl Default for SidebarGroupAction {
    fn default() -> Self {
        Self::new()
    }
}

impl HasViewId for SidebarGroupAction {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for SidebarGroupAction {
    type V = Container;
    type Intermediate = Container;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }

    fn into_view(self) -> Self::V {
        let container = {
            Container::with_id(self.id, ()).style(|s| {
                s.with_shadcn_theme(|s, t| {
                    s.p_1()
                        .border_radius(t.radius_sm)
                        .cursor(CursorStyle::Pointer)
                        .hover(|s| s.background(t.accent))
                })
            })
        };
        self.id.set_children_vec(self.children);
        container
    }
}

impl ParentView for SidebarGroupAction {
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
// SidebarMenu - Menu container
// ============================================================================

/// Sidebar menu container.
pub struct SidebarMenu {
    id: ViewId,
    children: Vec<floem::AnyView>,
}

impl SidebarMenu {
    #[must_use]
    /// Create a new sidebar menu
    pub fn new() -> Self {
        Self {
            id: ViewId::new(),
            children: Vec::new(),
        }
    }
}

impl Default for SidebarMenu {
    fn default() -> Self {
        Self::new()
    }
}

impl HasViewId for SidebarMenu {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for SidebarMenu {
    type V = Container;
    type Intermediate = Container;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }

    fn into_view(self) -> Self::V {
        let container = {
            Container::with_id(self.id, ()).style(|s| {
                s.flex_direction(floem::style::FlexDirection::Column)
                    .w_full()
                    .gap(2.0)
            })
        };
        self.id.set_children_vec(self.children);
        container
    }
}

impl ParentView for SidebarMenu {
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
// SidebarMenuItem - Menu item wrapper
// ============================================================================

/// Sidebar menu item wrapper
pub struct SidebarMenuItem {
    id: ViewId,
    children: Vec<floem::AnyView>,
}

impl SidebarMenuItem {
    #[must_use]
    /// Create a new menu item
    pub fn new() -> Self {
        Self {
            id: ViewId::new(),
            children: Vec::new(),
        }
    }
}

impl Default for SidebarMenuItem {
    fn default() -> Self {
        Self::new()
    }
}

impl HasViewId for SidebarMenuItem {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for SidebarMenuItem {
    type V = Container;
    type Intermediate = Container;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }

    fn into_view(self) -> Self::V {
        let container =
            { Container::with_id(self.id, ()).style(floem_tailwind::TailwindExt::w_full) };
        self.id.set_children_vec(self.children);
        container
    }
}

impl ParentView for SidebarMenuItem {
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
// SidebarMenuButton - Clickable menu button
// ============================================================================

/// Size variants for `SidebarMenuButton` (matches shadcn/ui)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum SidebarMenuButtonSize {
    /// Small size: 28px height
    Sm,
    /// Default size: 32px height
    #[default]
    Default,
    /// Large size: 48px height
    Lg,
}

impl SidebarMenuButtonSize {
    #[must_use]
    /// Get the height in pixels for this size variant
    pub fn height(&self) -> f32 {
        match self {
            SidebarMenuButtonSize::Sm => 28.0,
            SidebarMenuButtonSize::Default => 32.0,
            SidebarMenuButtonSize::Lg => 48.0,
        }
    }
}

/// Sidebar menu button (the actual clickable item)
///
/// Active state is controlled via the `is_active` prop, following the same pattern
/// as shadcn/ui. This gives full control to the consumer.
///
/// Supports both simple labels and custom children (like icons + text):
///
/// # Example with label
/// ```rust
/// # use floem::prelude::*;
/// # use floem_shadcn::components::sidebar::*;
/// let active = floem::reactive::RwSignal::new("buttons");
///
/// SidebarMenuButton::with_label("Buttons")
///     .is_active(move || active.get() == "buttons")
///     .on_event_stop(floem::event::listener::Click, move |_, _| active.set("buttons"));
/// ```
///
/// # Example with children
/// ```rust
/// # use floem::prelude::*;
/// # use floem_shadcn::components::sidebar::*;
/// let active = floem::reactive::RwSignal::new("home");
///
/// SidebarMenuButton::new_empty()
///     .child(floem::views::text("Home"))
///     .is_active(move || active.get() == "home")
///     .on_event_stop(floem::event::listener::Click, move |_, _| active.set("home"));
/// ```
pub struct SidebarMenuButton {
    id: ViewId,
    children: Vec<floem::AnyView>,
    is_active: Rc<Option<Box<dyn Fn() -> bool>>>,
    size: SidebarMenuButtonSize,
}

impl SidebarMenuButton {
    /// Create a new menu button with a label.
    pub fn with_label(label: impl Into<String>) -> SidebarMenuButtonWithLabel {
        SidebarMenuButtonWithLabel {
            id: ViewId::new(),
            label: label.into(),
            is_active: None,
            size: SidebarMenuButtonSize::Default,
        }
    }

    #[must_use]
    /// Create a new empty menu button that accepts children via `.child()`.
    pub fn new_empty() -> Self {
        Self {
            id: ViewId::new(),
            is_active: Rc::new(None),
            size: SidebarMenuButtonSize::Default,

            children: Vec::new(),
        }
    }

    #[must_use]
    /// Set whether this button is active (reactive closure).
    pub fn is_active(mut self, active: impl Fn() -> bool + 'static) -> Self {
        self.is_active = Rc::new(Some(Box::new(active)));
        self
    }

    #[must_use]
    /// Set the size variant for this button.
    pub fn size(mut self, size: SidebarMenuButtonSize) -> Self {
        self.size = size;
        self
    }
}

impl HasViewId for SidebarMenuButton {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for SidebarMenuButton {
    type V = Container;
    type Intermediate = Container;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }

    fn into_view(self) -> Self::V {
        let container = {
            let is_active = self.is_active.clone();
            let height = self.size.height();
            Container::with_id(self.id, ()).style(move |s| {
                let is_active = is_active.clone();
                s.with_shadcn_theme(move |s, t| {
                    let active = is_active.as_ref().as_ref().is_some_and(|f| f());
                    let base = s
                        .w_full()
                        .height(height)
                        .flex_direction(floem::style::FlexDirection::Row)
                        .items_center()
                        .gap_2()
                        .padding_left(12.0)
                        .padding_right(12.0)
                        .border_radius(t.radius_sm)
                        .text_sm()
                        .cursor(CursorStyle::Pointer)
                        .transition(
                            floem::style::Background,
                            floem::style::Transition::linear(millis(100)),
                        )
                        .hover(move |s| s.background(t.accent));
                    if active {
                        base.background(t.accent).color(t.accent_foreground)
                    } else {
                        base.background(floem::peniko::Color::TRANSPARENT)
                            .color(t.foreground)
                    }
                })
            })
        };
        self.id.set_children_vec(self.children);
        container
    }
}

impl ParentView for SidebarMenuButton {
    fn child(mut self, child: impl IntoView + 'static) -> Self {
        self.children.push(child.into_any());
        self
    }
    fn children(mut self, children: impl IntoViewIter + 'static) -> Self {
        self.children.extend(children.into_view_iter());
        self
    }
}

/// Sidebar menu button with a simple label (for backward compatibility)
pub struct SidebarMenuButtonWithLabel {
    id: ViewId,
    label: String,
    is_active: Option<Box<dyn Fn() -> bool>>,
    size: SidebarMenuButtonSize,
}

impl SidebarMenuButtonWithLabel {
    #[must_use]
    /// Set whether this button is active (reactive closure).
    pub fn is_active(mut self, active: impl Fn() -> bool + 'static) -> Self {
        self.is_active = Some(Box::new(active));
        self
    }

    #[must_use]
    /// Set the size variant for this button.
    pub fn size(mut self, size: SidebarMenuButtonSize) -> Self {
        self.size = size;
        self
    }
}

impl HasViewId for SidebarMenuButtonWithLabel {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for SidebarMenuButtonWithLabel {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }

    fn into_view(self) -> Self::V {
        let label = self.label.clone();
        let is_active: Rc<Option<Box<dyn Fn() -> bool>>> = Rc::new(self.is_active);
        let height = self.size.height();

        Box::new(
            floem::views::Container::with_id(
                self.id,
                floem::views::Label::derived(move || label.clone()),
            )
            .style(move |s| {
                let is_active = is_active.clone();
                s.with_shadcn_theme(move |s, t| {
                    let active = is_active.as_ref().as_ref().is_some_and(|f| f());
                    let base = s
                        .w_full()
                        .height(height)
                        .items_center()
                        .padding_left(12.0)
                        .padding_right(12.0)
                        .border_radius(t.radius_sm)
                        .text_sm()
                        .cursor(CursorStyle::Pointer)
                        .transition(
                            floem::style::Background,
                            floem::style::Transition::linear(millis(100)),
                        )
                        .hover(move |s| s.background(t.accent));
                    if active {
                        base.background(t.accent).color(t.accent_foreground)
                    } else {
                        base.background(floem::peniko::Color::TRANSPARENT)
                            .color(t.foreground)
                    }
                })
            }),
        )
    }
}

// ============================================================================
// SidebarSeparator - Visual divider
// ============================================================================

/// Sidebar separator (horizontal line)
pub struct SidebarSeparator;

impl SidebarSeparator {
    #[must_use]
    /// Create a new separator
    pub fn new() -> Self {
        Self
    }
}

impl Default for SidebarSeparator {
    fn default() -> Self {
        Self::new()
    }
}

impl HasViewId for SidebarSeparator {
    fn view_id(&self) -> ViewId {
        ViewId::new()
    }
}

impl IntoView for SidebarSeparator {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }

    fn into_view(self) -> Self::V {
        Box::new(floem::views::Empty::new().style(|s| {
            s.with_shadcn_theme(|s, t| {
                s.w_full()
                    .height(1.0)
                    .margin_top(8.0)
                    .margin_bottom(8.0)
                    .background(t.border)
            })
        }))
    }
}

// ============================================================================
// Helper
// ============================================================================

fn millis(ms: u64) -> std::time::Duration {
    std::time::Duration::from_millis(ms)
}

// ============================================================================
// SidebarMenuBadge - badge shown next to a menu button
// ============================================================================

/// Small badge rendered next to a sidebar menu button
pub struct SidebarMenuBadge {
    view_id: ViewId,
    text: String,
}

impl SidebarMenuBadge {
    /// Create a new menu badge with the given text
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            view_id: ViewId::new(),
            text: text.into(),
        }
    }
}

impl HasViewId for SidebarMenuBadge {
    fn view_id(&self) -> ViewId {
        self.view_id
    }
}

impl IntoView for SidebarMenuBadge {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }

    fn into_view(self) -> Self::V {
        let text = self.text;
        Box::new(
            floem::views::Container::with_id(self.view_id, floem::views::Label::new(text)).style(
                |s| {
                    s.with_shadcn_theme(|s, t| {
                        s.text_xs()
                            .font_medium()
                            .color(t.muted_foreground)
                            .padding_left(4.0)
                            .padding_right(4.0)
                    })
                },
            ),
        )
    }
}

// ============================================================================
// SidebarMenuAction - small action button on a menu row
// ============================================================================

/// Small icon-sized action button displayed on a sidebar menu row
pub struct SidebarMenuAction {
    view_id: ViewId,
    children: Vec<floem::AnyView>,
}

impl SidebarMenuAction {
    #[must_use]
    /// Create a new menu action button
    pub fn new() -> Self {
        Self {
            view_id: ViewId::new(),

            children: Vec::new(),
        }
    }
}

impl Default for SidebarMenuAction {
    fn default() -> Self {
        Self::new()
    }
}

impl HasViewId for SidebarMenuAction {
    fn view_id(&self) -> ViewId {
        self.view_id
    }
}

impl ParentView for SidebarMenuAction {
    fn child(mut self, child: impl IntoView + 'static) -> Self {
        self.children.push(child.into_any());
        self
    }
    fn children(mut self, children: impl IntoViewIter + 'static) -> Self {
        self.children.extend(children.into_view_iter());
        self
    }
}

impl IntoView for SidebarMenuAction {
    type V = Container;
    type Intermediate = Container;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }

    fn into_view(self) -> Self::V {
        let container = {
            Container::with_id(self.view_id, ()).style(|s| {
                s.with_shadcn_theme(|s, t| {
                    s.size_7()
                        .items_center()
                        .justify_center()
                        .border_radius(t.radius_sm)
                        .cursor(CursorStyle::Pointer)
                        .color(t.muted_foreground)
                        .hover(|s| s.background(t.accent))
                })
            })
        };
        self.view_id.set_children_vec(self.children);
        container
    }
}

// ============================================================================
// SidebarMenuSkeleton - loading placeholder for menu rows
// ============================================================================

/// Loading placeholder row for a sidebar menu
pub struct SidebarMenuSkeleton {
    view_id: ViewId,
}

impl SidebarMenuSkeleton {
    #[must_use]
    /// Create a new menu skeleton row
    pub fn new() -> Self {
        Self {
            view_id: ViewId::new(),
        }
    }
}

impl Default for SidebarMenuSkeleton {
    fn default() -> Self {
        Self::new()
    }
}

impl HasViewId for SidebarMenuSkeleton {
    fn view_id(&self) -> ViewId {
        self.view_id
    }
}

impl IntoView for SidebarMenuSkeleton {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }

    fn into_view(self) -> Self::V {
        Box::new(
            floem::views::Container::with_id(self.view_id, ()).style(|s| {
                s.with_shadcn_theme(|s, t| {
                    s.w_full()
                        .height(20.0)
                        .border_radius(t.radius_sm)
                        .background(t.muted)
                })
            }),
        )
    }
}

// ============================================================================
// SidebarMenuSub / SubItem / SubButton - nested navigation
// ============================================================================

/// Nested sub-menu container rendered below a menu button
pub struct SidebarMenuSub {
    view_id: ViewId,
    children: Vec<floem::AnyView>,
}

impl SidebarMenuSub {
    #[must_use]
    /// Create a new sub-menu container
    pub fn new() -> Self {
        Self {
            view_id: ViewId::new(),

            children: Vec::new(),
        }
    }
}

impl Default for SidebarMenuSub {
    fn default() -> Self {
        Self::new()
    }
}

impl HasViewId for SidebarMenuSub {
    fn view_id(&self) -> ViewId {
        self.view_id
    }
}

impl ParentView for SidebarMenuSub {
    fn child(mut self, child: impl IntoView + 'static) -> Self {
        self.children.push(child.into_any());
        self
    }
    fn children(mut self, children: impl IntoViewIter + 'static) -> Self {
        self.children.extend(children.into_view_iter());
        self
    }
}

impl IntoView for SidebarMenuSub {
    type V = Container;
    type Intermediate = Container;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }

    fn into_view(self) -> Self::V {
        let container = {
            Container::with_id(self.view_id, ()).style(|s| {
                s.with_shadcn_theme(|s, t| {
                    s.flex_direction(floem::style::FlexDirection::Column)
                        .gap_0p5()
                        .margin_left(14.0)
                        .padding_left(10.0)
                        .padding_top(2.0)
                        .padding_bottom(2.0)
                        .border_left(1.0)
                        .border_color(t.border)
                        .w_full()
                })
            })
        };
        self.view_id.set_children_vec(self.children);
        container
    }
}

/// Wrapper for a single entry inside a [`SidebarMenuSub`]
pub struct SidebarMenuSubItem {
    view_id: ViewId,
    children: Vec<floem::AnyView>,
}

impl SidebarMenuSubItem {
    #[must_use]
    /// Create a new sub-menu item wrapper
    pub fn new() -> Self {
        Self {
            view_id: ViewId::new(),

            children: Vec::new(),
        }
    }
}

impl Default for SidebarMenuSubItem {
    fn default() -> Self {
        Self::new()
    }
}

impl HasViewId for SidebarMenuSubItem {
    fn view_id(&self) -> ViewId {
        self.view_id
    }
}

impl ParentView for SidebarMenuSubItem {
    fn child(mut self, child: impl IntoView + 'static) -> Self {
        self.children.push(child.into_any());
        self
    }
    fn children(mut self, children: impl IntoViewIter + 'static) -> Self {
        self.children.extend(children.into_view_iter());
        self
    }
}

impl IntoView for SidebarMenuSubItem {
    type V = Container;
    type Intermediate = Container;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }

    fn into_view(self) -> Self::V {
        let container =
            { Container::with_id(self.view_id, ()).style(floem_tailwind::TailwindExt::w_full) };
        self.view_id.set_children_vec(self.children);
        container
    }
}

/// Compact clickable entry used inside a [`SidebarMenuSub`]
pub struct SidebarMenuSubButton {
    view_id: ViewId,
    children: Vec<floem::AnyView>,
    is_active: Rc<Option<Box<dyn Fn() -> bool>>>,
}

impl SidebarMenuSubButton {
    #[must_use]
    /// Create a new sub-menu button
    pub fn new() -> Self {
        Self {
            view_id: ViewId::new(),
            is_active: Rc::new(None),

            children: Vec::new(),
        }
    }

    #[must_use]
    /// Set whether this button is active (reactive closure).
    pub fn is_active(mut self, active: impl Fn() -> bool + 'static) -> Self {
        self.is_active = Rc::new(Some(Box::new(active)));
        self
    }
}

impl Default for SidebarMenuSubButton {
    fn default() -> Self {
        Self::new()
    }
}

impl HasViewId for SidebarMenuSubButton {
    fn view_id(&self) -> ViewId {
        self.view_id
    }
}

impl ParentView for SidebarMenuSubButton {
    fn child(mut self, child: impl IntoView + 'static) -> Self {
        self.children.push(child.into_any());
        self
    }
    fn children(mut self, children: impl IntoViewIter + 'static) -> Self {
        self.children.extend(children.into_view_iter());
        self
    }
}

impl IntoView for SidebarMenuSubButton {
    type V = Container;
    type Intermediate = Container;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }

    fn into_view(self) -> Self::V {
        let container = {
            let is_active = self.is_active.clone();
            Container::with_id(self.view_id, ()).style(move |s| {
                let is_active = is_active.clone();
                s.with_shadcn_theme(move |s, t| {
                    let active = is_active.as_ref().as_ref().is_some_and(|f| f());
                    let base = s
                        .w_full()
                        .height(28.0)
                        .items_center()
                        .padding_left(8.0)
                        .padding_right(8.0)
                        .border_radius(t.radius_sm)
                        .text_xs()
                        .cursor(CursorStyle::Pointer)
                        .transition(
                            floem::style::Background,
                            floem::style::Transition::linear(millis(100)),
                        )
                        .hover(move |s| s.background(t.accent));
                    if active {
                        base.background(t.accent).color(t.accent_foreground)
                    } else {
                        base.background(floem::peniko::Color::TRANSPARENT)
                            .color(t.foreground)
                    }
                })
            })
        };
        self.view_id.set_children_vec(self.children);
        container
    }
}

// ============================================================================
// SidebarTrigger - toggle button that flips the open signal
// ============================================================================

/// Button that toggles the sidebar's open state
pub struct SidebarTrigger {
    view_id: ViewId,
    open: RwSignal<bool>,
}

impl SidebarTrigger {
    #[must_use]
    /// Create a new trigger bound to the sidebar open signal
    pub fn new(open: RwSignal<bool>) -> Self {
        Self {
            view_id: ViewId::new(),
            open,
        }
    }
}

impl HasViewId for SidebarTrigger {
    fn view_id(&self) -> ViewId {
        self.view_id
    }
}

impl IntoView for SidebarTrigger {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }

    fn into_view(self) -> Self::V {
        let open = self.open;
        Box::new(
            floem::views::Container::with_id(
                self.view_id,
                floem::views::svg(move || PANEL_LEFT_SVG.to_string()),
            )
            .style(|s| {
                s.with_shadcn_theme(|s, t| {
                    s.size_7()
                        .items_center()
                        .justify_center()
                        .border_radius(t.radius_sm)
                        .cursor(CursorStyle::Pointer)
                        .color(t.foreground)
                        .hover(|s| s.background(t.accent))
                })
            })
            .on_event_stop(floem::event::listener::Click, move |_, ()| {
                open.update(|v| *v = !*v);
            }),
        )
    }
}

// ============================================================================
// SidebarRail - thin clickable strip on the sidebar edge
// ============================================================================

/// Thin strip along the sidebar edge that toggles the open state on click
pub struct SidebarRail {
    view_id: ViewId,
    open: RwSignal<bool>,
}

impl SidebarRail {
    #[must_use]
    /// Create a new rail bound to the sidebar open signal
    pub fn new(open: RwSignal<bool>) -> Self {
        Self {
            view_id: ViewId::new(),
            open,
        }
    }
}

impl HasViewId for SidebarRail {
    fn view_id(&self) -> ViewId {
        self.view_id
    }
}

impl IntoView for SidebarRail {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }

    fn into_view(self) -> Self::V {
        let open = self.open;
        // Attach the pre-created ViewId via Container so `view_id()` queries
        // resolve against the rendered view, not an orphan node.
        Box::new(
            floem::views::Container::with_id(self.view_id, floem::views::Empty::new())
                .style(move |s| {
                    s.with_shadcn_theme(move |s, t| {
                        s.w_px()
                            .h_full()
                            .cursor(CursorStyle::Pointer)
                            .background(t.border)
                            .hover(|s| s.w_2().background(t.muted_foreground))
                            .apply_if(!open.get(), floem_tailwind::TailwindExt::w_0)
                    })
                })
                .on_event_stop(floem::event::listener::Click, move |_, ()| {
                    open.update(|v| *v = !*v);
                }),
        )
    }
}

// ============================================================================
// SidebarInset - main content area beside the sidebar
// ============================================================================

/// Main content area rendered beside the sidebar
pub struct SidebarInset {
    view_id: ViewId,
    children: Vec<floem::AnyView>,
}

impl SidebarInset {
    #[must_use]
    /// Create a new sidebar inset content area
    pub fn new() -> Self {
        Self {
            view_id: ViewId::new(),

            children: Vec::new(),
        }
    }
}

impl Default for SidebarInset {
    fn default() -> Self {
        Self::new()
    }
}

impl HasViewId for SidebarInset {
    fn view_id(&self) -> ViewId {
        self.view_id
    }
}

impl ParentView for SidebarInset {
    fn child(mut self, child: impl IntoView + 'static) -> Self {
        self.children.push(child.into_any());
        self
    }
    fn children(mut self, children: impl IntoViewIter + 'static) -> Self {
        self.children.extend(children.into_view_iter());
        self
    }
}

impl IntoView for SidebarInset {
    type V = Container;
    type Intermediate = Container;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }

    fn into_view(self) -> Self::V {
        let container = {
            Container::with_id(self.view_id, ()).style(|s| {
                s.with_shadcn_theme(|s, t| {
                    s.flex_grow(1.0_f32)
                        .flex_direction(floem::style::FlexDirection::Column)
                        .background(t.background)
                        .border_radius(t.radius_lg)
                })
            })
        };
        self.view_id.set_children_vec(self.children);
        container
    }
}

/// Panel-left toggle icon used by [`SidebarTrigger`]
const PANEL_LEFT_SVG: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect width="18" height="18" x="3" y="3" rx="2"/><path d="M9 3v18"/></svg>"#;
