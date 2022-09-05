fn main() {
    let mut i =1;
    while i <= 3 {
        println!("{:?}", i);
        i = i + 1;
    }

    //counts down from 5 to 1, display "done" when complete
    let mut counter = 5;
    while counter >= 1 {
        println!("{:?}", counter);
        counter = counter - 1;
    }
    println!("Done");
}