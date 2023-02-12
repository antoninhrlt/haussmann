use crate::graphics::shapes;
use crate::graphics::{ Point, Size };
use crate::Border;

/// Implementation for a rectangle shape.
impl shapes::Builder<4> {
    pub fn rectangle(&mut self, position: Point, size: Size, borders: Option<[Border; 4]>) -> &mut Self {
        self.create([
            // top left
            [position[0], position[1]],
            // top right
            [position[0] + size[0] as isize, position[1]],
            // bottom right
            [position[0] + size[0] as isize, position[1] + size[1] as isize], 
            // bottom left
            [position[0], position[1] + size[1] as isize],
        ], borders)
    }
}
