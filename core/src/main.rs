use std::io;

use cube::cube::Cube;

fn _temp() {}
fn main() {
    let mut cube = Cube::new();
    loop {
        //cube.shuffle(10);
        println!("{}", cube);
        println!("Command:");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        input = input.replace("\n", "");
        input = input.replace("[", "");
        input = input.replace("]", "");
        let moves: Vec<String> = input.split(" ").map(|comand| comand.to_string()).collect();
        cube.parse_string_move(moves);
        // cube.make_move_string(input.trim());
    }
}
