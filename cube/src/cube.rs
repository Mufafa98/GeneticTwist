use std::fmt::Display;

mod cube_face;
use cube_face::{CubeColors, Face};
use cube_moves::{decode_moves, CubeMoves, MoveModifier};

mod cube_moves;

#[derive(Debug)]
pub struct Cube {
    cube: [Face; 6],
}

impl Cube {
    pub fn new() -> Cube {
        Cube {
            cube: [
                Face::from(CubeColors::Yellow),
                Face::from(CubeColors::Orange),
                Face::from(CubeColors::Blue),
                Face::from(CubeColors::Red),
                Face::from(CubeColors::Green),
                Face::from(CubeColors::White),
            ],
        }
    }

    fn generate_shuffle(size: usize) -> Vec<(CubeMoves, MoveModifier)> {
        let mut len = size.clone();
        let mut moves: Vec<(CubeMoves, MoveModifier)> = Vec::new();
        while len > 0 {
            let generated_move = CubeMoves::get_random();
            moves.push((generated_move, MoveModifier::Normal));
            len -= 1;
        }
        for (current_move, modifier) in moves.iter_mut() {
            let mut new_modifier = MoveModifier::get_random();
            while current_move.is_special() && !new_modifier.match_special() {
                new_modifier = MoveModifier::get_random();
            }
            *modifier = new_modifier;
        }
        moves
    }
    pub fn shuffle(&mut self, size: usize) {
        let shuffle = Cube::generate_shuffle(size);
        for (move_to_make, modifier) in shuffle {
            self.make_move(move_to_make, modifier);
        }
    }
    pub fn parse_string_move(&mut self, moves: Vec<String>) {
        for move_string in moves {
            self.make_move_string(move_string.as_str());
        }
    }
    fn reset(&mut self) {
        self.cube = [
            Face::from(CubeColors::Yellow),
            Face::from(CubeColors::Orange),
            Face::from(CubeColors::Blue),
            Face::from(CubeColors::Red),
            Face::from(CubeColors::Green),
            Face::from(CubeColors::White),
        ];
    }
    pub fn make_move_string(&mut self, move_string: &str) {
        let move_data = decode_moves(move_string);
        if let Some((data, modifier)) = move_data {
            self.make_move(data, modifier);
        } else {
            let tokens: Vec<String> = move_string
                .split("+")
                .map(|token| token.to_string())
                .collect();
            if let Some(command) = tokens.get(0) {
                if command == "!shuffle" {
                    if let Some(no_moves_string) = tokens.get(1) {
                        if let Ok(number) = no_moves_string.parse() {
                            self.shuffle(number)
                        } else {
                            println!("Invalid number format")
                        }
                    } else {
                        println!("Invalid number of moves\n Shuffle format is: !shuffle+<number>");
                    }
                } else if command == "!reset" {
                    self.reset();
                } else {
                    println!("The move {} was not recognized. Try again", move_string);
                }
            } else {
                println!("No command recived");
            }
        }
    }
    pub fn make_move(&mut self, move_to_make: CubeMoves, modifier: MoveModifier) {
        match modifier {
            MoveModifier::Normal => match move_to_make {
                CubeMoves::Up => self.up(),
                CubeMoves::Down => self.down(),
                CubeMoves::Right => self.right(),
                CubeMoves::Left => self.left(),
                CubeMoves::Front => self.front(),
                CubeMoves::Back => self.back(),
                CubeMoves::X => self.x(),
                CubeMoves::Y => self.y(),
                CubeMoves::Z => self.z(),
                CubeMoves::M => self.m(),
                CubeMoves::E => self.e(),
                CubeMoves::S => self.s(),
            },
            MoveModifier::Big => match move_to_make {
                CubeMoves::Up => {
                    self.e();
                    self.e();
                    self.e();
                    self.up();
                }
                CubeMoves::Down => {
                    self.e();
                    self.down();
                }
                CubeMoves::Right => {
                    self.m();
                    self.m();
                    self.m();
                    self.right();
                }
                CubeMoves::Left => {
                    self.m();
                    self.left();
                }
                CubeMoves::Front => {
                    self.s();
                    self.front();
                }
                CubeMoves::Back => {
                    self.s();
                    self.back();
                }
                _ => println!(
                    "Unsuported move combination {} {}",
                    move_to_make.short_name(),
                    modifier.short_name()
                ),
            },
            MoveModifier::BigPrim => match move_to_make {
                CubeMoves::Up => {
                    self.e();
                    self.e();
                    self.e();
                    self.up();

                    self.e();
                    self.e();
                    self.e();
                    self.up();

                    self.e();
                    self.e();
                    self.e();
                    self.up();
                }
                CubeMoves::Down => {
                    self.e();
                    self.down();

                    self.e();
                    self.down();

                    self.e();
                    self.down();
                }
                CubeMoves::Right => {
                    self.m();
                    self.m();
                    self.m();
                    self.right();

                    self.m();
                    self.m();
                    self.m();
                    self.right();

                    self.m();
                    self.m();
                    self.m();
                    self.right();
                }
                CubeMoves::Left => {
                    self.m();
                    self.left();

                    self.m();
                    self.left();

                    self.m();
                    self.left();
                }
                CubeMoves::Front => {
                    self.s();
                    self.front();

                    self.s();
                    self.front();

                    self.s();
                    self.front();
                }
                CubeMoves::Back => {
                    self.s();
                    self.back();

                    self.s();
                    self.back();

                    self.s();
                    self.back();
                }
                _ => println!(
                    "Unsuported move combination {} {}",
                    move_to_make.short_name(),
                    modifier.short_name()
                ),
            },
            MoveModifier::BigDouble => match move_to_make {
                CubeMoves::Up => {
                    self.e();
                    self.e();
                    self.e();
                    self.up();

                    self.e();
                    self.e();
                    self.e();
                    self.up();
                }
                CubeMoves::Down => {
                    self.e();
                    self.down();
                    self.e();
                    self.down();
                }
                CubeMoves::Right => {
                    self.m();
                    self.m();
                    self.m();
                    self.right();
                    self.m();
                    self.m();
                    self.m();
                    self.right();
                }
                CubeMoves::Left => {
                    self.m();
                    self.left();
                    self.m();
                    self.left();
                }
                CubeMoves::Front => {
                    self.s();
                    self.front();
                    self.s();
                    self.front();
                }
                CubeMoves::Back => {
                    self.s();
                    self.back();
                    self.s();
                    self.back();
                }
                _ => println!(
                    "Unsuported move combination {} {}",
                    move_to_make.short_name(),
                    modifier.short_name()
                ),
            },
            MoveModifier::Double => match move_to_make {
                CubeMoves::Up => {
                    self.up();
                    self.up()
                }
                CubeMoves::Down => {
                    self.down();
                    self.down()
                }
                CubeMoves::Right => {
                    self.right();
                    self.right();
                }
                CubeMoves::Left => {
                    self.left();
                    self.left();
                }
                CubeMoves::Front => {
                    self.front();
                    self.front();
                }
                CubeMoves::Back => {
                    self.back();
                    self.back();
                }
                CubeMoves::X => {
                    self.x();
                    self.x();
                }
                CubeMoves::Y => {
                    self.y();
                    self.y();
                }
                CubeMoves::Z => {
                    self.z();
                    self.z();
                }
                CubeMoves::M => {
                    self.m();
                    self.m();
                }
                CubeMoves::E => {
                    self.e();
                    self.e();
                }
                CubeMoves::S => {
                    self.s();
                    self.s();
                }
            },
            MoveModifier::Prim => match move_to_make {
                CubeMoves::Up => {
                    self.up();
                    self.up();
                    self.up();
                }
                CubeMoves::Down => {
                    self.down();
                    self.down();
                    self.down();
                }
                CubeMoves::Right => {
                    self.right();
                    self.right();
                    self.right();
                }
                CubeMoves::Left => {
                    self.left();
                    self.left();
                    self.left();
                }
                CubeMoves::Front => {
                    self.front();
                    self.front();
                    self.front();
                }
                CubeMoves::Back => {
                    self.back();
                    self.back();
                    self.back();
                }
                CubeMoves::X => {
                    self.x();
                    self.x();
                    self.x();
                }
                CubeMoves::Y => {
                    self.y();
                    self.y();
                    self.y();
                }
                CubeMoves::Z => {
                    self.z();
                    self.z();
                    self.z();
                }
                CubeMoves::M => {
                    self.m();
                    self.m();
                    self.m();
                }
                CubeMoves::E => {
                    self.e();
                    self.e();
                    self.e();
                }
                CubeMoves::S => {
                    self.s();
                    self.s();
                    self.s();
                }
            },
        }
    }
    fn generate_line(
        &self,
        first: &String,
        second: &String,
        third: &String,
        fourth: &String,
    ) -> String {
        let mut result = String::new();
        result += first;
        result += " ";
        result += second;
        result += " ";
        result += third;
        result += " ";
        result += fourth;
        result += "\n";
        return result;
    }
    fn generate_face_line(
        &self,
        first: &[String; 3],
        second: &[String; 3],
        third: &[String; 3],
        fourth: &[String; 3],
    ) -> String {
        let mut result = String::new();
        result += &self.generate_line(&first[0], &second[0], &third[0], &fourth[0]);
        result += &self.generate_line(&first[1], &second[1], &third[1], &fourth[1]);
        result += &self.generate_line(&first[2], &second[2], &third[2], &fourth[2]);
        return result;
    }
}

