use crate::component::{ComponentDrawable, Component};
use crate::buffer::Buffer;
use tetra::graphics::Color;

#[derive(Clone)]
pub struct DividerComponent {
    component: Component,
    centered: bool,
    horizontal: bool,
    corners: char,
    line: char,
    center: char,
}
impl DividerComponent {
    pub fn new(cmp: Component) -> Self {
        DividerComponent {
            component: cmp,
            centered:false,
            horizontal:true,
            corners: '+',
            line: '-',
            center: '@',
        }
    }
    pub fn centered(&mut self) -> &mut Self{
        self.centered = true;
        self
    }
    pub fn vertical(&mut self) -> &mut Self{
        self.horizontal = false;
        self
    }
    pub fn line_char(&mut self, ch: char) -> &mut Self{
        self.line = ch;
        self
    }
    pub fn corner_char(&mut self, ch: char) -> &mut Self{
        self.corners = ch;
        self
    }
    pub fn center_char(&mut self, ch: char) -> &mut Self{
        self.center = ch;
        self
    }
}
impl ComponentDrawable for DividerComponent {
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
    // fn get_size(&mut self) -> (i32,i32){
    //     self.component.size
    // }
    fn generate(&mut self){
        
        let size = self.component.size;
        let actual_size: (i32,i32);
        let max_size = if self.component.size.0 > self.component.size.1 {
            self.component.size.0
        } else {
            self.component.size.1
        };
        let increment: (i32,i32);
        if self.horizontal {
            increment = (1,0);
            actual_size = (max_size,1);
        }else {
            increment = (0,1);
            actual_size = (1,max_size);
        }
        //print!("{:?}\n",self.component.pos);
        self.component.size = actual_size;
        let mut buf = Buffer::new(actual_size.0 as usize, actual_size.1 as usize);
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
        self.component.data = Some(buf);
    }
}