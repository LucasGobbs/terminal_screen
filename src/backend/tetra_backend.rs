use crate::backend::Backend;
use crate::buffer::{Buffer};

use tetra::{Context};
use tetra::graphics::{self, Color, Texture, DrawParams, Rectangle};
use tetra::math::Vec2;
#[derive(Clone)]
pub struct TetraBackend{
    //ctx: &'static mut Context,
    font: Texture,
    cell_size: f32,
}
impl TetraBackend {
    pub fn new(/*ctx: &'static mut Context,*/ font: Texture) -> TetraBackend {
        let cell_size = (font.width() / 16) as f32;
        TetraBackend{
            //ctx,
            font,
            cell_size
        }
    }
}
impl Backend for TetraBackend {
    type Context = Context;
    fn draw(&mut self,buf: Buffer, ctx: &mut Context){
        for (i, cell) in buf.data.iter().enumerate(){
            let (x, y) = (i % buf.width, i / buf.width);
            let sprite_x = (219 / 16) as f32 * 8.0;
            let sprite_y = (219 % 16) as f32 * 8.0;
            
            if cell.background != Color::BLACK {
                graphics::draw(
                    ctx,
                    &self.font,
                    DrawParams::new()
                        .position(Vec2::new(
                            self.cell_size * x as f32,
                            self.cell_size * y as f32,
                        ))
                        .color(cell.background)
                        .clip(Rectangle::new(sprite_x, sprite_y, 8.0, 8.0)),
                );
            }
            
            if cell.glyph != ' ' {
  
                let codepoint = cell.glyph as u8;
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

                        .color(cell.foreground)
                        .clip(Rectangle::new(sprite_x, sprite_y, 8.0, 8.0)),
                ); 
            }
        }
    }
    fn clear(){

    }
}