//  Moves implementation
impl Cube {
    fn up(&mut self) {
        let up_buffer = self.cube[1].get_up().clone();
        self.cube[1].set_up(self.cube[2].get_up());
        self.cube[2].set_up(self.cube[3].get_up());
        self.cube[3].set_up(self.cube[4].get_up());
        self.cube[4].set_up(up_buffer);
        self.cube[0].rotate(1);
    }
    fn down(&mut self) {
        let down_buffer = self.cube[4].get_down().clone();
        self.cube[4].set_down(self.cube[3].get_down());
        self.cube[3].set_down(self.cube[2].get_down());
        self.cube[2].set_down(self.cube[1].get_down());
        self.cube[1].set_down(down_buffer);
        self.cube[5].rotate(1);
    }
    fn front(&mut self) {
        self.cube[3].rotate(1);
        let front_buffer = self.cube[0].get_down().clone();
        let mut temp = self.cube[2].get_right();
        temp.reverse();
        self.cube[0].set_down(temp);
        temp = self.cube[5].get_up();
        self.cube[2].set_right(temp);
        temp = self.cube[4].get_left();
        temp.reverse();
        self.cube[5].set_up(temp);
        self.cube[4].set_left(front_buffer)
    }
    fn back(&mut self) {
        self.cube[1].rotate(1);
        let mut back_buffer = self.cube[0].get_up().clone();
        let mut temp = self.cube[4].get_right();
        self.cube[0].set_up(temp);
        temp = self.cube[5].get_down();
        temp.reverse();
        self.cube[4].set_right(temp);
        temp = self.cube[2].get_left();
        self.cube[5].set_down(temp);
        back_buffer.reverse();
        self.cube[2].set_left(back_buffer)
    }
    fn right(&mut self) {
        let mut right_buffer = self.cube[0].get_right().clone();
        self.cube[0].set_right(self.cube[3].get_right());
        self.cube[3].set_right(self.cube[5].get_right());
        let mut temp = self.cube[1].get_left();
        temp.swap(0, 2);
        self.cube[5].set_right(temp);
        right_buffer.swap(0, 2);
        self.cube[1].set_left(right_buffer);
        self.cube[4].rotate(1);
    }
    fn left(&mut self) {
        let mut left_buffer = self.cube[5].get_left().clone();
        self.cube[5].set_left(self.cube[3].get_left());
        self.cube[3].set_left(self.cube[0].get_left());
        let mut temp = self.cube[1].get_right();
        temp.swap(0, 2);
        self.cube[0].set_left(temp);
        left_buffer.swap(0, 2);
        self.cube[1].set_right(left_buffer);
        self.cube[2].rotate(1);
    }
    fn x(&mut self) {
        let mut x_buffer = self.cube[0].get_face();
        self.cube[0].set_face(self.cube[3].get_face());
        self.cube[3].set_face(self.cube[5].get_face());
        let mut temp = self.cube[1].get_face();
        temp.swap(0, 2);
        temp = self.swap_cols(&mut temp, 0, 2);
        self.cube[5].set_face(temp);
        x_buffer.swap(0, 2);
        x_buffer = self.swap_cols(&mut x_buffer, 0, 2);
        self.cube[1].set_face(x_buffer);
        self.cube[2].rotate(-1);
        self.cube[4].rotate(1);
    }
    fn y(&mut self) {
        let y_buffer = self.cube[1].get_face();
        self.cube[1].set_face(self.cube[2].get_face());
        self.cube[2].set_face(self.cube[3].get_face());
        self.cube[3].set_face(self.cube[4].get_face());
        self.cube[4].set_face(y_buffer);
        self.cube[0].rotate(1);
        self.cube[5].rotate(-1);
    }
    fn z(&mut self) {
        self.cube[1].rotate(-1);
        self.cube[3].rotate(1);

        self.cube[0].rotate(1);

        self.cube[4].rotate(1);

        self.cube[5].rotate(1);

        self.cube[2].rotate(1);

        self.cube.swap(0, 4);
        self.cube.swap(2, 5);
        self.cube.swap(0, 5);
    }
    fn m(&mut self) {
        let mid_buffer = self.cube[0].get_mid_v();
        let mut temp = self.cube[1].get_mid_v();
        temp.swap(0, 2);
        self.cube[0].set_mid_v(temp);
        let mut temp = self.cube[5].get_mid_v();
        temp.swap(0, 2);
        self.cube[1].set_mid_v(temp);
        self.cube[5].set_mid_v(self.cube[3].get_mid_v());
        self.cube[3].set_mid_v(mid_buffer);
    }
    fn s(&mut self) {
        self.front();
        self.front();
        self.front();
        self.back();
        self.z();
    }
    fn e(&mut self) {
        let e_buffer = self.cube[1].get_mid_h();
        self.cube[1].set_mid_h(self.cube[4].get_mid_h());
        self.cube[4].set_mid_h(self.cube[3].get_mid_h());
        self.cube[3].set_mid_h(self.cube[2].get_mid_h());
        self.cube[2].set_mid_h(e_buffer);
    }
    fn swap_cols(
        &self,
        matrix: &mut [[CubeColors; 3]; 3],
        a: usize,
        b: usize,
    ) -> [[CubeColors; 3]; 3] {
        let temp_col = [matrix[0][a], matrix[1][a], matrix[2][a]];
        matrix[0][a] = matrix[0][b];
        matrix[1][a] = matrix[1][b];
        matrix[2][a] = matrix[2][b];
        matrix[0][b] = temp_col[0];
        matrix[1][b] = temp_col[1];
        matrix[2][b] = temp_col[2];
        *matrix
    }
}

impl Display for Cube {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut to_display = String::new();
        let empty_face = Face::from(CubeColors::None).face_data();
        let yellow_face = self.cube[0].face_data();
        let orange_face = self.cube[1].face_data();
        let blue_face = self.cube[2].face_data();
        let red_face = self.cube[3].face_data();
        let green_face = self.cube[4].face_data();
        let white_face = self.cube[5].face_data();
        to_display += &self.generate_face_line(&empty_face, &empty_face, &yellow_face, &empty_face);
        to_display += &self.generate_face_line(&orange_face, &blue_face, &red_face, &green_face);
        to_display += &self.generate_face_line(&empty_face, &empty_face, &white_face, &empty_face);

        write!(f, "{to_display}")
    }
}
