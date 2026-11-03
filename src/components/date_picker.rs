use crate::components::calendar::SimpleDate;
use crate::theme::ShadcnThemeExt;
use floem::context::LayoutChanged;
use floem::prelude::*;
use floem::reactive::{RwSignal, SignalGet, SignalUpdate};
use floem::style::CursorStyle;
use floem::views::Decorators;
use floem::{HasViewId, ViewId};
use floem_tailwind::TailwindExt;

/// The `DatePicker`.
pub struct DatePicker {
    id: ViewId,
    selected: RwSignal<Option<SimpleDate>>,
    placeholder: String,
    disabled: bool,
    trigger_origin: RwSignal<floem::kurbo::Point>,
    trigger_size: RwSignal<floem::kurbo::Size>,
}
impl DatePicker {
    #[must_use]
    /// Creates a new it.
    pub fn new(selected: RwSignal<Option<SimpleDate>>) -> Self {
        Self {
            id: ViewId::new(),
            selected,
            placeholder: "Pick a date".to_string(),
            disabled: false,
            trigger_origin: RwSignal::new(floem::kurbo::Point::ZERO),
            trigger_size: RwSignal::new(floem::kurbo::Size::ZERO),
        }
    }
    #[must_use]
    /// The placeholder.
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }
    #[must_use]
    /// The disabled.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}
