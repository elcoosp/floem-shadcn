//! Rich-content accordion: items whose trigger and content are arbitrary
//! views.
//!
//! [`crate::components::accordion::AccordionItem`] (0.1.x) renders plain
//! `String` title/content pairs. Framework bindings that lower a declarative
//! element tree — `quoin`'s `quoin_render!`, for one — need items whose
//! trigger and content carry *any* Floem view: styled labels, rows of
//! controls, nested layouts. [`RichAccordionItem`] keeps the exact toggle
//! mechanics of the string item (click-to-toggle against a shared
//! `RwSignal<Option<String>>`, `Display::None` on the collapsed content,
//! chevron flip) while accepting boxed views.
//!
//! # Example
//!
//! ```
//! use floem::prelude::*;
//! use floem::reactive::RwSignal;
//! use floem_shadcn::components::accordion::Accordion;
//! use floem_shadcn::components::accordion_rich::RichAccordionItem;
//!
//! let expanded = RwSignal::new(Option::<String>::None);
//!
//! Accordion::new(expanded, (
//!     RichAccordionItem::new("item-1")
//!         .trigger(text("Is it composable?"))
//!         .content(text("Yes — trigger and content are plain views.")),
//!     RichAccordionItem::new("item-2")
//!         .trigger(text("Is it styled?"))
//!         .content(text("Yes — shadcn theme, same as the string items.")),
//! ));
//! ```

use floem::prelude::*;
use floem::reactive::{RwSignal, SignalGet, SignalUpdate};
use floem::style::CursorStyle;
use floem::views::Decorators;
use floem::{AnyView, HasViewId, ViewId};
use floem_tailwind::TailwindExt;

use crate::theme::ShadcnThemeExt;

/// One accordion section carrying arbitrary trigger/content views.
///
/// The shared expanded signal follows the string-based accordion contract:
/// `Some(value)` marks the single open section, `None` collapses everything.
/// Attach the *same* signal to every item of one accordion; the item reads
/// it for its chevron/visibility and writes it from its trigger click.
pub struct RichAccordionItem {
    view_id: ViewId,
    value: String,
    trigger: Option<AnyView>,
    content: Option<AnyView>,
    expanded_signal: Option<RwSignal<Option<String>>>,
}

impl RichAccordionItem {
    /// Creates an item for `value` with no views attached yet.
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            view_id: ViewId::new(),
            value: value.into(),
            trigger: None,
            content: None,
            expanded_signal: None,
        }
    }

    /// Sets the always-visible trigger row content.
    #[must_use]
    pub fn trigger(mut self, view: impl IntoView) -> Self {
        self.trigger = Some(view.into_any());
        self
    }

    /// Sets the collapsible content region.
    #[must_use]
    pub fn content(mut self, view: impl IntoView) -> Self {
        self.content = Some(view.into_any());
        self
    }

    /// Shares the accordion's expanded signal with this item.
    #[must_use]
    pub fn expanded(mut self, signal: RwSignal<Option<String>>) -> Self {
        self.expanded_signal = Some(signal);
        self
    }
}

impl HasViewId for RichAccordionItem {
    fn view_id(&self) -> ViewId {
        self.view_id
    }
}

impl IntoView for RichAccordionItem {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;

    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }

    fn into_view(self) -> Self::V {
        let value = self.value;
        let expanded_signal = self.expanded_signal;
        let value_chevron = value.clone();
        let value_click = value.clone();
        let value_content = value.clone();

        let trigger_body = self.trigger.unwrap_or_else(|| {
            floem::views::Label::new(value_chevron.clone()).into_any()
        });
        let content_body = self
            .content
            .unwrap_or_else(|| floem::views::Empty::new().into_any());

        let trigger = floem::views::Container::with_id(
            ViewId::new(),
            floem::views::Stack::horizontal((
                trigger_body,
                floem::views::svg(move || {
                    let is_expanded = expanded_signal
                        .is_some_and(|sig| sig.get() == Some(value_chevron.clone()));
                    if is_expanded {
                        CHEVRON_UP_SVG.to_string()
                    } else {
                        CHEVRON_DOWN_SVG.to_string()
                    }
                })
                .style(|s| s.with_shadcn_theme(move |s, t| s.size_4().color(t.muted_foreground))),
            ))
            .style(|s| s.w_full().items_center().gap_2()),
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
        });

        let content = floem::views::Container::with_id(ViewId::new(), content_body).style(
            move |s| {
                let value_content = value_content.clone();
                s.with_shadcn_theme(move |s, t| {
                    let is_expanded = expanded_signal
                        .is_some_and(|sig| sig.get() == Some(value_content.clone()));

                    s.w_full()
                        .p_4()
                        .pt_0()
                        .border_bottom(1.0)
                        .border_color(t.border)
                        .apply_if(!is_expanded, |s| s.display(floem::style::Display::None))
                })
            },
        );

        Box::new(
            floem::views::Stack::with_id(self.view_id, (trigger, content))
                .style(floem_tailwind::TailwindExt::w_full)
                .style(|s| s.flex_col()),
        )
    }
}

const CHEVRON_DOWN_SVG: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="6 9 12 15 18 9"></polyline></svg>"#;
const CHEVRON_UP_SVG: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="18 15 12 9 6 15"></polyline></svg>"#;

#[cfg(test)]
mod tests {
    use super::*;

    /// The builder chains must stay `#[must_use]`-clean and produce a view
    /// without panicking; layout behavior is covered by
    /// `tests/accordion_rich.rs`.
    #[test]
    fn builder_chain_builds_a_view() {
        let expanded = RwSignal::new(Option::<String>::None);
        let item = RichAccordionItem::new("item-1")
            .trigger(floem::views::Label::new("Title"))
            .content(floem::views::Label::new("Body"))
            .expanded(expanded);
        let _view = item.into_any();
        let _view_id = RichAccordionItem::new("item-2").view_id();
    }

    /// An item with neither trigger nor content still renders (defaults:
    /// the value as the trigger label, an empty content region).
    #[test]
    fn defaults_render_without_panicking() {
        let _view = RichAccordionItem::new("solo").into_any();
    }
}
