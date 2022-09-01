fn main(){
    loop{
        println!("hello");
        break;
    }
    let mut i = 5;
    loop{
        println!("{:?}",i);
        i = i - 1;
        if i == 0 {
            break;
        }
    }
    println!("done!");
    // * Display "1" through "4" in the terminal 
    let mut n = 1 ;
    loop{
        println!("{:?}",n);
        //n = n + 1; it will break before #4 
        if n == 4 {
            break;
        }
        n = n + 1;
    }
}