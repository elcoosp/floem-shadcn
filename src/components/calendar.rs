//! Calendar component with builder-style API
//!
//! Based on shadcn/ui Calendar - a date picker component.
//!
//! # Example
//!
//! ```
//! use floem::reactive::RwSignal;
//! use floem_shadcn::components::calendar::*;
//!
//! let selected_date = RwSignal::new(None);
//!
//! Calendar::new(selected_date);
//! ```

use crate::theme::ShadcnThemeExt;
use floem::prelude::*;
use floem::reactive::{RwSignal, SignalGet, SignalUpdate};
use floem::style::CursorStyle;
use floem::views::Decorators;
use floem::{HasViewId, ViewId};
use floem_tailwind::TailwindExt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// The `SimpleDate`.
pub struct SimpleDate {
    /// The item.
    pub year: i32,
    /// The item.
    pub month: u32,
    /// The item.
    pub day: u32,
}

impl SimpleDate {
    #[must_use]
    /// Creates a new it.
    pub fn new(year: i32, month: u32, day: u32) -> Self {
        Self { year, month, day }
    }
    #[must_use]
    /// The today.
    pub fn today() -> Self {
        Self::new(2025, 1, 1)
    }
    #[must_use]
    /// The days in month.
    pub fn days_in_month(year: i32, month: u32) -> u32 {
        match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            2 => {
                if (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0) {
                    29
                } else {
                    28
                }
            }
            _ => 30,
        }
    }
    #[must_use]
    /// The first day of week.
    pub fn first_day_of_week(year: i32, month: u32) -> u32 {
        let month_i = i32::try_from(month).unwrap_or(1);
        let (year_adj, month_adj) = if month_i < 3 {
            (year - 1, month_i + 12)
        } else {
            (year, month_i)
        };
        let day_of_month = 1;
        let year_of_century = year_adj % 100;
        let century = year_adj / 100;
        let weekday = (day_of_month
            + (13 * (month_adj + 1)) / 5
            + year_of_century
            + year_of_century / 4
            + century / 4
            + 5 * century)
            % 7;
        u32::try_from((weekday + 6) % 7).unwrap_or(0)
    }
    #[must_use]
    /// The month name.
    pub fn month_name(month: u32) -> &'static str {
        match month {
            1 => "January",
            2 => "February",
            3 => "March",
            4 => "April",
            5 => "May",
            6 => "June",
            7 => "July",
            8 => "August",
            9 => "September",
            10 => "October",
            11 => "November",
            12 => "December",
            _ => "Unknown",
        }
    }
}

/// The Calendar.
pub struct Calendar {
    id: ViewId,
    selected: RwSignal<Option<SimpleDate>>,
    view_year: RwSignal<i32>,
    view_month: RwSignal<u32>,
}
impl Calendar {
    #[must_use]
    /// Creates a new it.
    pub fn new(selected: RwSignal<Option<SimpleDate>>) -> Self {
        let today = SimpleDate::today();
        Self {
            id: ViewId::new(),
            selected,
            view_year: RwSignal::new(today.year),
            view_month: RwSignal::new(today.month),
        }
    }
    #[must_use]
    /// The year.
    pub fn year(self, year: i32) -> Self {
        self.view_year.set(year);
        self
    }
    #[must_use]
    /// The month.
    pub fn month(self, month: u32) -> Self {
        self.view_month.set(month);
        self
    }
}
impl HasViewId for Calendar {
    fn view_id(&self) -> ViewId {
        self.id
    }
}
impl IntoView for Calendar {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        let selected = self.selected;
        let view_year = self.view_year;
        let view_month = self.view_month;

        let prev_button = floem::views::Label::new("<")
            .style(|s| {
                s.with_shadcn_theme(move |s, t| {
                    s.p_2()
                        .text_sm()
                        .color(t.foreground)
                        .cursor(CursorStyle::Pointer)
                        .rounded_md()
                        .hover(|s| s.background(t.muted))
                })
            })
            .on_event_stop(floem::event::listener::Click, move |_, ()| {
                let m = view_month.get();
                if m == 1 {
                    view_month.set(12);
                    view_year.update(|y| *y -= 1);
                } else {
                    view_month.set(m - 1);
                }
            });
        let next_button = floem::views::Label::new(">")
            .style(|s| {
                s.with_shadcn_theme(move |s, t| {
                    s.p_2()
                        .text_sm()
                        .color(t.foreground)
                        .cursor(CursorStyle::Pointer)
                        .rounded_md()
                        .hover(|s| s.background(t.muted))
                })
            })
            .on_event_stop(floem::event::listener::Click, move |_, ()| {
                let m = view_month.get();
                if m == 12 {
                    view_month.set(1);
                    view_year.update(|y| *y += 1);
                } else {
                    view_month.set(m + 1);
                }
            });
        let month_label = floem::views::Label::derived(move || {
            format!(
                "{} {}",
                SimpleDate::month_name(view_month.get()),
                view_year.get()
            )
        })
        .style(|s| {
            s.with_shadcn_theme(move |s, t| {
                s.flex_grow(1.0_f32)
                    .text_sm()
                    .font_medium()
                    .color(t.foreground)
                    .justify_center()
            })
        });
        let header = floem::views::Stack::horizontal((prev_button, month_label, next_button))
            .style(|s| s.w_full().pb_2().items_center());

