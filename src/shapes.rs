#[derive(Debug)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
}

impl Rect {
    pub fn build(x: i32, y: i32) -> Rect {
        Rect {
            x,
            y
        }
    }
    pub fn area(&self) -> i32 {
        self.x * self.y
    }
}