impl HasViewId for DatePicker {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

/// Build the date-picker trigger chip showing the selected date (or placeholder).
fn date_picker_trigger(
    selected: RwSignal<Option<SimpleDate>>,
    placeholder: String,
    disabled: bool,
) -> impl IntoView {
    floem::views::Stack::horizontal((
        floem::views::Label::new("📅").style(floem_tailwind::TailwindExt::text_sm),
        floem::views::Label::derived(move || {
            if let Some(date) = selected.get() {
                format!("{:04}-{:02}-{:02}", date.year, date.month, date.day)
            } else {
                placeholder.clone()
            }
        })
        .style(move |s| {
            s.with_shadcn_theme(move |s, t| {
                if selected.get().is_some() {
                    s.color(t.foreground)
                } else {
                    s.color(t.muted_foreground)
                }
            })
        }),
    ))
    .style(move |s| {
        s.with_shadcn_theme(move |s, t| {
            let base = s
                .gap_2()
                .px_3()
                .py_2()
                .min_width(200.0)
                .text_sm()
                .background(t.background)
                .border_1()
                .border_color(t.input)
                .rounded_md()
                .items_center()
                .cursor(if disabled {
                    CursorStyle::Default
                } else {
                    CursorStyle::Pointer
                });
            if disabled {
                base.color(t.muted_foreground).background(t.muted)
            } else {
                base.hover(|s| s.border_color(t.ring))
            }
        })
    })
}
impl IntoView for DatePicker {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        let selected = self.selected;
        let placeholder = self.placeholder;
        let disabled = self.disabled;
        let trigger_origin = self.trigger_origin;
        let trigger_size = self.trigger_size;
        let is_open = RwSignal::new(false);
        let trigger = date_picker_trigger(selected, placeholder, disabled);
        let trigger = if disabled {
            trigger.into_any()
        } else {
            trigger
                .on_event_stop(
                    LayoutChanged::listener(),
                    move |_cx, event: &LayoutChanged| {
                        trigger_origin.set(event.new_window_origin);
                        trigger_size.set(event.new_box.size());
                    },
                )
                .on_event_stop(floem::event::listener::Click, move |_, ()| {
                    is_open.update(|v| *v = !*v);
                })
                .into_any()
        };
        let view_year = RwSignal::new(2024);
        let view_month = RwSignal::new(12u32);
        if let Some(date) = selected.get_untracked() {
            view_year.set(date.year);
            view_month.set(date.month);
        }
        let calendar = create_calendar_content(selected, view_year, view_month, is_open);
        let popup = floem::views::Container::new(calendar).style(move |s| {
            s.with_shadcn_theme(move |s, t| {
                let open = is_open.get();
                let origin = trigger_origin.get();
                let tsize = trigger_size.get();
                let base = s
                    .absolute()
                    .inset_left(origin.x)
                    .inset_top(origin.y + tsize.height + 4.0)
                    .min_width(tsize.width.max(200.0))
                    .mt_1()
                    .p_3()
                    .background(t.popover)
                    .border_1()
                    .border_color(t.border)
                    .rounded_md()
                    .box_shadow_blur(8.0)
                    .box_shadow_color(t.foreground.with_alpha(0.1))
                    .z_index(100);
                if open {
                    base
                } else {
                    base.display(floem::style::Display::None)
                }
            })
        });
        let backdrop = floem::views::Empty::new()
            .style(move |s| {
                let open = is_open.get();
                if open {
                    s.absolute()
                        .inset_top(-1000.0)
                        .inset_left(-1000.0)
                        .width(3000.0)
                        .height(3000.0)
                        .z_index(99)
                } else {
                    s.display(floem::style::Display::None)
                }
            })
            .on_event_stop(floem::event::listener::Click, move |_, ()| {
                is_open.set(false);
            });
        Box::new(
            floem::views::Container::new(floem::views::Stack::new((trigger, backdrop, popup)))
                .style(floem_tailwind::TailwindExt::relative),
        )
    }
}
fn create_calendar_content(
    _selected: RwSignal<Option<SimpleDate>>,
    view_year: RwSignal<i32>,
    view_month: RwSignal<u32>,
    _is_open: RwSignal<bool>,
) -> impl IntoView {
    let header = floem::views::Stack::horizontal((
        floem::views::Label::new("◀")
            .style(|s| {
                s.with_shadcn_theme(move |s, t| {
                    s.p_1()
                        .text_xs()
                        .color(t.foreground)
                        .rounded_sm()
                        .cursor(CursorStyle::Pointer)
                        .hover(|s| s.background(t.accent))
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
            }),
        floem::views::Label::derived(move || {
            let month_names = [
                "",
                "January",
                "February",
                "March",
                "April",
                "May",
                "June",
                "July",
                "August",
                "September",
                "October",
                "November",
                "December",
            ];
            let m = view_month.get() as usize;
            let y = view_year.get();
            format!("{} {}", month_names.get(m).unwrap_or(&""), y)
        })
        .style(|s| {
            s.with_shadcn_theme(move |s, t| {
                s.flex_grow(1.0_f32)
                    .text_sm()
                    .font_medium()
                    .color(t.foreground)
                    .justify_center()
            })
        }),
        floem::views::Label::new("▶")
            .style(|s| {
                s.with_shadcn_theme(move |s, t| {
                    s.p_1()
                        .text_xs()
                        .color(t.foreground)
                        .rounded_sm()
                        .cursor(CursorStyle::Pointer)
                        .hover(|s| s.background(t.accent))
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
            }),
    ))
    .style(|s| s.w_full().items_center().mb_2());
    floem::views::Stack::vertical((header,))
}
/// The `DateRangePicker`.
pub struct DateRangePicker {
    id: ViewId,
    start: RwSignal<Option<SimpleDate>>,
    end: RwSignal<Option<SimpleDate>>,
    placeholder: String,
    disabled: bool,
}

impl DateRangePicker {
    #[must_use]
    /// Create a range picker bound to the given start/end signals.
    pub fn new(start: RwSignal<Option<SimpleDate>>, end: RwSignal<Option<SimpleDate>>) -> Self {
        Self {
            id: ViewId::new(),
            start,
            end,
            placeholder: "Pick a date range".to_string(),
            disabled: false,
        }
    }

    #[must_use]
    /// Override the placeholder shown when no range is selected.
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    #[must_use]
    /// Disable interaction with the picker.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    #[must_use]
    /// The start-date signal.
    pub fn start_signal(&self) -> RwSignal<Option<SimpleDate>> {
        self.start
    }

    #[must_use]
    /// The end-date signal.
    pub fn end_signal(&self) -> RwSignal<Option<SimpleDate>> {
        self.end
    }
}

impl HasViewId for DateRangePicker {
    fn view_id(&self) -> ViewId {
        self.id
    }
}

impl IntoView for DateRangePicker {
    type V = Box<dyn View>;
    type Intermediate = Box<dyn View>;
    fn into_intermediate(self) -> Self::Intermediate {
        self.into_view()
    }
    fn into_view(self) -> Self::V {
        let start = self.start;
        let end = self.end;
        let placeholder = self.placeholder;
        let disabled = self.disabled;
        let trigger = floem::views::Stack::horizontal((
            floem::views::Label::new("📅").style(floem_tailwind::TailwindExt::text_sm),
            floem::views::Label::derived(move || match (start.get(), end.get()) {
                (Some(s), Some(e)) => {
                    format!(
                        "{:04}-{:02}-{:02} → {:04}-{:02}-{:02}",
                        s.year, s.month, s.day, e.year, e.month, e.day
                    )
                }
                (Some(s), None) => {
                    format!("{:04}-{:02}-{:02} → …", s.year, s.month, s.day)
                }
                _ => placeholder.clone(),
            })
            .style(move |s| {
                s.with_shadcn_theme(move |s, t| {
                    if start.get().is_some() {
                        s.color(t.foreground)
                    } else {
                        s.color(t.muted_foreground)
                    }
                })
            }),
        ))
        .style(move |s| {
            s.with_shadcn_theme(move |s, t| {
                let base = s
                    .gap_2()
                    .px_3()
                    .py_2()
                    .min_width(240.0)
                    .text_sm()
                    .background(t.background)
                    .border_1()
                    .border_color(t.input)
                    .rounded_md()
                    .items_center()
                    .cursor(if disabled {
                        CursorStyle::Default
                    } else {
                        CursorStyle::Pointer
                    });
                if disabled {
                    base.color(t.muted_foreground).background(t.muted)
                } else {
                    base.hover(|s| s.border_color(t.ring))
                }
            })
        });
        Box::new(floem::views::Container::with_id(self.id, trigger))
    }
}
