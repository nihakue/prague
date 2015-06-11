#![feature(io)]

extern crate rustbox;

mod entity;
mod snell;
mod draw;

use entity::Entity;
use entity::EntityType::*;
use snell::Snell;
use draw::Draw;

use rustbox::{Color, RustBox, RB_BOLD};
use rustbox::Key;

use std::fs::File;
use std::io::prelude::*;


fn main() {
    let mut lvl = String::new();
    let mut f = File::open("testlevel.txt").unwrap();
    f.read_to_string(&mut lvl);

    let rb = match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };
    let mut p1 = Entity::new(1, 1).kind(Player);
    let test_snell = Snell::new().load(&lvl).expect("Error");

    loop {
        test_snell.draw(&rb);
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

