use crate::container::ContainerDrawable;
use crate::buffer::Buffer;
use tetra::graphics::Color;
pub struct TextContainer{
    data: Option<Buffer>,
    pos: Option<(i32,i32)>,
    text: Option<String>,
}
impl TextContainer {
    pub fn new() -> TextContainer{
        TextContainer{
            data: None,
            pos: Some((0,0)),
            text: None,
        }
    }
    pub fn pos(&mut self, x: i32, y: i32) -> &mut TextContainer {
        self.pos = Some((x,y));
        self
    }
    pub fn text(&mut self, text: String) -> &mut TextContainer {
        self.text = Some(text);
       
        self
    }
}
impl ContainerDrawable for TextContainer {
    fn get_buffer(&mut self) -> (Box<&Buffer>,i32,i32){
        let cl = Box::new(self.data.as_ref().unwrap());

        (cl,self.pos.unwrap().0,self.pos.unwrap().1)
    }
    fn get_position(&mut self) -> Option<(i32,i32)>{
        self.pos
    }
    fn get_size(&mut self) -> (i32,i32){
        (self.text.as_ref().unwrap().len() as i32,1)
    }
    fn generate(&mut self) -> (Buffer,i32,i32){
        let pos = self.pos.unwrap();
        let size = self.get_size();
        let mut buf = Buffer::new(size.0 as usize,size.1 as usize);
        for (i, ch) in self.text.as_ref().unwrap().chars().enumerate() {
            buf.set_char(i as i32, 0,ch, Color::RED);
            print!("x: {}+{}|y: {}| {}| size: {} {}\n",pos.0,i,pos.1,ch,size.0,size.1);
        }
        //buf.print();
        (buf, pos.0, pos.1)
    }
}