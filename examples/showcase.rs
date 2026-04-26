use floem::prelude::{SignalGet, SignalUpdate};
use floem::text::FontWeight;
use floem::views::Decorators;
use floem_shadcn::theme::{current_theme, set_theme};

/// Showcase example for floem-shadcn components
///
/// Run with: cargo run --example showcase
use floem::IntoView;
use floem::reactive::RwSignal;
use floem::views::{Label, Stack};
use floem_shadcn::prelude::*;
use floem_shadcn::theme::{ShadcnTheme, ShadcnThemeExt, ThemeMode};
use floem_tailwind::TailwindExt;

fn main() {
    floem::launch(app_view);
}

fn app_view() -> impl IntoView {
    let active_section = RwSignal::new("buttons".to_string());
    let theme_mode = RwSignal::new(ThemeMode::Light);

    // Keep global theme in sync
    floem::reactive::Effect::new(move |_| {
        let theme = match theme_mode.get() {
            ThemeMode::Light => ShadcnTheme::light(),
            ThemeMode::Dark => ShadcnTheme::dark(),
        };
        set_theme(theme);
    });

    Stack::horizontal((
        Sidebar::new()
            .header(
                SidebarHeader::new().child(
                    Stack::vertical((
                        Label::derived(|| "floem-shadcn")
                            .style(|s| s.text_lg().font_weight(FontWeight::BOLD)),
                        Button::new("Toggle Theme").outline().sm().on_event_stop(
                            floem::event::listener::Click,
                            move |_, _| {
                                theme_mode.update(|m| {
                                    *m = match m {
                                        ThemeMode::Light => ThemeMode::Dark,
                                        ThemeMode::Dark => ThemeMode::Light,
                                    }
                                });
                            },
                        ),
                    ))
                    .style(|s| s.gap_3()),
                ),
            )
            .content(
                SidebarContent::new()
                    .child(sidebar_group(
                        "Form Inputs",
                        active_section,
                        vec![
                            "Buttons",
                            "Badges",
                            "Cards",
                            "Inputs",
                            "Textarea",
                            "Checkbox",
                            "Switch",
                            "Radio Group",
                            "Slider",
                            "Select",
                            "Combobox",
                            "Input OTP",
                            "Date Picker",
                            "Label",
                        ],
                    ))
                    .child(SidebarSeparator::new())
                    .child(sidebar_group(
                        "Layout & Feedback",
                        active_section,
                        vec![
                            "Tabs",
                            "Accordion",
                            "Collapsible",
                            "Dialog",
                            "Alert Dialog",
                            "Drawer",
                            "Alert",
                            "Avatar",
                            "Progress",
                            "Separator",
                            "Skeleton",
                            "Tooltip",
                            "Aspect Ratio",
                            "Scroll Area",
                            "Resizable",
                        ],
                    ))
                    .child(SidebarSeparator::new())
                    .child(sidebar_group(
                        "Overlays & Navigation",
                        active_section,
                        vec![
                            "Popover",
                            "Sheet",
                            "Dropdown Menu",
                            "Menubar",
                            "Navigation Menu",
                            "Breadcrumb",
                            "Pagination",
                            "Command",
                        ],
                    ))
                    .child(SidebarSeparator::new())
                    .child(sidebar_group(
                        "Data & Misc",
                        active_section,
                        vec![
                            "Table",
                            "Calendar",
                            "Carousel",
                            "Toast",
                            "Toggle",
                            "Toggle Group",
                            "Hover Card",
                            "Context Menu",
                            "Sidebar",
                        ],
                    )),
            )
            .footer(
                SidebarFooter::new().child(Label::derived(|| "v0.1.0").style(|s| {
                    s.text_xs()
                        .with_shadcn_theme(|s, t| s.color(t.muted_foreground))
                })),
            ),
        floem::views::Scroll::new(floem::views::dyn_container(
            move || active_section.get(),
            move |section| match section.as_str() {
                "buttons" => buttons_demo().into_any(),
                _ => buttons_demo().into_any(),
            },
        ))
        .style(|s| s.flex_grow(1.0).h_full().p_8().bg_background()),
    ))
    .style(move |s| {
        let _ = theme_mode.get(); // reactive dependency
        s.with_shadcn_theme(|s, t| {
            s.background(t.background)
                .color(t.foreground)
                .w_full()
                .h_full()
        })
    })
}

// Helper: sidebar group with owned items (no lifetime issues)
fn sidebar_group(label: &str, active_section: RwSignal<String>, items: Vec<&str>) -> SidebarGroup {
    let mut content = SidebarGroupContent::new().child(SidebarMenu::new());
    for item in items {
        let id = item.to_lowercase().replace(' ', "_");
        let id_clone = id.clone();
        content = content.child(
            SidebarMenuItem::new().child(
                SidebarMenuButton::new(item.to_string())
                    .is_active(move || active_section.get() == id)
                    .on_event_stop(floem::event::listener::Click, move |_, _| {
                        active_section.set(id_clone.clone())
                    }),
            ),
        );
    }
    SidebarGroup::new()
        .child(SidebarGroupLabel::new(label.to_string()))
        .child(content)
}

// Minimum demo for compilation
fn buttons_demo() -> impl IntoView {
    demo_section(
        "Buttons",
        "A button component with multiple variants and sizes.",
        Stack::vertical((Stack::horizontal((
            Button::new("Default"),
            Button::new("Secondary").secondary(),
            Button::new("Destructive").destructive(),
        ))
        .style(|s| s.gap_2().flex_wrap(floem::style::FlexWrap::Wrap)),))
        .style(|s| s.gap_8()),
    )
}

fn demo_section<V: IntoView + 'static>(
    title: &'static str,
    desc: &'static str,
    content: V,
) -> impl IntoView {
    Stack::vertical((
        Label::derived(move || title).style(|s| s.text_2xl().font_weight(FontWeight::BOLD).mb_2()),
        Label::derived(move || desc).style(|s| {
            s.text_sm()
                .mb_6()
                .with_shadcn_theme(|s, t| s.color(t.muted_foreground))
        }),
        content,
    ))
    .style(|s| s.w_full())
}
