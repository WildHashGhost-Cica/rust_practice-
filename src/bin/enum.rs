enum Direction{
    Up,
    Down,
    Left,
    Right
}
fn main() {
    let go = Direction::Left;
    match go {
        Direction::Up => println!("up"),
        Direction::Down => println!("down"),
        Direction::Left => println!("left"),
        Direction::Right => println!("right"),
    }
}