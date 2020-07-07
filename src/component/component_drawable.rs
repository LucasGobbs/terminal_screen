#[macro_use]
use crate::derive_builder::*;
use crate::buffer::Buffer;

pub trait ComponentDrawable {
    fn get_buffer(self) -> Buffer;
    fn get_position(self) -> (i32,i32);
    fn get_size(&mut self) -> (i32,i32);
    fn generate(&mut self) ;
}


#[derive(Builder, Clone)]
pub struct Component {
    pub pos: (i32,i32),

    #[builder(default = "(0,0)")]
    pub size: (i32,i32),
    #[builder(setter(into, strip_option), default)]
    pub data: Option<Buffer>,
    pub changed: bool,
}