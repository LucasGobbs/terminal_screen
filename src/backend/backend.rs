
use crate::buffer::Buffer;
pub trait Backend{
    type Context;
    fn draw(&mut self, buf: Buffer, ctx: &mut Self::Context);
    fn clear(); 

}
