enum Access {
    Admin,
    Manager,
    User,
    Guest
}
// Use a function to print the messages
fn print_message(gt_100: bool){
    //Use a match experssion to determine which message to pring
    match gt_100 {
        true => println!("it's big"),
        false => println!("it's small"),
    }
}

fn main(){
    // secret file: admins only
    let access_level = Access::Guest;
    let can_access_file = match access_level{
        Access::Admin => true,
        _ => false,
    };
    println!("can access: {:?}", can_access_file);

    let value = 100;
    let is_gt_100 = value > 100 ;
    //it's not neccessary to use
   /* if value > 100{
        true
    } else {
        false
    }*/
    print_message(is_gt_100);
}