        let day_names = floem::views::Stack::horizontal((
            day_header("Su"),
            day_header("Mo"),
            day_header("Tu"),
            day_header("We"),
            day_header("Th"),
            day_header("Fr"),
            day_header("Sa"),
        ))
        .style(|s| s.w_full().mb_1());

        let grid = floem::views::Stack::vertical((
            create_week_row(0, selected, view_year, view_month),
            create_week_row(1, selected, view_year, view_month),
            create_week_row(2, selected, view_year, view_month),
            create_week_row(3, selected, view_year, view_month),
            create_week_row(4, selected, view_year, view_month),
            create_week_row(5, selected, view_year, view_month),
        ))
        .style(floem_tailwind::TailwindExt::gap_0p5);

        Box::new(
            floem::views::Stack::vertical((header, day_names, grid)).style(|s| {
                s.with_shadcn_theme(move |s, t| {
                    s.p_3()
                        .background(t.card)
                        .border_1()
                        .border_color(t.border)
                        .rounded_md()
                })
            }),
        )
    }
}

fn day_header(text: &'static str) -> impl IntoView {
    floem::views::Label::new(text).style(|s| {
        s.with_shadcn_theme(move |s, t| {
            s.w_8()
                .h_8()
                .text_xs()
                .font_medium()
                .color(t.muted_foreground)
                .flex()
                .items_center()
                .justify_center()
        })
    })
}

fn create_week_row(
    week: i32,
    selected: RwSignal<Option<SimpleDate>>,
    view_year: RwSignal<i32>,
    view_month: RwSignal<u32>,
) -> impl IntoView {
    floem::views::Stack::horizontal((
        create_day_cell(week * 7, selected, view_year, view_month),
        create_day_cell(week * 7 + 1, selected, view_year, view_month),
        create_day_cell(week * 7 + 2, selected, view_year, view_month),
        create_day_cell(week * 7 + 3, selected, view_year, view_month),
        create_day_cell(week * 7 + 4, selected, view_year, view_month),
        create_day_cell(week * 7 + 5, selected, view_year, view_month),
        create_day_cell(week * 7 + 6, selected, view_year, view_month),
    ))
    .style(floem_tailwind::TailwindExt::w_full)
}

fn create_day_cell(
    cell_index: i32,
    selected: RwSignal<Option<SimpleDate>>,
    view_year: RwSignal<i32>,
    view_month: RwSignal<u32>,
) -> impl IntoView {
    floem::views::Label::derived(move || {
        let year = view_year.get();
        let month = view_month.get();
        let first_day = i32::try_from(SimpleDate::first_day_of_week(year, month)).unwrap_or(0);
        let days_in_month = i32::try_from(SimpleDate::days_in_month(year, month)).unwrap_or(0);
        let day_num = cell_index - first_day + 1;
        if day_num < 1 || day_num > days_in_month {
            String::new()
        } else {
            day_num.to_string()
        }
    })
    .style(move |s| {
        s.with_shadcn_theme(move |s, t| {
            let year = view_year.get();
            let month = view_month.get();
            let sel = selected.get();
            let first_day = i32::try_from(SimpleDate::first_day_of_week(year, month)).unwrap_or(0);
            let days_in_month = i32::try_from(SimpleDate::days_in_month(year, month)).unwrap_or(0);
            let day_num = cell_index - first_day + 1;
            let is_valid = day_num >= 1 && day_num <= days_in_month;
            let is_selected = is_valid
                && sel.is_some_and(|d| {
                    d.year == year
                        && d.month == month
                        && d.day == u32::try_from(day_num).unwrap_or_default()
                });
            let base = s
                .w_8()
                .h_8()
                .text_sm()
                .rounded_sm()
                .flex()
                .items_center()
                .justify_center();
            if !is_valid {
                base
            } else if is_selected {
                base.background(t.primary)
                    .color(t.primary_foreground)
                    .cursor(CursorStyle::Pointer)
            } else {
                base.color(t.foreground)
                    .cursor(CursorStyle::Pointer)
                    .hover(|s| s.background(t.accent).color(t.accent_foreground))
            }
        })
    })
    .on_event_stop(floem::event::listener::Click, move |_, ()| {
        let year = view_year.get();
        let month = view_month.get();
        let first_day = i32::try_from(SimpleDate::first_day_of_week(year, month)).unwrap_or(0);
        let days_in_month = i32::try_from(SimpleDate::days_in_month(year, month)).unwrap_or(0);
        let day_num = cell_index - first_day + 1;
        if day_num >= 1 && day_num <= days_in_month {
            selected.set(Some(SimpleDate::new(
                year,
                month,
                u32::try_from(day_num).unwrap_or(1),
            )));
        }
    })
}

