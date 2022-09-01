fn main() {
    let my_name = "Leon";
    match my_name {
        "Cica" => println!("That is my name"),
        "Jayson" => println!("not my name"),
        "Alex" => println!("Hello Alex"),
        _ => println!("nice to meet you!")
    }
    let my_bool = false;
    match my_bool {
        true => println!("it's true"),
        false => println!("it's false"),
    }
    let my_number = 2;
    match my_number {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("other"),
    }
}