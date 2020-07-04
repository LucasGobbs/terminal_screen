use crate::buffer::Buffer;
pub trait ContainerDrawable {
    fn get_buffer(&mut self) -> (Box<&Buffer>,i32,i32);
    fn get_position(&mut self) -> Option<(i32,i32)>;
    fn get_size(&mut self) -> (i32,i32);
    fn generate(&mut self) -> (Buffer,i32,i32);
}