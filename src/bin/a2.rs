// * Use a function to add two numbers together
fn sum(a: i32, b: i32) -> i32 {
    a + b
}

// * Use a function to display the result
fn display_result(result: i32){
    println!("{:?}", result);
}





fn main() {
    let result= sum(2,2);
    display_result(result);
}