
use crate::buffer::Buffer;

pub trait ComponentDrawable {
    fn get_buffer(&mut self) -> (Box<&Buffer>,i32,i32);
    fn get_position(&mut self) -> Option<(i32,i32)>;
    fn get_size(&mut self) -> (i32,i32);
    fn generate(&mut self) -> (Buffer,i32,i32);
}

struct ComponentFields {
    pub pos: (i32,i32),
    pub size: (i32,i32),
    pub data: Option<Buffer>,
}
impl ComponentFields {
    pub fn new() -> ComponentFields {
        ComponentFields {
            pos: (0,0),
            size: (0,0),
            data: None,
        }
    }
    pub fn position(mut self, x: i32, y:i32) -> Self{
        self.pos.0 = x;
        self.pos.1 = y;
        self
    }
    pub fn size(mut self, width: i32, height: i32) -> Self{
        self.size.0 = width;
        self.size.1 = height;
        self
    }
}