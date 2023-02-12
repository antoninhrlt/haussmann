use crate::graphics::shapes;
use crate::graphics::Size;
use crate::Border;

/// Implementation for a rectangle shape.
impl shapes::Builder<4> {
    pub fn rectangle(&mut self, size: Size, borders: Option<[Border; 4]>) -> &mut Self {
        self.create([
            [0, 0],
            [size[0] as isize, 0],
            [size[0] as isize, size[1] as isize], 
            [0, size[1] as isize],
        ], borders)
    }
}
