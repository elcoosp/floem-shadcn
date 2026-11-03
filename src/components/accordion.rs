//! Accordion component with builder-style API
//!
//! Based on shadcn/ui Accordion - collapsible content sections.
//!
//! # Example
//!
//! ```
//! use floem::reactive::RwSignal;
//! use floem_shadcn::components::accordion::{Accordion, AccordionItem};
//!
//! let expanded = RwSignal::new(Some("item-1".to_string()));
//!
//! Accordion::new(expanded, (
//!     AccordionItem::new("item-1", "Is it accessible?", "Yes. It adheres to the WAI-ARIA design pattern."),
//!     AccordionItem::new("item-2", "Is it styled?", "Yes. It comes with default styles."),
//! ));
//! ```

use floem::prelude::*;
use floem::reactive::{RwSignal, SignalGet, SignalUpdate};
use floem::style::CursorStyle;
use floem::views::Decorators;
use floem::{HasViewId, ViewId};
use floem_tailwind::TailwindExt;

use crate::theme::ShadcnThemeExt;

// ============================================================================
// Accordion
// ============================================================================

/// The Accordion.
pub struct Accordion<V> {
    id: ViewId,
    expanded: RwSignal<Option<String>>,
    child: V,
}

impl<V: IntoView + 'static> Accordion<V> {
    /// Creates a new it.
    pub fn new(expanded: RwSignal<Option<String>>, child: V) -> Self {
        Self {
            id: ViewId::new(),
            expanded,
            child,
        }
    }
}

impl<V: IntoView + 'static> Accordion<V> {
    /// The shared expanded signal, to hand to each [`AccordionItem::expanded`].
    pub fn expanded_signal(&self) -> RwSignal<Option<String>> {
        self.expanded
    }
}

impl<V: IntoView + 'static> HasViewId for Accordion<V> {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl<V: IntoView + 'static> IntoView for Accordion<V> {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        Box::new(
            floem::views::Container::with_id(self.id, self.child).style(|s| s.w_full().flex_col()),
        )
    }
}

// ============================================================================
// AccordionItem
// ============================================================================

/// The `AccordionItem`.
pub struct AccordionItem {
    view_id: ViewId,
    id: String,
    title: String,
    content: String,
    expanded_signal: Option<RwSignal<Option<String>>>,
}

impl AccordionItem {
    /// Creates a new it.
    pub fn new(
        id: impl Into<String>,
        title: impl Into<String>,
        content: impl Into<String>,
    ) -> Self {
        Self {
            view_id: ViewId::new(),
            id: id.into(),
            title: title.into(),
            content: content.into(),
            expanded_signal: None,
        }
    }

    #[must_use]
    /// The expanded.
    pub fn expanded(mut self, signal: RwSignal<Option<String>>) -> Self {
        self.expanded_signal = Some(signal);
        self
    }

    #[must_use]
    /// Builds it.
    pub fn build(self) -> impl IntoView {
        let id = self.id.clone();
        let title = self.title.clone();
        let content = self.content.clone();
        let expanded_signal = self.expanded_signal;
        let item_id = id.clone();
        let item_id_click = id.clone();
        let item_id_content = id.clone();

        let trigger = floem::views::Container::new(
            floem::views::Stack::horizontal((
                floem::views::Label::new(title).style(|s| {
                    s.with_shadcn_theme(|s, t| {
                        s.text_sm()
                            .font_medium()
                            .color(t.foreground)
                            .flex_grow(1.0_f32)
                    })
                }),
                floem::views::svg(move || {
                    let is_expanded =
                        expanded_signal.is_some_and(|sig| sig.get() == Some(item_id.clone()));
                    if is_expanded {
                        CHEVRON_UP_SVG.to_string()
                    } else {
                        CHEVRON_DOWN_SVG.to_string()
                    }
                })
                .style(|s| s.with_shadcn_theme(move |s, t| s.size_4().color(t.muted_foreground))),
            ))
            .style(|s| s.w_full().items_center()),
        )
        .style(|s| {
            s.with_shadcn_theme(|s, t| {
                s.w_full()
                    .p_4()
                    .cursor(CursorStyle::Pointer)
                    .border_bottom(1.0)
                    .border_color(t.border)
                    .hover(|s| s.background(t.muted))
            })
        })
        .on_event_stop(floem::event::listener::Click, move |_, ()| {
            if let Some(signal) = expanded_signal {
                signal.update(|current| {
                    if *current == Some(item_id_click.clone()) {
                        *current = None;
                    } else {
                        *current = Some(item_id_click.clone());
                    }
                });
            }
        });

        let content_view =
            floem::views::Container::new(floem::views::Label::new(content).style(|s| {
                s.with_shadcn_theme(|s, t| s.text_sm().color(t.muted_foreground).leading_normal())
            }))
            .style(move |s| {
                let item_id = item_id_content.clone();
                s.with_shadcn_theme(move |s, t| {
                    let is_expanded =
                        expanded_signal.is_some_and(|sig| sig.get() == Some(item_id.clone()));

                    s.w_full()
                        .p_4()
                        .pt_0()
                        .border_bottom(1.0)
                        .border_color(t.border)
                        .apply_if(!is_expanded, |s| s.display(floem::style::Display::None))
                })
            });

        floem::views::Stack::vertical((trigger, content_view))
            .style(floem_tailwind::TailwindExt::w_full)
    }
}

