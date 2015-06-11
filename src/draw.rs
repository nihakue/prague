use entity::Entity;
use snell::Snell;
use rustbox::{Color, RustBox, Key, RB_BOLD};

pub trait Draw {
    fn draw(&self, &RustBox);
}

impl Draw for Entity {
    fn draw(&self, r: &RustBox) {
        r.print(self.x, self.y, RB_BOLD, Color::White, Color::Black,
            &format!("{}", self));
    }
}

impl Draw for Snell {
    fn draw(&self, r: &RustBox){
        for e in &self.entities{
            e.draw(r);
        }
    }
}
