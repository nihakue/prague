use std::str::FromStr;
use std::fmt::{Display, Formatter, Error};
use std::default::Default;
use self::EntityType::*;


#[derive(Clone, Copy)]
pub enum EntityType {
    Player,
    Ground,
    Wall
}

pub enum EntityError {
    UnknownEntity,
    InvalidEntity
}

impl Default for EntityType {
    fn default() -> EntityType {EntityType::Ground}
}

#[derive(Clone, Copy, Default)]
pub struct Entity {
    pub kind: EntityType,
    pub x: usize,
    pub y: usize
}

impl Display for Entity {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        Display::fmt(match self.kind {
            Player => "Ů",
            Wall   => "𝌆",
            Ground => ".",
        }, f)
    }
}

// impl FromStr for Entity {
//     type Err = &'static str;

//     fn from_str(s: &str) -> Result<Entity, &'static str> {
//         match s{
//             'Ů' => Ok(Entity{kind: Player, ..Default::default()}),
//             '.' => Ok(Entity{kind: Ground, ..Default::default()}),
//             '𝌆' => Ok(Entity{kind: Wall, ..Default::default()}),
//             _   => Err("invalid string (unkown entity type)"),
//         }
//     }
// }

impl Entity {
    pub fn new(x: usize, y: usize) -> Entity{
        Entity{x: x, y: y, ..Default::default()}
    }
    pub fn from_char(mut self, c: char) -> Result<Entity, String>{
        match c{
            'Ů' => {self.kind = Player; Ok(self)},
            '.' => {self.kind = Ground; Ok(self)},
            '𝌆' => {self.kind = Wall; Ok(self)},
            _   => Err(format!("unknown entity for: {}", c))
        }
    }
    pub fn kind(mut self, kind: EntityType) -> Entity{
        self.kind = kind;
        self
    }
}
