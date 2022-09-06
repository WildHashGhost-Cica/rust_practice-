// Use function that return tuple
fn coordinate() -> (i32, i32){
    (1,7)
}

fn main(){
    let coord = (2,3);
    println!("{:?}, {:?}", coord.0, coord.1);

    let (x, y) = (2,3);
    println!("{:?}, {:?}", x, y);

    let (name, age) = ("Cica", 40);
    println!("{:?}, {:?}", name, age);

    let favorite = ("black", "69", "UK", "pizza", "TV Show", "home");

    let state = favorite.2;
    let place = favorite.5;
    println!("{:?}", place);

    //Destructure the return value into two variables
    let (a, b) = coordinate();

    if b > 5 {
        println!(">5");
    } else if b < 5 {
        println!("<5");
    } else {
        println!("=5");
    }

   

}