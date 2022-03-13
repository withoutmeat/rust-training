use dioxus::prelude::*;
use log::info;

use crate::Todos;

#[derive(Props)]
pub struct TodoItemProps<'a> {
    pub id: u32,
    pub todos: &'a UseState<Todos>,
}

pub fn todo_item<'a>(cx: Scope<'a, TodoItemProps<'a>>) -> Element {   
    let id = cx.props.id;
    let todos = cx.props.todos;
    let todo = &todos[&id];

    let is_editing = use_state(&cx, || false);

    let completed = if todo.completed { "completed" } else { "" };
    let editing = if *is_editing.get() { "editing" } else { "" };

    

    rsx!(cx, li {
        class: "{completed} {editing}",
        div { 
            class: "view",
            input {
                class: "toggle",
                r#type: "checkbox",
                id: "todo-{todo.id}",
                onclick: move |e| {
                    info!("todo item clicked: {e:?}");
                    info!("todo item clicked: {}", todo.completed);
                    todos.make_mut().get_mut(&id).map(|todo| {
                        todo.completed = !todo.completed;
                    });
                }
            },
            label {
                // r#for: "todo-{todo.id}",
                onclick: move |e| {
                    info!("clicked label: {e:?}");
                    is_editing.set(true);
                },
                "{todo.title}"
            }
        }
        is_editing.then(|| rsx! {
            input {
                class: "edit",
                value: "{todo.title}",
                oninput: move |e| {
                    todos.make_mut().get_mut(&id).map(|todo| {
                        todo.title = e.value.clone();
                    });
                },
                autofocus: "true",
                onkeydown: move |e| {
                    match e.key.as_str() {
                        "Enter" | "Escape" | "Tab" => {
                            is_editing.set(false);
                        },

                        _ => {}
                    }
                },
            }
        })
    })
}