//! Integration tests for the Dialog component.

use floem::headless::TestRoot;
use floem::prelude::*;
use floem::reactive::RwSignal;
use floem::views::Scroll;
use floem_shadcn::components::button::Button;
use floem_shadcn::components::dialog::{
    Dialog, DialogClose, DialogContent, DialogContext, DialogFooter, DialogHeader, DialogTrigger,
};
use floem_test::prelude::*;

#[test]
fn test_dialog_opens_via_trigger() {
    floem_shadcn::theme::set_theme(floem_shadcn::theme::ShadcnTheme::light());
    // TestRoot must be created before the view tree: ViewIds created before
    // the root exists never receive dispatched events.
    let root = TestRoot::new();
    let dialog = Dialog::new((
        DialogTrigger::new(Button::new("Open Dialog")),
        DialogContent::new((DialogHeader::new()
            .title("Test Dialog")
            .description("This is a test dialog"),)),
    ));
    let open = dialog.open_signal();
    let dialog_id = dialog.view_id();
    let view = Stack::vertical((dialog,));
    let mut harness = HeadlessHarness::new_with_size(root, view, 800.0, 600.0);
    harness.rebuild();
    assert!(!open.get(), "Dialog starts closed");
    // Click the trigger button at the top of the stack.
    harness.click(50.0, 20.0);
    harness.rebuild();
    assert!(open.get(), "Clicking the trigger should open the dialog");
    assert!(
        dialog_id.get_layout().is_some(),
        "Dialog renders a layout once mounted"
    );
}

#[test]
fn test_dialog_closes_via_close_button() {
    floem_shadcn::theme::set_theme(floem_shadcn::theme::ShadcnTheme::light());
    let dialog = Dialog::new((
        DialogTrigger::new(Button::new("Open")),
        DialogContent::new((
            DialogHeader::new().title("Test"),
            DialogFooter::new(DialogClose::new(Button::new("Close"))),
        )),
    ));
    let open = dialog.open_signal();
    let view = Stack::new((dialog,)).style(|s| s.size(800.0, 600.0));
    let mut harness = HeadlessHarness::new_with_size(TestRoot::new(), view, 800.0, 600.0);
    harness.rebuild();
    open.set(true);
    harness.rebuild();
    assert!(open.get(), "Dialog should be open");
    harness.click(400.0, 300.0);
    harness.rebuild();
}

#[test]
fn test_dialog_backdrop_click_closes() {
    floem_shadcn::theme::set_theme(floem_shadcn::theme::ShadcnTheme::light());
    // TestRoot must be created before the view tree: ViewIds created before
    // the root exists are registered outside the root's view registry and
    // never receive dispatched events.
    let root = TestRoot::new();
    let dialog = Dialog::new((
        DialogTrigger::new(Button::new("Open")),
        DialogContent::new((DialogHeader::new().title("Test"),)),
    ));
    let open = dialog.open_signal();
    let view = Stack::new((dialog,)).style(|s| s.size(800.0, 600.0));
    let mut harness = HeadlessHarness::new_with_size(root, view, 800.0, 600.0);
    harness.rebuild();
    open.set(true);
    harness.rebuild();
    harness.click(10.0, 10.0);
    harness.rebuild();
    assert!(!open.get(), "Dialog should close on backdrop click");
}

#[test]
fn test_dialog_reactivity() {
    use std::cell::RefCell;
    use std::rc::Rc;

    floem_shadcn::theme::set_theme(floem_shadcn::theme::ShadcnTheme::light());
    let dialog = Dialog::new((
        DialogTrigger::new(Button::new("Open")),
        DialogContent::new((DialogHeader::new().title("Test"),)),
    ));
    let open = dialog.open_signal();
    let effect_count = Rc::new(RefCell::new(0));
    let effect_count_clone = effect_count.clone();
    floem::reactive::Effect::new(move |_| {
        let _is_open = open.get();
        *effect_count_clone.borrow_mut() += 1;
    });
    let view = Stack::new((dialog,)).style(|s| s.size(800.0, 600.0));
    let mut harness = HeadlessHarness::new_with_size(TestRoot::new(), view, 800.0, 600.0);
    harness.rebuild();
    open.set(true);
    harness.rebuild();
    open.set(false);
    harness.rebuild();
    assert!(*effect_count.borrow() >= 2);
}

#[test]
fn test_dialog_with_paint_cycle() {
    floem_shadcn::theme::set_theme(floem_shadcn::theme::ShadcnTheme::light());
    let dialog = Dialog::new((
        DialogTrigger::new(Button::new("Open")),
        DialogContent::new((
            DialogHeader::new()
                .title("Test Dialog")
                .description("Testing paint cycle"),
            DialogFooter::new(DialogClose::new(Button::new("Close"))),
        )),
    ));
    let open = dialog.open_signal();
    let view = Stack::new((dialog,)).style(|s| s.size(800.0, 600.0));
    let mut harness = HeadlessHarness::new_with_size(TestRoot::new(), view, 800.0, 600.0);
    harness.rebuild();
    harness.paint();
    open.set(true);
    harness.rebuild();
    harness.paint();
    open.set(false);
    harness.rebuild();
    harness.paint();
    open.set(true);
    harness.rebuild();
    harness.paint();
    open.set(false);
    harness.rebuild();
    harness.paint();
    assert!(!open.get());
}