/// The `CalendarSimple`.
pub struct CalendarSimple {
    id: ViewId,
    year: i32,
    month: u32,
}
impl CalendarSimple {
    #[must_use]
    /// Creates a new it.
    pub fn new(year: i32, month: u32) -> Self {
        Self {
            id: ViewId::new(),
            year,
            month,
        }
    }
}
impl HasViewId for CalendarSimple {
    fn view_id(&self) -> ViewId {
        self.id
    }
}
impl IntoView for CalendarSimple {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        let year = self.year;
        let month = self.month;
        let title = floem::views::Label::new(format!("{} {}", SimpleDate::month_name(month), year))
            .style(|s| {
                s.with_shadcn_theme(move |s, t| {
                    s.w_full()
                        .pb_2()
                        .text_sm()
                        .font_medium()
                        .color(t.foreground)
                        .justify_center()
                })
            });
        let day_names = floem::views::Stack::horizontal((
            day_header("Su"),
            day_header("Mo"),
            day_header("Tu"),
            day_header("We"),
            day_header("Th"),
            day_header("Fr"),
            day_header("Sa"),
        ))
        .style(|s| s.w_full().mb_1());
        let first_day = i32::try_from(SimpleDate::first_day_of_week(year, month)).unwrap_or(0);
        let days_in_month = i32::try_from(SimpleDate::days_in_month(year, month)).unwrap_or(0);
        let grid = floem::views::Stack::vertical((
            create_static_week(0, first_day, days_in_month),
            create_static_week(1, first_day, days_in_month),
            create_static_week(2, first_day, days_in_month),
            create_static_week(3, first_day, days_in_month),
            create_static_week(4, first_day, days_in_month),
            create_static_week(5, first_day, days_in_month),
        ))
        .style(floem_tailwind::TailwindExt::gap_0p5);
        Box::new(
            floem::views::Stack::vertical((title, day_names, grid)).style(|s| {
                s.with_shadcn_theme(move |s, t| {
                    s.p_3()
                        .background(t.card)
                        .border_1()
                        .border_color(t.border)
                        .rounded_md()
                })
            }),
        )
    }
}

fn create_static_week(week: i32, first_day: i32, days_in_month: i32) -> impl IntoView {
    floem::views::Stack::horizontal((
        static_day_cell(week * 7, first_day, days_in_month),
        static_day_cell(week * 7 + 1, first_day, days_in_month),
        static_day_cell(week * 7 + 2, first_day, days_in_month),
        static_day_cell(week * 7 + 3, first_day, days_in_month),
        static_day_cell(week * 7 + 4, first_day, days_in_month),
        static_day_cell(week * 7 + 5, first_day, days_in_month),
        static_day_cell(week * 7 + 6, first_day, days_in_month),
    ))
    .style(floem_tailwind::TailwindExt::w_full)
}

fn static_day_cell(cell_index: i32, first_day: i32, days_in_month: i32) -> impl IntoView {
    let day_num = cell_index - first_day + 1;
    let text = if day_num >= 1 && day_num <= days_in_month {
        day_num.to_string()
    } else {
        String::new()
    };
    floem::views::Label::new(text).style(move |s| {
        s.with_shadcn_theme(move |s, t| {
            s.w_8()
                .h_8()
                .text_sm()
                .rounded_sm()
                .color(t.foreground)
                .flex()
                .items_center()
                .justify_center()
                .hover(|s| {
                    if day_num >= 1 && day_num <= days_in_month {
                        s.background(t.accent).color(t.accent_foreground)
                    } else {
                        s
                    }
                })
        })
    })
}
