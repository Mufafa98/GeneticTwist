use std::io;

use cube::cube::Cube;

fn main() {
    let mut cube = Cube::new();
    loop {
        //cube.shuffle(10);
        let url = format!(
            "https://rubiks-cube-solver.com/solution.php?cube=0{}&x=1",
            cube.format_for_url()
        );
        //println!("{}", cube.get_online_solution(&url));
        println!("{}", cube);
        println!("Moves:");
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
// S X B B E B L X S M
// RE
// source: https://ruwix.com/online-rubiks-cube-solver-program/
// https://rubiks-cube-solver.com/solution.php?
//      idk             sus/jos                 sus/jos
// cube=0   666666666   333333333   222222222   555555555   444444444   111111111 &x=1
// cube=0   222222222   333333333   111111111   555555555   666666666   444444444 &x=1

//  orange  yellow  Blue    Red Green   White
//  2       6       5       4   3       1

//   1
// 2 3 4 5
//   6

// 123456789
//
// 123
// 456
// 789
//

//0 531545441 466436516 225265265 112652113 343313414 226423356
//0 531545441 225265265 112652113 343313414 466436516 226423356
