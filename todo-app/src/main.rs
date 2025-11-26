use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

#[derive(Clone, PartialEq)]
struct TodoItem {
    id: usize,
    text: String,
    completed: bool,
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut todos = use_signal(|| vec![
        TodoItem { id: 0, text: "Learn Dioxus".to_string(), completed: false },
        TodoItem { id: 1, text: "Build a desktop app".to_string(), completed: false },
    ]);
    let mut input_value = use_signal(|| String::new());
    let mut next_id = use_signal(|| 2);
    let mut filter = use_signal(|| "all".to_string());

    // Add new todo
    let add_todo = move |_evt: Event<MouseData>| {
        let text = input_value.read().trim().to_string();
        if !text.is_empty() {
            let id = *next_id.read();
            todos.write().push(TodoItem {
                id,
                text: text.clone(),
                completed: false,
            });
            *next_id.write() = id + 1;
            *input_value.write() = String::new();
        }
    };

    // Toggle todo completion
    let mut toggle_todo = move |id: usize| {
        todos.write().iter_mut().for_each(|todo| {
            if todo.id == id {
                todo.completed = !todo.completed;
            }
        });
    };

    // Delete todo
    let mut delete_todo = move |id: usize| {
        todos.write().retain(|todo| todo.id != id);
    };

    // Clear completed
    let clear_completed = move |_evt: Event<MouseData>| {
        todos.write().retain(|todo| !todo.completed);
    };

    // Filter todos
    let filtered_todos = use_memo(move || {
        let all_todos = todos.read();
        let current_filter = filter.read();
        match current_filter.as_str() {
            "active" => all_todos.iter().filter(|t| !t.completed).cloned().collect::<Vec<_>>(),
            "completed" => all_todos.iter().filter(|t| t.completed).cloned().collect::<Vec<_>>(),
            _ => all_todos.clone(),
        }
    });

    let remaining_count = use_memo(move || {
        todos.read().iter().filter(|t| !t.completed).count()
    });

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        
        div {
            class: "app-container",
            
            header {
                class: "app-header",
                h1 { "📝 Todo App" }
                p { class: "subtitle", "Built with Dioxus + Rust" }
            }

            main {
                class: "main-content",
                
                // Input section
                div {
                    class: "input-section",
                    input {
                        class: "todo-input",
                        r#type: "text",
                        placeholder: "What needs to be done?",
                        value: "{input_value}",
                        oninput: move |evt| *input_value.write() = evt.value(),
                        onkeypress: move |evt| {
                            if evt.key() == Key::Enter {
                                let text = input_value.read().trim().to_string();
                                if !text.is_empty() {
                                    let id = *next_id.read();
                                    todos.write().push(TodoItem {
                                        id,
                                        text: text.clone(),
                                        completed: false,
                                    });
                                    *next_id.write() = id + 1;
                                    *input_value.write() = String::new();
                                }
                            }
                        }
                    }
                    button {
                        class: "add-button",
                        onclick: add_todo,
                        "Add"
                    }
                }

                // Filter buttons
                div {
                    class: "filter-section",
                    button {
                        class: if *filter.read() == "all" { "filter-btn active" } else { "filter-btn" },
                        onclick: move |_| *filter.write() = "all".to_string(),
                        "All"
                    }
                    button {
                        class: if *filter.read() == "active" { "filter-btn active" } else { "filter-btn" },
                        onclick: move |_| *filter.write() = "active".to_string(),
                        "Active"
                    }
                    button {
                        class: if *filter.read() == "completed" { "filter-btn active" } else { "filter-btn" },
                        onclick: move |_| *filter.write() = "completed".to_string(),
                        "Completed"
                    }
                }

                // Todo list
                div {
                    class: "todo-list",
                    if filtered_todos.read().is_empty() {
                        div {
                            class: "empty-state",
                            "🎉 No todos to show!"
                        }
                    } else {
                        { filtered_todos.read().iter().map(|todo| {
                            let todo_id = todo.id;
                            let todo_text = todo.text.clone();
                            let todo_completed = todo.completed;
                            rsx! {
                                div {
                                    key: "{todo_id}",
                                    class: if todo_completed { "todo-item completed" } else { "todo-item" },
                                    
                                    input {
                                        r#type: "checkbox",
                                        class: "todo-checkbox",
                                        checked: todo_completed,
                                        onchange: move |_| toggle_todo(todo_id)
                                    }
                                    
                                    span {
                                        class: "todo-text",
                                        "{todo_text}"
                                    }
                                    
                                    button {
                                        class: "delete-button",
                                        onclick: move |_| delete_todo(todo_id),
                                        "🗑️"
                                    }
                                }
                            }
                        }) }
                    }
                }

                // Footer
                div {
                    class: "footer",
                    span {
                        class: "todo-count",
                        "{remaining_count} item(s) left"
                    }
                    button {
                        class: "clear-button",
                        onclick: clear_completed,
                        "Clear Completed"
                    }
                }
            }
        }
    }
}
