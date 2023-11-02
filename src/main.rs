use std::fs::File;

const FILENAME: &str = "todos.json";

fn input(msg: &str, expect_length: bool, remove_newline_char: bool) -> String {
    println!("{}", msg);
    let mut buf = String::new();
    loop {
        std::io::stdin().read_line(&mut buf).unwrap();
        if !expect_length {
            break buf;
        };
        if buf.len() > 0 {
            break if remove_newline_char {
                String::from(
                    buf.as_str()
                        .strip_suffix("\r\n")
                        .or(buf.strip_suffix("\n"))
                        .unwrap_or(buf.as_str()),
                )
            } else {
                buf
            };
        } else {
            println!("Please type something...");
        }
    }
}

fn clear() {
    print!("{esc}c", esc = 27 as char);
}

struct ID {
    id: u32,
}

impl ID {
    fn new() -> ID {
        ID { id: 0 }
    }

    fn get(&mut self) -> u32 {
        self.id += 1;
        self.id
    }
}

struct TodoItem {
    id: u32,
    title: String,
}

fn load_todos(todos: &mut Vec<TodoItem>) {
    let mut file = match File::open(FILENAME) {
        Err(why) => return,
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    // let mut s = String::new();
    // match file.read_to_string(&mut s) {
    //     Err(why) => panic!("couldn't read {}: {}", display, why),
    //     Ok(_) => print!("{} contains:\n{}", display, s),
    // }
}

fn add_todo(todos: &mut Vec<TodoItem>, id: u32) {
    let title = input("Type the title of todo: ", true, true);

    todos.push(TodoItem { id, title });
}

fn remove_todo(todos: &mut Vec<TodoItem>) {
    let input_id = input("Please type index of element to remove: ", true, true);

    if let Some(index) = todos.iter().position(|x| x.id.to_string() == input_id) {
        todos.remove(index);
    }
}

fn main() {
    let mut todos: Vec<TodoItem> = Vec::new();

    let mut id_generator = ID::new();

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
