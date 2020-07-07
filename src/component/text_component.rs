use crate::component::{ComponentDrawable,ComponentBuilder, Component};
use crate::buffer::Buffer;

use tetra::graphics::Color;

#[derive(Clone)]
pub struct TextComponent{
    pub component: Component,
    text: Vec<(String, Color)>,
   
}
impl TextComponent {
    pub fn new(builder: &mut ComponentBuilder) -> Self{
        TextComponent{
            component: builder.build().unwrap(),
            text: Vec::new(),
        }
    }
    pub fn add_text(&mut self, word: &str, color: Color) ->&mut Self{
        self.text.push((String::from(word),color));
        
        self
    }
    fn generate_size(&mut self){
        let size = self.text.iter()
                                   .fold(0, 
                                            |size, tuple| 
                                            size + tuple.0.len());
        self.component.size = (size as i32,1);
    }
}
impl ComponentDrawable for TextComponent {
    fn get_buffer(self) -> Buffer{
        let size = (self.component.size.0 as usize, 
                                    self.component.size.1 as usize);
        let mut n_buf = Buffer::new(size.0, size.1);
        for cell in self.component.data.unwrap().data {
            n_buf.data.push(cell);
        }
        n_buf
    }
    fn get_position(self) -> (i32,i32){
        self.component.pos
    }
    fn get_size(&mut self) -> (i32,i32){
        self.component.size
    }
    fn generate(&mut self){
        
        let pos = self.component.pos;

        self.generate_size();
        let size = self.component.size;
        
        let mut buf = Buffer::new(size.0 as usize,size.1 as usize);
        let mut index = 0;
        for (word,color) in self.text.iter() {
            for (i, ch) in word.chars().enumerate() {
                buf.set_char((index + i) as i32, 0, ch, *color);

            }
            index += word.len();
        }
        self.component.data = Some(buf);
       // (buf, pos.0, pos.1)
    }
}