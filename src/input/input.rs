pub enum Input {
    Quit,
    Back,
    Enter,
    Move(Move),
}

pub enum Move {
    Up,
    Down,
    Left,
    Right,
}
