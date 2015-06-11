use entity::Entity;

use std::default::Default;
use std::io::Read;

#[derive(Default)]
pub struct Snell {
    pub width: usize,
    pub height: usize,
    pub entities: Vec<Entity>
}

impl Snell {
    pub fn new() -> Snell{
        let e: Vec<Entity> = Vec::new();
        Snell{entities: e, ..Default::default()}
    }
    ///Initializes a snell from a string, using newlines to determine
    ///the width of the snell (I think this is horrible)
    pub fn load(mut self, reader: &String) -> Option<Snell>{
        let mut x = 0;
        let mut y = 0;
        for c in reader.chars(){
            match c {
                '\n' | '\r' => {
                    if x > self.width{
                        self.width = x;
                    }
                    x = 0;
                    y += 1;
                },
                _ => {
                    x += 1;
                    let e_result = Entity::new(x, y).from_char(c);
                    match e_result {
                        Ok(e) => self.entities.push(e),
                        Err(error) => {
                            println!("error while parsing snell: {}", error);
                        }
                    }
                }
            }
        }
        Some(self)
    }
}