impl HasViewId for AccordionItem {
    fn view_id(&self) -> ViewId {
        self.view_id
    }
}

impl IntoView for AccordionItem {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        Box::new(self.build().into_view())
    }
}

const CHEVRON_DOWN_SVG: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="6 9 12 15 18 9"></polyline></svg>"#;
const CHEVRON_UP_SVG: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="18 15 12 9 6 15"></polyline></svg>"#;

// ============================================================================
// AccordionTrigger - standalone trigger for custom accordion items
// ============================================================================

/// Standalone accordion trigger row (title + chevron) that toggles the
/// shared expanded signal for `value`.
pub struct AccordionTrigger {
    view_id: ViewId,
    value: String,
    title: String,
    expanded_signal: Option<RwSignal<Option<String>>>,
}

impl AccordionTrigger {
    /// Create a new trigger bound to `value` with a plain-text title
    pub fn new(
        value: impl Into<String>,
        title: impl Into<String>,
        expanded: RwSignal<Option<String>>,
    ) -> Self {
        Self {
            view_id: ViewId::new(),
            value: value.into(),
            title: title.into(),
            expanded_signal: Some(expanded),
        }
    }

    #[must_use]
    /// Override the expanded signal after construction
    pub fn expanded(mut self, signal: RwSignal<Option<String>>) -> Self {
        self.expanded_signal = Some(signal);
        self
    }

    #[must_use]
    /// Build the trigger view
    pub fn build(self) -> impl IntoView {
        let value = self.value.clone();
        let title = self.title;
        let expanded_signal = self.expanded_signal;
        let value_click = value.clone();
        let value_chevron = value.clone();

        floem::views::Container::with_id(
            self.view_id,
            floem::views::Stack::horizontal((
                floem::views::Label::new(title).style(|s| {
                    s.with_shadcn_theme(|s, t| {
                        s.text_sm()
                            .font_medium()
                            .color(t.foreground)
                            .flex_grow(1.0_f32)
                    })
                }),
                floem::views::svg(move || {
                    let is_expanded =
                        expanded_signal.is_some_and(|sig| sig.get() == Some(value_chevron.clone()));
                    if is_expanded {
                        CHEVRON_UP_SVG.to_string()
                    } else {
                        CHEVRON_DOWN_SVG.to_string()
                    }
                })
                .style(|s| s.with_shadcn_theme(move |s, t| s.size_4().color(t.muted_foreground))),
            ))
            .style(|s| s.w_full().items_center()),
        )
        .style(|s| {
            s.with_shadcn_theme(|s, t| {
                s.w_full()
                    .p_4()
                    .cursor(CursorStyle::Pointer)
                    .border_bottom(1.0)
                    .border_color(t.border)
                    .hover(|s| s.background(t.muted))
            })
        })
        .on_event_stop(floem::event::listener::Click, move |_, ()| {
            if let Some(signal) = expanded_signal {
                signal.update(|current| {
                    if *current == Some(value_click.clone()) {
                        *current = None;
                    } else {
                        *current = Some(value_click.clone());
                    }
                });
            }
        })
    }
}

impl HasViewId for AccordionTrigger {
    fn view_id(&self) -> ViewId {
        self.view_id
    }
}

impl IntoView for AccordionTrigger {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        Box::new(self.build().into_view())
    }
}

// ============================================================================
// AccordionContent - standalone collapsible content region
// ============================================================================

/// Standalone accordion content region, visible only while `value` is
/// the active entry in the shared expanded signal.
pub struct AccordionContent {
    view_id: ViewId,
    value: String,
    text: String,
    expanded_signal: Option<RwSignal<Option<String>>>,
}

impl AccordionContent {
    /// Create a new content region bound to `value` with plain text
    pub fn new(
        value: impl Into<String>,
        text: impl Into<String>,
        expanded: RwSignal<Option<String>>,
    ) -> Self {
        Self {
            view_id: ViewId::new(),
            value: value.into(),
            text: text.into(),
            expanded_signal: Some(expanded),
        }
    }

    #[must_use]
    /// Override the expanded signal after construction
    pub fn expanded(mut self, signal: RwSignal<Option<String>>) -> Self {
        self.expanded_signal = Some(signal);
        self
    }
}

impl HasViewId for AccordionContent {
    fn view_id(&self) -> ViewId {
        self.view_id
    }
}

impl IntoView for AccordionContent {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        let value = self.value;
        let text = self.text;
        let expanded_signal = self.expanded_signal;
        Box::new(
            floem::views::Container::with_id(
                self.view_id,
                floem::views::Label::new(text).style(|s| {
                    s.with_shadcn_theme(|s, t| {
                        s.text_sm().color(t.muted_foreground).leading_normal()
                    })
                }),
            )
            .style(move |s| {
                let value = value.clone();
                s.with_shadcn_theme(move |s, t| {
                    let is_expanded =
                        expanded_signal.is_some_and(|sig| sig.get() == Some(value.clone()));

                    s.w_full()
                        .p_4()
                        .pt_0()
                        .border_bottom(1.0)
                        .border_color(t.border)
                        .apply_if(!is_expanded, |s| s.display(floem::style::Display::None))
                })
            }),
        )
    }
}
