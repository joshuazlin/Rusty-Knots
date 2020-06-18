extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use std::io;

mod visibility_graph;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    rotation: f64,  // Rotation for the square.
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 50.0);
        let rotation = self.rotation;
        let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);

            let transform = c
                .transform
                .trans(x, y)
                .rot_rad(rotation)
                .trans(-25.0, -25.0);

            // Draw a box rotating around the middle of the screen.
            rectangle(RED, square, transform, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        // self.rotation += 4.0 * args.dt;
    }
}


pub struct CombKnot<'a> {
    //  A struct for a combinatorial representation of a knot drawing.
    //  The knot is represented as a collection of half_edges, labelled from 
    //  0 ... 4*v-1 where v is the number of vertices of the drawing. 
    //  It is assumed that half-edge 2k and 2k+1 are part of the same edge. 
    //  sigma provides the anticlockwise ordering of half-edges about a vertex, 
    //  so it is a 2D list of size v * 4. It is always assumed that the 
    //  first entry in this list belongs to the under-strand of the crossing. 
    sigma : &'a [&'a [u32]],
}

impl CombKnot<'_> {
    fn draw(&mut self) {
        //A really bad drawing algorithm. 
        //The idea is this: We will just be iterating through the vertices.
        //Every time, we will just be looking at an unconnected edge we have so far, 
        //find the vertex it corresponds to, and try drawing that somewhere. 
        //We also record all the edges that we have drawn, and we try
        //not to run into ourselves essentially. 


    }
    
    fn check(&mut self) -> bool{
        //Checks that its a valid combinatorial representation of a knot
        //For now, only basic checks; doesn't actually guarentee correctness, just 
        //weeds out bad cases. 
        
        let mut seen_vertex = vec![0; self.sigma.len()*4];

        for v in self.sigma{
            if v.len() != 4{ return false }
            for w in *v{ seen_vertex[*w as usize] += 1; }
        }

        for x in seen_vertex{
            if x != 1{return false}
        }

        true
    }
}






fn main() {






    // println!("Input some representation of a knot");

    // let mut guess = String::new();

    // io::stdin()
    //     .read_line(&mut guess)
    //     .expect("Failed to read line");

    // println!("You guessed: {}", guess);





    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("spinning-square", [200, 200])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        rotation: 0.0,
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}