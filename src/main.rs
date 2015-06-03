extern crate rustbox;

use std::default::Default;

use rustbox::{Color, RustBox};
use rustbox::Key;
use rustbox::Mouse;

fn main() {
    let rustbox = match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };

    let mut x = 1;
    let mut y = 2;

    rustbox.print(1, 1, rustbox::RB_BOLD, Color::Blue, Color::Black, "Hello, world!");
    rustbox.print(1, 3, rustbox::RB_BOLD, Color::White, Color::Black,
                  "Press 'q' to quit.");
    loop {
        rustbox.print(x, y, rustbox::RB_BOLD, Color::White, Color::Black, "@");
        rustbox.present();

        match rustbox.poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                rustbox.clear();
                match key {
                    Some(Key::Left) => { x -= 1; },
                    Some(Key::Right) => { x += 1; },
                    Some(Key::Up) => { y -= 1; },
                    Some(Key::Down) => { y += 1; },
                    Some(Key::Char('q')) => { break; },
                    _ => {
                        rustbox.print(20, 5, rustbox::RB_BOLD, Color::Red, Color::Black, "You pressed a key!");
                    }
                }
            },

            Ok(rustbox::Event::MouseEvent(mouse, x, y)) => {
                match mouse{
                    Mouse::Left => {
                        rustbox.print(2, 1, rustbox::RB_BOLD, Color::Red, Color::Black, "CLICK! CLIKGINGNIG!");
                    }
                    _ => { }
                }
            },

            Ok(rustbox::Event::ResizeEvent(x, y)) => {
                rustbox.print(30, 10, rustbox::RB_BOLD, Color::Red, Color::Black, &format!("{}, {}", x, y));
            }
            Err(e) => panic!("{}", e),
            _ => { }
        }

    }
}

