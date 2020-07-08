use tetra::{Context, ContextBuilder, State};
use crate::buffer::Buffer;
pub trait Backend{
    fn draw(&mut self,ctx: &mut Context,buf: Buffer);
    fn clear(); 

}