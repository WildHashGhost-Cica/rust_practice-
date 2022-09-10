//Use enum for the tickets 
//Backstage and Vip ticket include the ticket holder's name
enum Ticket {
    Backstage(f64, String),
    Standard(f64),
    Vip(f64, String),
}

fn main(){
    //Create one of each ticket and place into a vector
    let tickets = vec![
        Ticket::Backstage(50.0, "Billy".to_owned()),
        Ticket::Standard(15.0),
        Ticket::Vip(30.0, "Amy".to_owned()),
    ];

    //Use a match expression while iterating the vector to print the ticket info
    for ticket in tickets {
        match ticket{
            Ticket::Backstage(price, holder) => println!("Backstage ticket holder: {:?}, price: {:?}", holder, price),
            Ticket::Standard(price) => println!("Standard ticket holder price: {:?}", price),
            Ticket::Vip(price, holder) => println!("Vip ticket holder:{:?}, price:  {:?}", holder, price),
        }
    }
}