use tetra::graphics::Color;
use itertools::izip;
use crate::shape::Shape;
#[derive(Clone, Copy, Debug)]
pub struct ConsoleCell {
    pub glyph: char,
    // pub scale: (f32,f32),
    pub foreground: Color,
    pub background: Color,
}
#[derive(Clone)]
pub struct Buffer {
    pub data: Vec<ConsoleCell>,
   // x: i32,
   // y: i32,
    width: usize,
    height: usize,
    size: usize,
}
#[allow(dead_code)]
impl Buffer {
    pub fn new(width: usize, height: usize) -> Buffer {
        Buffer {
            data: vec![ConsoleCell {
                            glyph: ' ',
                            //scale: (1.0,1.0),
                            foreground: Color::rgb(1.0, 1.0, 1.0),
                            background: Color::rgb(0.0, 0.0, 0.0),
                        };
                width * height // 80 * 50
            ],
            width,
            height,
            size: width * height,
        }
    }
    pub fn clear(&mut self){
        
        for cell in &mut self.data {
            cell.glyph = ' ';
            cell.foreground = Color::rgb(1.0, 1.0, 1.0);
            cell.background = Color::rgb(0.0, 0.0, 0.0);
        }
    }
    pub fn set_char(&mut self, x: i32, y: i32, glyph: char, color: Color){
        if x < self.width as i32 && y < self.height as i32 && x >= 0 && y>= 0 {
            self.data[(x + self.width as i32 * y) as usize].glyph = glyph;
            self.data[(x + self.width as i32 * y) as usize].foreground = color;        
        }
    }
    pub fn get_char(&mut self, x: i32, y: i32) -> Option<ConsoleCell>{
        if self.check_bound(x, y){
            return Some(self.data[(x + self.width as i32 * y) as usize]);       
        }else{
            return None;
        }
    }
    pub fn check_bound(&mut self, x: i32, y: i32) -> bool{
        return x < self.width as i32 && y < self.height as i32 && x >= 0 && y>= 0;
    }
    pub fn set_string(&mut self, x: i32, y: i32, text: &str, color: Color){
        let text_string = String::from(text);
        for (i,c) in text_string.chars().enumerate() {
            self.set_char(x + (i as i32), y, c, color);
        }
    }
    pub fn g_draw<T>(&mut self, shape: T, glyph: char, color: Color)
    where T: Shape{
        let cells = shape.get_cells();

        for cell in cells {
            self.set_char(cell.0, cell.1, glyph, color);
        }
    }
    pub fn g_draw_c<T>(&mut self, shape: T, glyph: char,color: impl Fn(i32, i32, i32, i32) -> Color)
    where T: Shape{
        let cells = shape.get_cells();
        for (index,cell) in cells.iter().enumerate() {
            let color_t = color(cells.len() as i32, index as i32, cell.0, cell.1);
            self.set_char(cell.0, cell.1, glyph,color_t);
        }
    }
    pub fn flood_fill(&mut self, x: i32, y: i32, glyph: char, color: Color){

        if self.check_bound(x,y){
            match self.get_char(x, y) {
                Some(cell) => self.flood_fill_iterative(x,y,cell.glyph,glyph,color),
                None => {},
            }
        }
        
    }
    pub fn flood_fill_rec(&mut self, x: i32, y: i32, glyph: char, color: Color){
        if self.check_bound(x,y){
            match self.get_char(x, y) {
                Some(cell) => self.flood_fill_scanline_recursive(x,y,cell.glyph,glyph,color),
                None => {},
            }
        }
        
    }
    pub fn flood_fill_naive_rec(&mut self, x: i32, y: i32, glyph: char, color: Color){
        if self.check_bound(x,y){
            match self.get_char(x, y) {
                Some(cell) => self.flood_fill_naive_recursive(x,y,cell.glyph,glyph,color),
                None => {},
            }
        }
    }
    fn flood_fill_iterative(&mut self, x: i32, y: i32, from_glyph: char, to_glyph: char, color: Color ){
        if self.check_bound(x, y){
            let mut stack: Vec<(i32,i32)> = Vec::new();

            stack.push((x,y));
            loop {
                let (xi, yi) = stack.pop().unwrap();
             
                if self.check_bound(xi, yi) {
                    match self.get_char(xi, yi) {
                        Some(cell) => {
                            if cell.glyph == from_glyph {
                                self.set_char(xi, yi, to_glyph, color);
 
                                stack.push((xi+1,yi));
                                stack.push((xi-1,yi));
                                stack.push((xi,yi+1));
                                stack.push((xi,yi-1));
                            }
                        }
                        None => {},
                    }
                }
                if stack.len() == 0 {
                    break
                }
            }
        }

   
    }
    fn flood_fill_naive_recursive(&mut self, x: i32, y: i32, from_glyph: char, to_glyph: char, color: Color){
       
        
        if self.check_bound(x, y) {

            match self.get_char(x, y) {
                Some(cell) => {
                    if cell.glyph == from_glyph {
                     
                        self.set_char(x, y, to_glyph, color);
                      //  self.set_char(x+1, y, to_glyph, color);
                       // self.set_char(x-1, y, to_glyph, color);
                       // self.set_char(x, y+1, to_glyph, color);
                       // self.set_char(x, y-1, to_glyph, color);
                        //    -     #-
                        //  - # -  ###-
                        //    -     #
                        self.flood_fill_naive_recursive(x+1,y,from_glyph,to_glyph, color);
                        self.flood_fill_naive_recursive(x-1,y,from_glyph,to_glyph, color);
                        self.flood_fill_naive_recursive(x,y+1,from_glyph,to_glyph, color);
                        self.flood_fill_naive_recursive(x,y-1,from_glyph,to_glyph, color);
                    }
                }
                None => {},
            }
            
        } 
    }
    fn flood_fill_scanline_recursive(&mut self, x: i32, y: i32, from_glyph: char, to_glyph: char, color: Color){

        if self.check_bound(x, y){
            match self.get_char(x, y) {
                Some(cell) => {
                 
                    if cell.glyph == from_glyph {
                     
                        //* Desenha linha pra direita
                        let mut x1 = x;
                        loop{
                          

                            if !self.check_bound(x1, y) {
                           
                                break;
                            }
                            match self.get_char(x1,y) {
                                Some(cell)=> if cell.glyph != from_glyph {
                                  
                                    break;
                                },
                                _ => {},
                            }
                            self.set_char(x1, y, to_glyph, color);
                            x1+=1;
                        }

                        //* Desenha linha pra esquerda
                        x1 = x - 1 ;
                        loop{
                            if !self.check_bound(x1, y) {
                                break;
                            }
                            match self.get_char(x1,y) {
                                Some(cell)=> if cell.glyph != from_glyph {
                                    break;
                                },
                                _ => {},
                            }
                            
                            self.set_char(x1, y, to_glyph, color);
                            x1-=1;
                        }
                      
                         //test for new scanlines above
                        x1 = x;
                        loop{
                            if !self.check_bound(x1, y) {
                                break;
                            }
                            match self.get_char(x1,y-1) {
                                Some(cell)=> if cell.glyph != from_glyph {
                                    break;
                                },
                                _ => {},
                            }
                            self.flood_fill_scanline_recursive(x1, y-1, from_glyph, to_glyph, color);
                           
                            x1+=1;
                        }

                        x1 = x - 1;
                        loop{
                            if !self.check_bound(x1, y) {
                                break;
                            }
                            match self.get_char(x1,y-1) {
                                Some(cell)=> if cell.glyph != from_glyph {
                                    break;
                                },
                                _ => {},
                            }
                            self.flood_fill_scanline_recursive(x1, y-1, from_glyph, to_glyph, color);
                           
                            x1-=1;
                        }

                        x1 = x;
                        loop{
                            if !self.check_bound(x1, y) {
                                break;
                            }
                            match self.get_char(x1,y+1) {
                                Some(cell)=> if cell.glyph != from_glyph {
                                    break;
                                },
                                _ => {},
                            }
                            self.flood_fill_scanline_recursive(x1, y+1, from_glyph, to_glyph, color);
                           
                            x1+=1;
                        }
                        x1 = x - 1;
                        loop{
                            if !self.check_bound(x1, y) {
                                break;
                            }
                            match self.get_char(x1,y+1) {
                                Some(cell)=> if cell.glyph != from_glyph {
                                    break;
                                },
                                _ => {},
                            }
                            self.flood_fill_scanline_recursive(x1, y+1, from_glyph, to_glyph, color);
                           
                            x1-=1;
                        }
                       
                    }
                }
                None => {},
            }
        }
    }
    pub fn sub_assign(&mut self, other: Buffer){
        for (mut cell_self,cell_other) in izip!(&mut self.data, other.data){
            if cell_other.glyph != ' '{
                cell_self.glyph = ' ';
                cell_self.foreground = Color::rgb(1.0, 1.0, 1.0);
                cell_self.background = Color::rgb(0.0, 0.0, 0.0);
            }
        }
    }
    pub fn add_assign(&mut self, other: Buffer){
        for (mut cell_self,cell_other) in izip!(&mut self.data, other.data){
            if cell_self.glyph == ' ' && cell_other.glyph != ' '{
                cell_self.glyph = cell_other.glyph;
                cell_self.foreground = cell_other.foreground;
                cell_self.background = cell_other.background;
            }
        }
    }
}