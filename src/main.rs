//mod container;
extern crate derive_builder;
mod shape;
mod buffer;
mod console;
#[macro_use]
mod component;
use tetra::graphics::{self, Color, Texture};
use tetra::input::{self, Key};
use tetra::time::*;
use tetra::{Context, ContextBuilder, State};




use regex::*;
use std::fmt::{self, Display};
use crate::component::{Component, ComponentDrawable,TextComponent, ComponentBuilder};

use crate::shape::*;

use crate::buffer::Buffer;
use crate::console::Console;
struct Player {
    x: i32,
    y: i32,
}
impl Player {
    pub fn new(x: i32, y: i32) -> Player {
        Player {
            x,
            y,
        }
    }
}
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
    player: Player,
    console: Console,
    el_time: f32,
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
            player: Player::new(50,50),
            drops,
            console,
            el_time: 0.0,
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
        self.el_time += get_delta_time(ctx).as_secs_f32();
       
        if input::is_key_down(ctx, Key::D){
            self.player.x += 1;
        } else if input::is_key_down(ctx, Key::A){
            self.player.x -= 1;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::BLACK);

        self.console.clear();
        
        self.console.temp_buffer.set_char(0,0,'2',Color::GREEN);
        self.console.temp_buffer.set_string(0,1,format!("fps: {}",get_fps(ctx)).as_str(),Color::WHITE);
        
        
        for drop in &self.drops {
            //self.console.temp_buffer.set_char(drop.x, drop.y, '|', Color::rgb8(drop.r,drop.g,drop.b));
        }

        let mousex = (input::get_mouse_position(ctx).x / 8.0) as i32;
        let mousey = (input::get_mouse_position(ctx).y / 8.0) as i32;
        if input::is_mouse_button_down(ctx, input::MouseButton::Left){
            self.console.temp_buffer.g_draw(Line::new(self.player.x,
                                                             self.player.y,
                                                             mousex,
                                                             mousey), 
                                            '.', Color::WHITE);
        }
        self.console.temp_buffer.set_char(self.player.x,self.player.y,'█',Color::GREEN);
        self.console.temp_buffer.set_char(mousex,mousey,'*',Color::GREEN);
        //self.console.temp_buffer.g_draw(Rect::new(mousex -2,mousey-4,4,8,true), 'm', Color::rgb(1.0,1.0,0.0));
        //self.console.temp_buffer.sub(t_buffer);
        //let mut tcmp = TextComponent::new().pos(2, 5)
        //                                                      .text(String::from("Alo"));
     
        let mut life_txt = TextComponent::new(ComponentBuilder::default()
                                                                                        .pos((5,0))
                                                                                        .size((5,0))
                                                                                        .changed(false));
        life_txt.add_text("Life:", Color::WHITE)
                .generate();                              
       
        self.console.temp_buffer.c_draw(    life_txt.clone());
        //let mut dimp2 = DividerComponent::new(10,15,9).centered();
      
        //self.console.temp_buffer.g_draw(Circle::new(10,10,30),'2',Color::BLUE);
        self.console.temp_buffer.set_string(1,50,"Life:",Color::WHITE);
        self.console.temp_buffer.set_string(7,50,"9",Color::RED);
        self.console.temp_buffer.set_string(9,50,"/",Color::WHITE);
        self.console.temp_buffer.set_string(11,50,"10",Color::GREEN);
        self.console.draw(ctx);
        


        Ok(())
    }
}
macro_rules! textcmp {
    ($fmt_string:expr, $( $arg:expr ),*) => {
        let regex = Regex::new(r"(?m)\{[a-zA-Z]*\}").unwrap();
        let result = regex.find_iter($fmt_string);
        let mut args: Vec<String> = Vec::new();
        $(
            args.push($arg.into());
        )*
        for a in args {
            println!("Arg: {}",a);
        }
        for mat in result {
            println!("Color{}",mat.as_str());
        }
    }
}
enum Colorr<D: Display> {
    Red(D),
    Green(D)
}
impl<D: Display> fmt::Display for Colorr<D> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Ok(match self {
            Colorr::Red(txt) => print!("Vermelho: {}", txt.to_string()),
            Colorr::Green(txt) => print!("Verde: {}", txt.to_string()),
        })
    }
}
fn main() -> tetra::Result {
    /*
    let regex = Regex::new(r"(?m)\{[a-zA-Z]*\}").unwrap();
    let string = "{White} {red} {a}";
  
  // result will be an iterator over tuples containing the start and end indices for each match in the string
    let result = regex.find_iter(string);
    
    for mat in result {
        println!("{}", mat.as_str());
        if String::from(mat.as_str()).to_lowercase().contains("white"){
             print!("AEEE")
        }
        
    }
    */
    //textcmp!("{white} {red} {white} {green}","life: ",9.to_string(),"/",10.to_string());

    
    let text = format!("Life: {}/{}", Colorr::Red(9), Colorr::Green("10"));
    ContextBuilder::new("Terminal", 150 * 8, 100 * 8)
       // .timestep(Timestep::Fixed(30.0))
        .quit_on_escape(true)
        .build()?
        .run(GameState::new)
}
