// parei em 1:25:38 
use std::env;

struct TodoItem {
    name: String,
    completed: char,
}

impl TodoItem {
    fn new(name: String) -> TodoItem {
        return TodoItem {
            name,
            completed: ' ',
        };
    }
}

struct TodoList {
    list: Vec<TodoItem>,
}

impl TodoList {
    fn new() -> TodoList {
        return TodoList { list: Vec::new() };
    }

    fn add_to_list(&mut self, name: String) {
        let todo_item = TodoItem::new(name);
        self.list.push(todo_item)
    }
}

fn main() {
    let arguments: Vec<String> = env::args().collect();
    let command = arguments[1].clone();
    let mut todo_list = TodoList::new();

    // let todo_list = TodoList::new();
    todo_list.add_to_list("Say hi to CJ".to_string());
    todo_list.add_to_list("Do something with Rust".to_string());

    if command == "get" {
        for item in todo_list.list {
            print!("[{}] - {}", item.completed, item.name);
        }
    }
}
