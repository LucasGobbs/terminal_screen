
use crate::buffer::Buffer;

pub trait ComponentDrawable {
    fn get_buffer(self) -> Buffer;
    fn get_position(self) -> (i32,i32);
    // fn get_size(&mut self) -> (i32,i32);
    fn generate(&mut self) ;
}


#[derive(Clone)]
pub struct Component {
    pub pos: (i32,i32),
    pub size: (i32,i32),
    pub data: Option<Buffer>,
    pub changed: bool,
}
impl Component {
    pub fn default() -> Component {
        Component {
            pos: (0,0),
            size: (0,0),
            data: None,
            changed: false,
        }
    }
    pub fn new(x: i32, y: i32) -> Component {
        Component {
            pos: (x,y),
            size: (0,0),
            data: None,
            changed: false,
        }
    }
    pub fn pos(mut self, x: i32, y: i32) -> Self{
        self.pos = (x,y);
        self
    }
    pub fn size(mut self, width: i32, height: i32) -> Self {
        self.size = (width,height);
        self
    }
}