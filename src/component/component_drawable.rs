#[macro_use]
use crate::derive_builder::*;
use crate::buffer::Buffer;

pub trait ComponentDrawable {
    fn get_buffer(&self) -> Box<&Buffer>;
    fn get_position(self) -> (i32,i32);
    fn get_size(&mut self) -> (i32,i32);
    fn generate(&mut self) -> (Buffer,i32,i32);
}


#[derive(Builder, Clone)]
pub struct Component {
    pub pos: (i32,i32),
    pub size: (i32,i32),
    pub data: Option<Buffer>,
    pub changed: bool,
}