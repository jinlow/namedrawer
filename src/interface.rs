use crate::namedrawing;
use std::io::{self, BufRead, Write};

fn print_directions() {
    println!("Name Drawing App!");
    println!("-----------------\n");
    println!("Enter the names of people to place in");
    println!("the drawing each on a new line.");
    println!("If there are two people you don't want");
    println!("to buy for eachother, enter them at the ");
    println!("same time seperated by a plus sign (+).");
    println!("Type draw_names() to draw names.");
    // println!("Type restart to clear the drawing and start over.");
    println!("Type quit() to end.");
}

pub fn run_interactive() {
    print_directions();
    let mut namedraw = namedrawing::NameDrawing::new();
    loop {
        print!("Enter Name: ");
        let _ = io::stdout().flush();
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("Input not read.");
        if name.trim() == "quit()" {
            break;
        }
        if name.trim() == "draw_names()" {
            namedraw.draw_names();
            continue;
        }
        let name_vec: Vec<String> = name.split("+").map(|x| x.trim().to_string()).collect();
        namedraw.add_names(name_vec);
    }
}
