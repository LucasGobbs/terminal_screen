use tetra::graphics::{self, DrawParams, Rectangle, Texture};
use tetra::math::Vec2;
use tetra::Context;
use itertools::izip;


use crate::buffer::{Buffer};

#[allow(dead_code)]
pub struct Console {
    font: Texture,
    width: usize,
    height: usize,
    pub temp_buffer: Buffer,
    pub perm_buffer: Buffer,

    //cells: Vec<ConsoleCell>,
    cell_size: f32,
}

impl Console {
    pub fn new(font: Texture, width: usize, height: usize) -> Console {
        let cell_size = (font.width() / 16) as f32;
        //let buffer_size = width * height;
        Console {
            font,
            width,
            height,  
            temp_buffer: Buffer::new(width, height),
            perm_buffer: Buffer::new(width, height),
            cell_size,
        }
    }

    pub fn clear(&mut self) {
        self.temp_buffer.clear();
        /*
        self.cells = vec![
            ConsoleCell {
                glyph: ' ',
                foreground: Color::rgb(1.0, 1.0, 1.0),
                background: Color::rgb(0.0, 0.0, 0.0),
            };
            self.buffer_size
        ];
        */
    }
    pub fn draw(&mut self, ctx: &mut Context) {
        
        for (i, (cell_perm,cell_temp)) in izip!(&self.perm_buffer.data, 
                                                &self.temp_buffer.data)
                                          .enumerate(){
            let (x, y) = (i % self.width, i / self.width);
            let sprite_x = (219 / 16) as f32 * 8.0;
            let sprite_y = (219 % 16) as f32 * 8.0;
            /*
            if cell_temp.background != Color::BLACK {
                graphics::draw(
                    ctx,
                    &self.font,
                    DrawParams::new()
                        .position(Vec2::new(
                            self.cell_size * x as f32,
                            self.cell_size * y as f32,
                        ))
                        .color(cell_temp.background)
                        .clip(Rectangle::new(sprite_x, sprite_y, 8.0, 8.0)),
                );
            }
            */
            if cell_temp.glyph != ' ' {
  
                let codepoint = cell_temp.glyph as u8;
                let sprite_x = f32::from(codepoint / 16) * 8.0;
                let sprite_y = f32::from(codepoint % 16) * 8.0;
                
                graphics::draw(
                    ctx,
                    &self.font,
                    DrawParams::new()
                        .position(Vec2::new(
                            self.cell_size * x as f32 ,
                            self.cell_size * y as f32 ,
                        ))

                        .color(cell_temp.foreground)
                        .clip(Rectangle::new(sprite_x, sprite_y, 8.0, 8.0)),
                ); 
                
                
            }
        }
    }
}
