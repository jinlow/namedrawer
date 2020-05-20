mod namedrawing;

// Simple name drawing program
// Names can be placed in the NameDrawing struct as Strings.
// If you don't want peaple to buy for eachother, place
// them in the same vector together.

fn main() {
    let mut drawing = namedrawing::NameDrawing::new(vec![
        vec![String::from("Hannah")],
        vec![String::from("James"), String::from("Katie I")],
        vec![String::from("Hollie"), String::from("Isaac")],
        vec![String::from("Luke"), String::from("Katie E")],
        vec![String::from("Josh"), String::from("Marquelle")],
    ]);
    drawing.draw_names();
}
