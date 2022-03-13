use std::sync::atomic::{AtomicU32, Ordering};

use dioxus::prelude::*;

use crate::{Todos, TodoItem};

static NEXT_TODO_ID: AtomicU32 = AtomicU32::new(1);

#[derive(Props)]
pub struct TodoInputProps<'a> {
    pub todos: &'a UseState<Todos>
}

pub fn todo_input<'a>(cx: Scope<'a, TodoInputProps<'a>>) -> Element {
    let draft = use_state(&cx, || "".to_string());
    let todos = cx.props.todos;

    rsx!(cx, header {
        class: "header",
        h1 { "todos" },
        input {
            class: "new-todo",
            placeholder: "What needs to be done?",
            value: "{draft}",
            oninput: move |e| {
                draft.set(e.value.clone());
            },
            onkeydown: move |e| {
                if e.key == "Enter" && !draft.is_empty() {
                    let id = NEXT_TODO_ID.fetch_add(1, Ordering::Relaxed);
                    todos.make_mut().insert(id, TodoItem {
                        id,
                        title: draft.get().clone(),
                        completed: false,
                    });
                    draft.set("".to_string());
                }
            }
        }
    })
}
