//! Integration tests for the floem-shadcn component library.
use floem_test::TestRoot;
// Focused tests for canonical shadcn/ui sub-parts added across components

use floem::prelude::*;
use floem::reactive::RwSignal;
use floem::view::ParentView;
use floem_test::prelude::*;

use floem_shadcn::components::accordion::*;
use floem_shadcn::components::button_group::*;
use floem_shadcn::components::card::*;
use floem_shadcn::components::empty::*;
use floem_shadcn::components::item::*;
use floem_shadcn::components::popover::*;
use floem_shadcn::components::sidebar::*;

#[test]
fn test_card_action_renders() {
    let action = CardAction::new(floem_shadcn::components::button::Button::new("Save"));
    let id = action.view_id();

    let container = Stack::new((action,)).style(|s| s.size(200.0, 50.0));
    let mut harness = HeadlessHarness::new_with_size(TestRoot::new(), container, 200.0, 50.0);
    harness.rebuild();

    let layout = id.get_layout().expect("CardAction layout should exist");
    assert!(
        layout.size.width > 10.0,
        "CardAction should size to content"
    );
}

#[test]
fn test_button_group_text_and_empty_header_footer() {
    let text = ButtonGroupText::new();
    let header = EmptyHeader::new();
    let footer = EmptyFooter::new();
    let text_id = text.view_id();

    let container = Stack::vertical((text, header, footer)).style(|s| s.size(200.0, 120.0));
    let mut harness = HeadlessHarness::new_with_size(TestRoot::new(), container, 200.0, 120.0);
    harness.rebuild();

    let layout = text_id
        .get_layout()
        .expect("ButtonGroupText layout should exist");
    assert!(
        layout.size.height > 10.0,
        "ButtonGroupText has button height"
    );
}

#[test]
fn test_item_new_subparts() {
    let group = ItemGroup::new()
        .child(
            Item::new().child(
                ItemHeader::new()
                    .child(ItemMedia::new())
                    .child(ItemContent::new().child(ItemTitle::new("Settings")))
                    .child(ItemActions::new()),
            ),
        )
        .child(ItemSeparator::new())
        .child(ItemFooter::new());
    let id = group.view_id();

    let container = Stack::new((group,)).style(|s| s.size(300.0, 300.0));
    let mut harness = HeadlessHarness::new_with_size(TestRoot::new(), container, 300.0, 300.0);
    harness.rebuild();

    let layout = id.get_layout().expect("ItemGroup layout should exist");
    assert!(layout.size.height > 40.0, "ItemGroup stacks its children");
}

#[test]
fn test_accordion_trigger_and_content() {
    let expanded = RwSignal::new(Some("item-1".to_string()));
    let trigger = AccordionTrigger::new("item-1", "Is it accessible?", expanded);
    let content = AccordionContent::new("item-1", "Yes it is.", expanded);
    let trigger_id = trigger.view_id();
    let content_id = content.view_id();

    let container = Stack::vertical((trigger, content)).style(|s| s.size(300.0, 200.0));
    let mut harness = HeadlessHarness::new_with_size(TestRoot::new(), container, 300.0, 200.0);
    harness.rebuild();

    let trigger_layout = trigger_id.get_layout().expect("trigger layout");
    assert!(trigger_layout.size.height > 10.0);
    let content_layout = content_id.get_layout().expect("content layout");
    assert!(content_layout.size.height > 0.0, "expanded content visible");
}

#[test]
fn test_popover_close_and_header_parts() {
    let open = RwSignal::new(true);
    let close = PopoverClose::new(open, floem::views::Label::derived(|| "Close".to_string()));
    let header = PopoverHeader::new()
        .child(PopoverTitle::new("Dimensions"))
        .child(PopoverDescription::new("Set the display dimensions"));
    let close_id = close.view_id();

    let container = Stack::vertical((close, header)).style(|s| s.size(240.0, 120.0));
    let mut harness = HeadlessHarness::new_with_size(TestRoot::new(), container, 240.0, 120.0);
    harness.rebuild();

    let layout = close_id.get_layout().expect("PopoverClose layout");
    assert!(layout.size.width > 10.0);
}

#[test]
fn test_sidebar_new_subparts() {
    let open = RwSignal::new(true);
    let active = RwSignal::new("home".to_string());

    let sidebar = Sidebar::new()
        .content(
            SidebarContent::new().child(
                SidebarGroup::new().child(
                    SidebarGroupContent::new().child(
                        SidebarMenu::new()
                            .child(
                                SidebarMenuItem::new().child(
                                    SidebarMenuButton::with_label("Home")
                                        .is_active(move || active.get() == "home"),
                                ),
                            )
                            .child(SidebarMenuBadge::new("3"))
                            .child(SidebarMenuAction::new())
                            .child(SidebarMenuSkeleton::new())
                            .child(
                                SidebarMenuSub::new().child(
                                    SidebarMenuSubItem::new().child(
                                        SidebarMenuSubButton::new()
                                            .is_active(move || active.get() == "home"),
                                    ),
                                ),
                            ),
                    ),
                ),
            ),
        )
        .footer(SidebarTrigger::new(open));

    let rail = SidebarRail::new(open);
    let inset = SidebarInset::new();
    let rail_id = rail.view_id();

    let container = Stack::horizontal((sidebar, rail, inset)).style(|s| s.size(400.0, 300.0));
    let mut harness = HeadlessHarness::new_with_size(TestRoot::new(), container, 400.0, 300.0);
    harness.rebuild();

    let layout = rail_id.get_layout().expect("SidebarRail layout");
    assert!(layout.size.width > 0.0, "rail is a thin strip");
}
