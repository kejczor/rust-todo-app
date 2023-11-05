mod conts;
mod structs;
mod utils;

use structs::{TodoItem, ID};
use utils::{clear, input};

use crate::structs::Appdata;

pub type Todos = Vec<TodoItem>;

fn add_todo(todos: &mut Todos, id: u32) {
    let title = input("Type the title of todo: ", true, true);

    todos.push(TodoItem { id, title });
}

fn remove_todo(todos: &mut Todos) {
    let input_id = input("Please type number of element to remove: ", true, true);

    // if let Some(index) = todos.iter().position(|x| x.id.to_string() == input_id) {
    //     todos.remove(index);
    // }
    todos.remove(input_id.parse().unwrap());
}

fn main() {
    let mut id_generator = ID::new();
    let mut todos = Vec::new();

    Appdata::load(&mut todos, &mut id_generator);

    println!("Welcome to TODO APP");
    loop {
        clear();
        println!("1 Add todo   |   2 Remove todo   |   q Quit");
        println!("-------------------------------------------");
        todos
            .iter()
            .enumerate()
            .for_each(|(i, todo)| println!("  {} -> {}", i, todo.title));
        println!("-------------------------------------------");
        let choice = input("Choose option: ", true, true);

        match choice.as_str() {
            "1" => add_todo(&mut todos, id_generator.get()),
            "2" => remove_todo(&mut todos),
            "q" => return Appdata::save(todos, id_generator),
            _ => continue,
        }
    }
}