#[test]
fn test_dialog_inside_scroll() {
    floem_shadcn::theme::set_theme(floem_shadcn::theme::ShadcnTheme::light());
    let dialog = Dialog::new((
        DialogTrigger::new(Button::new("Open Dialog")),
        DialogContent::new((
            DialogHeader::new()
                .title("Test Dialog in Scroll")
                .description("Inside scroll"),
            DialogFooter::new(DialogClose::new(Button::new("Close"))),
        )),
    ));
    let open = dialog.open_signal();
    let view = Scroll::new(Stack::vertical((dialog,)));
    let mut harness = HeadlessHarness::new_with_size(TestRoot::new(), view, 800.0, 600.0);
    harness.rebuild();
    open.set(true);
    harness.rebuild();
    assert!(open.get());
    open.set(false);
    harness.rebuild();
    assert!(!open.get());
}

#[test]
fn test_dialog_with_dyn_container_in_scroll() {
    floem_shadcn::theme::set_theme(floem_shadcn::theme::ShadcnTheme::light());
    let section = RwSignal::new("dialog".to_string());
    let dialog_open = RwSignal::new(false);
    let view = Scroll::new(floem::views::dyn_container(
        move || section.get(),
        move |s| match s.as_str() {
            "dialog" => {
                let dialog = Dialog::new((
                    DialogTrigger::new(Button::new("Open Dialog")),
                    DialogContent::new((DialogHeader::new().title("Test"),)),
                ));
                let open = dialog.open_signal();
                floem::reactive::Effect::new(move |_| {
                    dialog_open.set(open.get());
                });
                Stack::vertical((dialog,)).into_any()
            }
            _ => Label::new("Other").into_any(),
        },
    ));
    let mut harness = HeadlessHarness::new_with_size(TestRoot::new(), view, 800.0, 600.0);
    harness.rebuild();
    harness.click(50.0, 20.0);
    harness.rebuild();
}

#[test]
fn test_multiple_dialogs() {
    floem_shadcn::theme::set_theme(floem_shadcn::theme::ShadcnTheme::light());
    let dialog1 = Dialog::new((
        DialogTrigger::new(Button::new("Open Dialog 1")),
        DialogContent::new((DialogHeader::new().title("Dialog 1"),)),
    ));
    let open1 = dialog1.open_signal();
    let dialog2 = Dialog::new((
        DialogTrigger::new(Button::new("Open Dialog 2")),
        DialogContent::new((DialogHeader::new().title("Dialog 2"),)),
    ));
    let open2 = dialog2.open_signal();
    let view = Stack::vertical((dialog1, dialog2));
    let mut harness = HeadlessHarness::new_with_size(TestRoot::new(), view, 800.0, 600.0);
    harness.rebuild();
    open1.set(true);
    open2.set(true);
    harness.rebuild();
    assert!(open1.get() && open2.get());
    open1.set(false);
    harness.rebuild();
    assert!(!open1.get() && open2.get());
}

#[test]
fn test_clicking_backdrop_closes_dialog() {
    floem_shadcn::theme::set_theme(floem_shadcn::theme::ShadcnTheme::light());
    let root = TestRoot::new();
    let dialog = Dialog::new((
        DialogTrigger::new(Button::new("Open")),
        DialogContent::new((DialogHeader::new()
            .title("Dialog")
            .description("Click outside to close"),)),
    ));
    let open = dialog.open_signal();
    let view = Stack::new((dialog,)).style(|s| s.size(800.0, 600.0));
    let mut harness = HeadlessHarness::new_with_size(root, view, 800.0, 600.0);
    harness.rebuild();
    open.set(true);
    harness.rebuild();
    // The backdrop fills the whole viewport; (10, 10) is outside the centered
    // content panel, so this click must hit the backdrop and close the dialog.
    harness.click(10.0, 10.0);
    harness.rebuild();
    assert!(!open.get(), "Clicking the backdrop should close the dialog");
}

#[test]
fn test_dialog_context_access() {
    use floem::reactive::Context;

    floem_shadcn::theme::set_theme(floem_shadcn::theme::ShadcnTheme::light());
    let context_found = RwSignal::new(false);
    let custom_content = floem::views::Empty::new().style(move |s| {
        if Context::get::<DialogContext>().is_some() {
            context_found.set(true);
        }
        s
    });
    let dialog = Dialog::new((
        DialogTrigger::new(Button::new("Open")),
        DialogContent::new((custom_content,)),
    ));
    let open = dialog.open_signal();
    let view = Stack::new((dialog,)).style(|s| s.size(800.0, 600.0));
    let mut harness = HeadlessHarness::new_with_size(TestRoot::new(), view, 800.0, 600.0);
    harness.rebuild();
    open.set(true);
    harness.rebuild();
}
