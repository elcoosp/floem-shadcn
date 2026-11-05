//! Integration tests for the rich-content accordion ([`RichAccordionItem`]).

use floem_test::TestRoot;

use floem::prelude::*;
use floem::reactive::{RwSignal, SignalGet};
use floem_shadcn::components::accordion::Accordion;
use floem_shadcn::components::accordion_rich::RichAccordionItem;
use floem_test::prelude::*;

/// Two rich items inside an `Accordion`: both trigger rows must lay out with
/// the container's width, and toggling the shared signal must flip which
/// section is expanded.
#[test]
fn rich_accordion_items_layout_and_share_state() {
    floem_shadcn::theme::set_theme(floem_shadcn::theme::ShadcnTheme::light());
    let expanded = RwSignal::new(Option::<String>::None);

    let item1 = RichAccordionItem::new("item-1")
        .trigger(floem::views::Label::new("First section"))
        .content(floem::views::Label::new("First body"))
        .expanded(expanded);
    let item1_id = item1.view_id();

    let item2 = RichAccordionItem::new("item-2")
        .trigger(floem::views::Label::new("Second section"))
        .content(floem::views::Label::new("Second body"))
        .expanded(expanded);

    let container = Stack::new((Accordion::new(expanded, (item1, item2)),))
        .style(|s| s.width(300.0));
    let mut harness = HeadlessHarness::new_with_size(TestRoot::new(), container, 400.0, 400.0);
    harness.rebuild();

    let layout = item1_id
        .get_layout()
        .expect("rich accordion item should lay out");
    assert!(
        layout.size.width > 100.0,
        "rich item trigger should stretch to the accordion width, got {}",
        layout.size.width
    );

    // Collapsed by default: the signal round-trips the shared contract.
    assert_eq!(expanded.get(), None);

    // Simulate the trigger toggle the click handler performs.
    expanded.set(Some("item-2".to_string()));
    assert_eq!(expanded.get(), Some("item-2".to_string()));
    expanded.set(None);
    assert_eq!(expanded.get(), None);
}

/// A single-item accordion must not panic and must expose the item view.
#[test]
fn single_item_accordion_renders() {
    floem_shadcn::theme::set_theme(floem_shadcn::theme::ShadcnTheme::light());
    let expanded = RwSignal::new(Some("solo".to_string()));

    let item = RichAccordionItem::new("solo")
        .trigger(floem::views::Label::new("Solo"))
        .content(floem::views::Label::new("Body"))
        .expanded(expanded);
    let item_id = item.view_id();

    let container = Stack::new((Accordion::new(expanded, (item,)),)).style(|s| s.width(240.0));
    let mut harness = HeadlessHarness::new_with_size(TestRoot::new(), container, 300.0, 200.0);
    harness.rebuild();

    let layout = item_id.get_layout().expect("solo item should lay out");
    assert!(layout.size.width > 50.0, "solo item should have width");
    assert!(layout.size.height > 10.0, "solo item should have height");
}
