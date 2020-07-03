
mod console;
mod shape;
mod buffer;
mod line;
mod circle;
mod rect;
mod triangle;
use tetra::graphics::{self, Color, Texture};
use tetra::input::{self, Key};
use tetra::time::get_fps;
use tetra::{Context, ContextBuilder, State};
use rand::Rng;

use std::time::{Duration, Instant};


use crate::console::{Console};
use crate::shape::Shape;
use crate::line::Line;
use crate::circle::Circle;
use crate::rect::Rect;
use crate::buffer::Buffer;
use crate::triangle::Triangle;

#[derive(Clone, Copy)]
struct Drop {
    x: i32,
    y: i32,
    r: u8,
    g: u8,
    b: u8,
}
impl Drop {
    pub fn new(x: i32, y: i32, r: u8, g: u8, b: u8) -> Drop{
        Drop {
            x,
            y,
            r,
            g,
            b,
        }
    }
    pub fn rand() -> Drop {
        let mut rng = rand::thread_rng();
        let x: i32 = rand::random::<i32>()% 150;
        let y: i32 = rand::random::<i32>() % 100 - 100;
        let r: u8 = rand::random::<u8>();
        let g: u8 = rand::random::<u8>();
        let b: u8 = rand::random::<u8>();
        return Drop::new(x,y,r,g,b);
    }
}
struct GameState {
    //world: World,
    //resources: Resources,
    drops: Vec<Drop>,
    console: Console,
    size: i32,
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
       // let mut world = World::new();
       // let resources = Resources::new(&mut world);

        let font = Texture::new(ctx, "./resources/terminal.png")?;
        let console = Console::new(font, 150, 100);
        let mut drops = Vec::new();
    
        for i in 0..500 {
            drops.push(Drop::rand());
        }
        Ok(GameState {
           // world,
           // resources,
            drops,
            console,
            size: 25,
        })
    }
}

impl State for GameState {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        for drop in &mut self.drops {
            drop.y += 1;
            if drop.y > 100 {
                *drop = Drop::rand();
            }
        }
        let x = input::get_mouse_wheel_movement(ctx).y;
        self.size += x;

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::BLACK);

        self.console.clear();
        
        self.console.temp_buffer.set_char(15,15,'2',Color::GREEN);
        self.console.temp_buffer.set_char(16,16,'3',Color::GREEN);
     
        self.console.temp_buffer.set_string(15,16,format!("fps: {}",get_fps(ctx)).as_str(),Color::WHITE);
        self.console.temp_buffer.set_string(15,17,format!("Raio: {}",self.size).as_str(),Color::WHITE);
        
        let mut ta = Buffer::new(200,200);
        let mut tb = Buffer::new(200,200);
        let mut tc = Buffer::new(200,200);
        ta.g_draw(Circle::new(50,50,self.size,true),'#',Color::RED);
        tb.g_draw(Circle::new(50,50,self.size,true),'#',Color::RED);
        tc.g_draw(Circle::new(50,50,self.size,true),'#',Color::RED);
        self.console.temp_buffer.g_draw(Circle::new(50,50,self.size,true),'#',Color::RED);
        if input::is_key_down(ctx,Key::C){
            //print!("CLICANDO");
            self.console.temp_buffer.flood_fill_rec(50,50, '.', Color::GREEN);
        } else {
            self.console.temp_buffer.flood_fill(50,50, '.', Color::GREEN);
        }
        
        print!("Raio: {}\n",self.size);

        /*
        let now = Instant::now();
        ta.flood_fill(50,50, '.', Color::GREEN);
        print!("Iterativo: {}\n", now.elapsed().as_micros());

    
        //let now2 = Instant::now();
        //tb.flood_fill_rec(50,50, '.', Color::GREEN);
       // print!("Recursivo: {}\n\n\n", now2.elapsed().as_micros());

        let now3 = Instant::now();
        tc.flood_fill_rec2(50,50, '.', Color::GREEN);
        print!("Recursivo 2: {}\n\n\n", now3.elapsed().as_micros());
        // */
        for drop in &self.drops {
            self.console.temp_buffer.set_char(drop.x, drop.y, '|', Color::rgb8(drop.r,drop.g,drop.b));
        }
        //self.console.temp_buffer.sub(t_buffer);
        self.console.draw(ctx);
        


        Ok(())
    }
}

fn main() -> tetra::Result {
    let lin = Line::new(0,0,4,1);
    for cell in  lin.get_cells(){
        print!("{} {}\n",cell.0,cell.1);
    }
    ContextBuilder::new("Rogue Terminal", 150 * 8, 100 * 8)
       // .timestep(Timestep::Fixed(30.0))
        .quit_on_escape(true)
        .build()?
        .run(GameState::new)
}
