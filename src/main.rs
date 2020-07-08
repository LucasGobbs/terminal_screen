#[crate_type = "bin"]

mod backend;
use backend::{Backend,TetraBackend};
mod shape;
mod buffer;
mod console;
mod buffer_app;
use crate::buffer_app::BufferApp;
mod component;
use tetra::graphics::{self, Color, Texture};
use tetra::input::{self, Key};

use tetra::{Context, ContextBuilder, State};




use regex::*;
use std::fmt::{self, Display};
use crate::component::*;

use crate::shape::*;

use crate::buffer::Buffer;
use crate::console::Console;

struct GameState {
 
    buffer_backend: TetraBackend,
    app: BufferApp,
}


impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
       // let mut world = World::new();
       // let resources = Resources::new(&mut world);
        let font = Texture::new(ctx, "./resources/terminal.png")?;
        let buffer_backend = TetraBackend::new(font);
       
        Ok(GameState {

            buffer_backend,
            app: BufferApp::new(150,100),
        })
    }
}

impl State for GameState {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::BLACK);

      
        self.app.buf().clear();
      
   
        let mousex = (input::get_mouse_position(ctx).x / 8.0) as i32;
        let mousey = (input::get_mouse_position(ctx).y / 8.0) as i32;

        self.app.buf().set_char(mousex,mousey,'█',Color::GREEN);
        
        let mut life_txt = TextComponent::new(Component::new(0,0));
        life_txt.add_text("Fala;",Color::WHITE)
                .add_text(" ai mano",Color::RED)
                .generate();
        let mut div = DividerComponent::new(Component::new(1,10).size(11,1));
        div.line_char('#')
           .vertical()
           .corner_char('@')
           .center_char('0')
           .generate();
        
        self.app.buf().c_draw(life_txt);
        self.app.buf().c_draw(div);
        //self.console.temp_buffer.c_draw(life_txt);
        
     
        //self.console.draw(ctx);

        self.buffer_backend.draw(self.app.clone().draw(2), ctx);

        Ok(())
    }
}
/*
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
*/
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

    
    //let text = format!("Life: {}/{}", Colorr::Red(9), Colorr::Green("10"));
    ContextBuilder::new("Terminal", 150 * 8, 100 * 8)
       // .timestep(Timestep::Fixed(30.0))
        .quit_on_escape(true)
        .build()?
        .run(GameState::new)
}
