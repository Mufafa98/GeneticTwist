use std::io;

use cube::cube::Cube;

fn main() {
    let mut cube = Cube::new();
    loop {
        println!("{}", cube);
        println!("Move:");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        input = input.replace("\n", "");
        cube.make_move_string(input.trim());
    }
}
