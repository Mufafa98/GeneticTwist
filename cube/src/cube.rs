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

    pub fn make_move_string(&mut self, move_string: &str) {
        let move_data = decode_moves(move_string);
        if let Some((data, modifier)) = move_data {
            self.make_move(data, modifier);
        } else {
            println!("The move was not recognized. Try again")
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
                _ => {
                    println!("Not a valid move. This should be a bug. Please contact the developer")
                }
            },
            MoveModifier::Big => {}
            MoveModifier::Double => {}
            MoveModifier::Prim => {}
            MoveModifier::Special => match move_to_make {
                CubeMoves::X => self.x(),
                CubeMoves::Y => self.y(),
                CubeMoves::Z => self.z(),
                CubeMoves::M => self.m(),
                CubeMoves::E => self.e(),
                CubeMoves::S => self.s(),
                _ => {
                    println!("Not a valid move. This should be a bug. Please contact the developer")
                }
            },
        }
    }
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
        let back_buffer = self.cube[0].get_up().clone();
        let mut temp = self.cube[2].get_left();
        temp.reverse();
        self.cube[0].set_up(temp);
        temp = self.cube[5].get_down();
        self.cube[2].set_left(temp);
        temp = self.cube[4].get_right();
        temp.reverse();
        self.cube[5].set_down(temp);
        self.cube[4].set_right(back_buffer)
    }
    fn right(&mut self) {
        let right_buffer = self.cube[0].get_right().clone();
        self.cube[0].set_right(self.cube[3].get_right());
        self.cube[3].set_right(self.cube[5].get_right());
        self.cube[5].set_right(right_buffer);
        self.cube[4].rotate(1);
    }
    fn left(&mut self) {
        let left_buffer = self.cube[5].get_left().clone();
        self.cube[5].set_left(self.cube[3].get_left());
        self.cube[3].set_left(self.cube[0].get_left());
        self.cube[0].set_left(left_buffer);
        self.cube[2].rotate(1);
    }
    fn x(&mut self) {}
    fn y(&mut self) {
        let y_buffer = self.cube[1].get_face();
        self.cube[1].set_face(self.cube[2].get_face());
        self.cube[2].set_face(self.cube[3].get_face());
        self.cube[3].set_face(self.cube[4].get_face());
        self.cube[4].set_face(y_buffer);
        self.cube[0].rotate(1);
        self.cube[0].rotate(1);
        self.cube[5].rotate(-1);
        self.cube[5].rotate(-1);
    }
    fn z(&mut self) {}
    fn m(&mut self) {}
    fn s(&mut self) {}
    fn e(&mut self) {}
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
