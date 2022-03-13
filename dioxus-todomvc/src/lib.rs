mod components;

use std::collections::BTreeMap;

use dioxus::prelude::*;

use crate::components::{todo_input, todo_item, todo_filter};

#[derive(Debug, Clone, PartialEq)]
pub struct TodoItem {
    pub id: u32,
    pub title: String,
    pub completed: bool,
}

pub type Todos = BTreeMap<u32, TodoItem>;

#[derive(Debug, Clone, PartialEq)]
pub enum Filter {
    All,
    Active,
    Completed,
}

pub fn app(cx: Scope) -> Element {
    let todos = use_state(&cx, || Todos::default());
    let filter = use_state(&cx, || Filter::All);

    let filterd_todos = todos
        .iter()
        .filter(|(_, todo)| match filter.get() {
            Filter::All => true,
            Filter::Active => !todo.completed,
            Filter::Completed => todo.completed,
        })
        .map(|f| *f.0)
        .collect::<Vec<_>>();

    rsx!(cx, section {
       class: "todoapp",
       style { [include_str!("./style.css")]}
       div {
           todo_input(todos: todos)
           ul {
                class: "todo-list",
                filterd_todos.iter().map(|id| rsx!(
                   todo_item(key: "{id}", id: *id, todos: todos)
                ))
           }
           todo_filter(todos: todos, filter: filter)
       }
    })
}



