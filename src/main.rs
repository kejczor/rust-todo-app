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

fn add_todo(todos: &mut Vec<TodoItem>, id: u32) {
    let title = input("Type the title of todo: ", true, true);

    todos.push(TodoItem { id, title });
}

fn main() {
    let mut todos: Vec<TodoItem> = Vec::new();

    let mut id_generator = ID::new();

    println!("Welcome to TODO APP");
    loop {
        let choice = input(
            "\
        Choose one of the options:
        1. Display todos
        2. Add a todo
        q  Quit
        ",
            true,
            true,
        );

        match choice.as_str() {
            "1" => println!(
                "{}",
                todos
                    .iter()
                    .map(|todo| todo.title.clone())
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            "2" => add_todo(&mut todos, id_generator.get()),
            "q" => return,
            _ => println!("Unsupported choice"),
        }
    }
}
