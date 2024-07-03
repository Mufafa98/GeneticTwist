use utils::random_utils::generate_between;
#[derive(Debug)]
pub enum CubeMoves {
    Up,
    Down,
    Right,
    Left,
    Front,
    Back,
    X,
    Y,
    Z,
    M,
    S,
    E,
}
#[derive(Debug)]
pub enum MoveModifier {
    Prim,
    Double,
    Big,
    Normal,
    BigPrim,
    BigDouble,
}

impl MoveModifier {
    pub fn get_random() -> MoveModifier {
        let possible_moves = vec![
            MoveModifier::Normal,
            MoveModifier::Prim,
            MoveModifier::Double,
            MoveModifier::Big,
            MoveModifier::BigDouble,
            MoveModifier::BigPrim,
        ];
        let probability = generate_between(0.0, 1.0);
        let division: f32 = 100.0 / possible_moves.len() as f32 / 100.0;
        let mut counter = division.clone();
        for posibility in possible_moves {
            if probability < counter {
                return posibility;
            }
            counter += division;
        }
        return MoveModifier::Normal;
    }
    pub fn short_name(&self) -> String {
        let name = match self {
            MoveModifier::Prim => "\'",
            MoveModifier::Double => "2",
            MoveModifier::Big => "w",
            MoveModifier::Normal => "",
            MoveModifier::BigPrim => "w\'",
            MoveModifier::BigDouble => "w2",
        }
        .to_string();
        name
    }
    pub fn match_special(&self) -> bool {
        match self {
            MoveModifier::Big | MoveModifier::BigDouble | MoveModifier::BigPrim => false,
            _ => true,
        }
    }
}

impl CubeMoves {
    pub fn get_random() -> CubeMoves {
        let possible_moves = vec![
            CubeMoves::Up,
            CubeMoves::Down,
            CubeMoves::Right,
            CubeMoves::Left,
            CubeMoves::Front,
            CubeMoves::Back,
            CubeMoves::X,
            CubeMoves::Y,
            CubeMoves::Z,
            CubeMoves::M,
            CubeMoves::S,
            CubeMoves::E,
        ];
        let probability = generate_between(0.0, 1.0);
        let division: f32 = 100.0 / possible_moves.len() as f32 / 100.0;
        let mut counter = division.clone();
        for posibility in possible_moves {
            if probability < counter {
                return posibility;
            }
            counter += division;
        }
        return CubeMoves::E;
    }
    pub fn short_name(&self) -> String {
        let name = format!("{:?}", self)
            .chars()
            .nth(0)
            .unwrap_or(' ')
            .to_string();
        name
    }
    pub fn is_special(&self) -> bool {
        match self {
            CubeMoves::E
            | CubeMoves::M
            | CubeMoves::S
            | CubeMoves::X
            | CubeMoves::Z
            | CubeMoves::Y => true,
            _ => false,
        }
    }
}

pub fn decode_moves(move_to_decode: &str) -> Option<(CubeMoves, MoveModifier)> {
    // let move_type = move_to_decode
    //     .chars()
    //     .nth(0)
    //     .unwrap_or(' ')
    //     .to_uppercase()
    //     .nth(0)
    //     .unwrap_or(' ');
    let move_type = move_to_decode.chars().nth(0).unwrap_or(' ');
    let move_found: CubeMoves;
    let mut wide_move: bool = false;
    match move_type {
        'U' => move_found = CubeMoves::Up,
        'D' => move_found = CubeMoves::Down,
        'R' => move_found = CubeMoves::Right,
        'L' => move_found = CubeMoves::Left,
        'F' => move_found = CubeMoves::Front,
        'B' => move_found = CubeMoves::Back,
        'X' | 'x' => move_found = CubeMoves::X,
        'Y' | 'y' => move_found = CubeMoves::Y,
        'Z' | 'z' => move_found = CubeMoves::Z,
        'M' => move_found = CubeMoves::M,
        'S' => move_found = CubeMoves::S,
        'E' => move_found = CubeMoves::E,
        'u' => {
            move_found = CubeMoves::Up;
            wide_move = true;
        }
        'd' => {
            move_found = CubeMoves::Down;
            wide_move = true;
        }
        'r' => {
            move_found = CubeMoves::Right;
            wide_move = true;
        }
        'l' => {
            move_found = CubeMoves::Left;
            wide_move = true;
        }
        'f' => {
            move_found = CubeMoves::Front;
            wide_move = true;
        }
        'b' => {
            move_found = CubeMoves::Back;
            wide_move = true;
        }
        _ => return None,
    };
    let modifier_found: MoveModifier;
    if wide_move {
        if move_to_decode.chars().nth(1).unwrap_or(' ') == '\'' {
            modifier_found = MoveModifier::BigPrim
        } else if move_to_decode.chars().nth(1).unwrap_or(' ') == '2' {
            modifier_found = MoveModifier::BigDouble
        } else {
            modifier_found = MoveModifier::Big
        }
    } else {
        let move_modifier = move_to_decode.chars().nth(1).unwrap_or(' ');
        match move_modifier {
            '2' => modifier_found = MoveModifier::Double,
            'w' => {
                if move_to_decode.chars().nth(2).unwrap_or(' ') == '\'' {
                    modifier_found = MoveModifier::BigPrim
                } else {
                    modifier_found = MoveModifier::Big
                }
            }
            '\'' => modifier_found = MoveModifier::Prim,
            _ => modifier_found = MoveModifier::Normal,
        }
    }
    return Some((move_found, modifier_found));
}

// 	R U R' U' R' F R2 U' R' U' R U R' F'
// [R' U R U'] R2 y' [R' U' R U] y x [R U R' U'] [R2 x' U']
// M2 U M2 U2 M2 U M2
// [R' U R' d'] R' F' R2 U' R' U R' F R F
// x' [R U' R' D] [R U R' D'] [R U R' D] [R U' R' D']
