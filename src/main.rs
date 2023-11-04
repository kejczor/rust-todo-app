mod structs;
mod utils;

use structs::{TodoItem, ID};
use utils::{clear, input, load_todos};

pub type Todos = Vec<TodoItem>;

fn add_todo(todos: &mut Todos, id: u32) {
    let title = input("Type the title of todo: ", true, true);

    todos.push(TodoItem { id, title });
}

fn remove_todo(todos: &mut Todos) {
    let input_id = input("Please type index of element to remove: ", true, true);

    if let Some(index) = todos.iter().position(|x| x.id.to_string() == input_id) {
        todos.remove(index);
    }
}

fn main() {
    let mut id_generator = ID::new();
    let mut todos = Vec::new();

    load_todos(&mut todos);

    println!("Welcome to TODO APP");
    loop {
        clear();
        println!("1 Add todo   |   2 Remove todo   |   q Quit");
        println!("-------------------------------------------");
        todos
            .iter()
            .for_each(|todo| println!("  {} -> {}", todo.id, todo.title));
        println!("-------------------------------------------");
        let choice = input("Choose option: ", true, true);

        match choice.as_str() {
            "1" => add_todo(&mut todos, id_generator.get()),
            "2" => remove_todo(&mut todos),
            "q" => return,
            _ => println!("Unsupported choice"),
        }
    }
}
