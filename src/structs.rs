use crate::conts::FILENAME;
use crate::Todos;

use serde::{Deserialize, Serialize};
use std::{fs::File, io::BufWriter, io::Write};

#[derive(Serialize, Deserialize)]
pub struct ID {
    id: u32,
}

impl ID {
    pub fn new() -> ID {
        ID { id: 0 }
    }

    pub fn get(&mut self) -> u32 {
        self.id += 1;
        self.id
    }
}

#[derive(Serialize, Deserialize)]
pub struct TodoItem {
    pub id: u32,
    pub title: String,
}

#[derive(Serialize, Deserialize)]
pub struct Appdata {
    id_generator: ID,
    todos: Todos,
}

impl Appdata {
    pub fn load(todos: &mut Todos, id_generator: &mut ID) {
        if let Ok(file) = File::open(FILENAME) {
            if let Ok(json) = serde_json::from_reader::<_, Appdata>(file) {
                *todos = json.todos;
                *id_generator = json.id_generator;
            }
        }
    }

    pub fn save(todos: Todos, id_generator: ID) {
        let file = File::create(FILENAME).unwrap();
        let appdata = Appdata {
            id_generator,
            todos,
        };
        let mut writer = BufWriter::new(file);
        serde_json::to_writer(&mut writer, &appdata).unwrap();
        writer.flush().unwrap();
    }
}
