extern crate rustbox;

use std::default::Default;

use std::str::FromStr;
use std::fmt::{Display, Formatter};

use rustbox::{Color, RustBox, RB_BOLD};
use rustbox::Key;

use self::EntityType::*;

#[derive(Clone, Copy)]
enum EntityType {
    Player,
    Ground,
    Wall
}

impl Default for EntityType {
    fn default() -> EntityType {EntityType::Ground}
}

#[derive(Clone, Copy, Default)]
struct Entity {
    kind: EntityType,
    x: usize,
    y: usize
}

impl Display for Entity {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        Display::fmt(match self.kind {
            Player => "Ů",
            Wall   => "|",
            Ground => "."
        }, f)
    }
}

impl FromStr for Entity {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Entity, &'static str> {
        match s{
            "Ů" => Ok(Entity{kind: Player, ..Default::default()}),
            "." => Ok(Entity{kind: Ground, ..Default::default()}),
            "|" => Ok(Entity{kind: Wall, ..Default::default()}),
            _   => Err("invalid string (unkown entity type)"),
        }
    }
}

impl Entity {
    fn new(x: usize, y: usize) -> Entity{
        Entity{x: x, y: y, ..Default::default()}
    }
    fn from_char(mut self, c: char) -> Result<Entity, String>{
        match c{
            'Ů' => {self.kind = Player; Ok(self)},
            '.' => {self.kind = Ground; Ok(self)},
            '|' => {self.kind = Wall; Ok(self)},
            _   => Err(format!("unknown entity for: {}", c))
        }
    }
    fn kind(mut self, kind: EntityType) -> Entity{
        self.kind = kind;
        self
    }
}

struct Snell {
    width: usize,
    height: usize,
    entities: Vec<Entity>
}

// impl FromStr for Snell {
//     type Err = &'static str;

//     fn from_str(s: &str) -> Result<Snell, &'static str> {

//     }
// }

trait Draw {
    fn draw(&self, &RustBox);
}

impl Draw for Entity {
    fn draw(&self, r: &RustBox) {
        r.print(self.x, self.y, RB_BOLD, Color::White, Color::Black,
            &format!("{}", self));
    }
}

fn main() {
    let lvl1 = "............................................................
            ............................................................
            ............................................................
            ............................................................
            ............................................................
            ............................................................
            ............................................................
            ............................................................
            ............................................................
            ............................................................
            ............................................................
            ............................................................
            ............................................................
            ............................................................
            ............................................................
            ............................................................
            ............................................................
            ............................................................
            ............................................................
            ............................................................".to_string();

    let rb = match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };

    let mut p1 = Entity::new(1, 1).kind(Player);
    let world = [Entity{kind: Wall, ..Default::default()}; 100];

    loop {
        for e in world.iter() {
            e.draw(&rb);
        }
        p1.draw(&rb);
        rb.present();

        match rb.poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                rb.clear();
                match key {
                    Some(Key::Left) => { p1.x -= 1; },
                    Some(Key::Right) => { p1.x += 1; },
                    Some(Key::Up) => { p1.y -= 1; },
                    Some(Key::Down) => { p1.y += 1; },
                    Some(Key::Char('q')) => { break; },
                    _ => {
                        rb.print(20, 5, rustbox::RB_BOLD, Color::Red, Color::Black, "You pressed a key!");
                    }
                }
            },

            Ok(rustbox::Event::ResizeEvent(x, y)) => {
                rb.print(30, 10, rustbox::RB_BOLD, Color::Red, Color::Black, &format!("{}, {}", x, y));
            }
            Err(e) => panic!("{}", e),
            _ => { }
        }

    }
}

// TODO Move this into the Snell
// fn load_level(lvl: String) -> Snell {
//     let width = 60;
//     let height = 20;
//     let mut l = [Entity::new(0, 0); 1240];
//     for x in 0..width{
//         for y in 0..height{
//             let i = width * y + x;
//             let c = lvl.char_at(i);
//             l[i] = Entity::new(x, y).from_char(lvl.char_at(i)).unwrap();
//         }
//     }
//     return l;
// }

