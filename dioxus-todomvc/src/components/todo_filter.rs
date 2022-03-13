use dioxus::prelude::*;

use crate::{Filter, Todos};

#[derive(Props)]
pub struct TodoFilterProps<'a> {
    pub todos: &'a UseState<Todos>,
    pub filter: &'a UseState<Filter>,
}

pub fn todo_filter<'a>(cx: Scope<'a, TodoFilterProps<'a>>) -> Element {
    let todos = cx.props.todos;
    let filter = cx.props.filter;

    let items_left = todos.iter().fold(
        0,
        |acc, (_, todo)| if todo.completed { acc } else { acc + 1 },
    );

    let item_text = if items_left == 1 {
        "item left"
    } else {
        "items left"
    };

    let show_clear_completed = todos.iter().any(|(_, todo)| todo.completed);
    let active_text = |f| {
        if filter.get() == &f {
            "selected"
        } else {
            ""
        }
    };
    let all_class = active_text(Filter::All);
    let active_class = active_text(Filter::Active);
    let completed_class = active_text(Filter::Completed);

    rsx!(cx, (!todos.is_empty()).then(|| rsx! {
        footer { class: "footer",
            span { class: "todo-count",
                strong { "{items_left}" },
                span {" {item_text}" },
            }
            ul { class: "filters",
                li {
                    a { class: "{ all_class }", href: "#/", onclick: move |_| filter.set(Filter::All),  "All" },
                },
                li {
                a { class: "{ active_class }", href: "#/active", onclick: move |_| filter.set(Filter::Active),  "Active" },
                },
                li {
                a { class: "{ completed_class }", href: "#/completed", onclick: move |_| filter.set(Filter::Completed),  "Completed" },

                },
            }

            show_clear_completed.then(|| rsx! {
                button {
                    class: "clear-completed",
                    onclick: move |_| {
                        todos.make_mut().retain(|_, todo| !todo.completed);
                    },
                    "Clear completed",
                }
            })
        }
    }))
}
