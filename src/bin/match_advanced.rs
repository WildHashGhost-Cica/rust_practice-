enum Discount {
    Percent(i32),
    Flat(i32),
}
 
struct Ticket {
    event: String,
    price: i32,
}

fn main(){
    let n = 3;
    match n {
        3 => println!("three"), 
        /*
        _ => println!("number: {:?}", n),*/
        //replace _ in match
        other => println!("number: {:?}", other),
    }
    let flat = Discount::Flat(2);
    match flat {
        Discount::Flat(2) => println!("flat 2"),
        Discount::Flat(amount) => println!("flat discount of {:?}", amount),
        //ignore other like Percent
        _ => (),
    }

    let concert = Ticket{
        event: "concert".to_owned(),
        price: 50,
    };

    match concert{
        //ignor other field with ..   
        Ticket {price: 50, event} => println!("event @ 50 ={:?}", event),
        Ticket {price, ..} => println!("price = {:?}", price),
    }


}