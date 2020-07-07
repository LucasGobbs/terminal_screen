use crate::component::{ComponentDrawable,ComponentBuilder, Component};
use crate::buffer::Buffer;
use tetra::graphics::Color;
pub struct DividerComponent {
    component: Component,
    centered: bool,
    horizontal: bool,
    corners: char,
    line: char,
    center: char,
}
impl DividerComponent {
    pub fn new(builder: ComponentBuilder) -> Self {
        DividerComponent {
            component: builder.build().unwrap(),
            centered:false,
            horizontal:true,
            corners: '+',
            line: '-',
            center: '@',
        }
    }
    pub fn centered(mut self) -> Self{
        self.centered = true;
        self
    }
    pub fn vertical(mut self) -> Self{
        self.horizontal = false;
        self
    }
    pub fn line_char(mut self, ch: char) -> Self{
        self.line = ch;
        self
    }
    pub fn corner_char(mut self, ch: char) -> Self{
        self.corners = ch;
        self
    }
    pub fn center_char(mut self, ch: char) -> Self{
        self.center = ch;
        self
    }
}
impl ComponentDrawable for DividerComponent {
    fn get_buffer(&mut self) -> (Box<&Buffer>,i32,i32){
        let cl = Box::new(self.component.data.as_ref().unwrap());

        (cl,self.component.pos.0,self.component.pos.1)
    }
    fn get_position(self) -> (i32,i32){
        self.component.pos
    }
    fn get_size(self) -> (i32,i32){
        self.component.size
    }
    fn generate(&mut self) -> (Buffer,i32,i32){
        
        let size = self.get_size();
        let mut max_size = 0;
        let mut increment: (i32,i32);
        if self.horizontal {
            max_size = size.0;
            increment = (1,0);
        }else {
            max_size = size.1;
            increment = (0,1);
        }
        let mut buf = Buffer::new(size.0 as usize,size.1 as usize);
        let mut xi = 0;
        let mut yi = 0;
        let mut index = 0;
        loop{
            if xi == 0 && yi == 0 {
                buf.set_char(xi, yi, self.corners, Color::BLUE);
            } else if xi ==  max_size -1  || yi == max_size -1  {
                buf.set_char(xi, yi, self.corners, Color::BLUE);
            } else if xi ==  max_size/2 || yi == max_size/2 {
                buf.set_char(xi, yi, self.center, Color::BLUE);
            } else {
                buf.set_char(xi, yi, self.line, Color::BLUE);
            } 


            xi += increment.0;
            yi += increment.1;
            index += 1;
            if index > max_size { break }
        }
        
       // buf.print();
        if self.centered {
            (buf, self.component.pos.0 - max_size / 2, self.component.pos.1)
        } else {
            (buf, self.component.pos.0, self.component.pos.1)
        }
        
    }
}