use crate::buffer::{Buffer};

#[derive(Clone)]
pub struct BufferApp {
    pub buffers: Vec<Buffer>,
    pub current_buffer: usize,
    size: (i32,i32),
}
impl BufferApp {
    pub fn new(width: i32, height: i32) -> BufferApp {
        let mut buffers: Vec<Buffer> = Vec::new();
        buffers.push(Buffer::new(width as usize, height as usize));
        BufferApp{
            buffers,
            current_buffer: 0,
            size: (width, height),
        }
    }
    pub fn buf(&mut self) ->&mut Buffer {
        &mut self.buffers[self.current_buffer]
    }
    pub fn draw(self, mx: i32) -> Buffer {
        let n_buf = self.buffers[self.current_buffer].clone();
        
        n_buf
    }
    pub fn add_buffer(mut self, n_buf: Buffer){
        self.buffers.push(n_buf);
    }
}