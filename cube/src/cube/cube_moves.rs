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
pub enum MoveModifier {
    Prim,
    Double,
    Big,
    Normal,
    Special,
}

pub fn decode_moves(move_to_decode: &str) -> Option<(CubeMoves, MoveModifier)> {
    match move_to_decode {
        "U" | "Up" => return Some((CubeMoves::Up, MoveModifier::Normal)),
        "D" | "Down" => return Some((CubeMoves::Down, MoveModifier::Normal)),
        "R" | "Right" => return Some((CubeMoves::Right, MoveModifier::Normal)),
        "L" | "Left" => return Some((CubeMoves::Left, MoveModifier::Normal)),
        "F" | "Front" => return Some((CubeMoves::Front, MoveModifier::Normal)),
        "B" | "Back" => return Some((CubeMoves::Back, MoveModifier::Normal)),
        "X" => return Some((CubeMoves::X, MoveModifier::Special)),
        "Y" => return Some((CubeMoves::Y, MoveModifier::Special)),
        "Z" => return Some((CubeMoves::Z, MoveModifier::Special)),
        "M" => return Some((CubeMoves::M, MoveModifier::Special)),
        "S" => return Some((CubeMoves::S, MoveModifier::Special)),
        "E" => return Some((CubeMoves::E, MoveModifier::Special)),
        _ => return None,
    };
}
