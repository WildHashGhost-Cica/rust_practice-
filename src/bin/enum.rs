enum Direction{
    Up,
    Down,
    Left,
    Right
}
enum Color{
    Red,
    Black,
    Green
}

fn print_color(my_color: Color){
    match my_color{
        Color::Red => println!("red"),
        Color::Black => println!("black"),
        Color::Green => println!("green"),
    }
}

fn main() {
    let go = Direction::Left;
    match go {
        Direction::Up => println!("up"),
        Direction::Down => println!("down"),
        Direction::Left => println!("left"),
        Direction::Right => println!("right"),
    }
    print_color(Color::Black);
}