use serde::Deserialize;

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

#[derive(Deserialize)]
pub struct TodoItem {
    pub id: u32,
    pub title: String,
}
