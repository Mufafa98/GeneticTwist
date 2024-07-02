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
}

pub fn decode_moves(move_to_decode: &str) -> Option<(CubeMoves, MoveModifier)> {
    let move_type = move_to_decode.chars().nth(0).unwrap_or(' ');
    let move_found: CubeMoves;
    match move_type {
        'U' => move_found = CubeMoves::Up,
        'D' => move_found = CubeMoves::Down,
        'R' => move_found = CubeMoves::Right,
        'L' => move_found = CubeMoves::Left,
        'F' => move_found = CubeMoves::Front,
        'B' => move_found = CubeMoves::Back,
        'X' => move_found = CubeMoves::X,
        'Y' => move_found = CubeMoves::Y,
        'Z' => move_found = CubeMoves::Z,
        'M' => move_found = CubeMoves::M,
        'S' => move_found = CubeMoves::S,
        'E' => move_found = CubeMoves::E,
        _ => return None,
    };
    let move_modifier = move_to_decode.chars().nth(1).unwrap_or(' ');
    let modifier_found: MoveModifier;
    match move_modifier {
        '2' => modifier_found = MoveModifier::Double,
        'w' => modifier_found = MoveModifier::Big,
        '\'' => modifier_found = MoveModifier::Prim,
        _ => modifier_found = MoveModifier::Normal,
    }
    return Some((move_found, modifier_found));
}
