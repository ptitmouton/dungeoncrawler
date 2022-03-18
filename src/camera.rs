use crate::prelude::*;

pub struct Camera {
    pub left_x: i32,
    pub right_x: i32,
    pub top_y: i32,
    pub bottom_y: i32,
}

impl Camera {
    pub fn new(position: Point) -> Self {
        Self {
            left_x: position.x - DISPLAY_WIDTH / 2,
            right_x: position.x + DISPLAY_WIDTH / 2,
            top_y: position.y - DISPLAY_HEIGHT / 2,
            bottom_y: position.y + DISPLAY_HEIGHT / 2,
        }
    }

    pub fn move_camera(&mut self, position: Point) {
        self.left_x = position.x - DISPLAY_WIDTH / 2;
        self.right_x = position.x + DISPLAY_WIDTH / 2;
        self.top_y = position.y - DISPLAY_HEIGHT / 2;
        self.bottom_y = position.y + DISPLAY_HEIGHT / 2;
    }
}
