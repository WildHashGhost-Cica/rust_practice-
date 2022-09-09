fn print_it(data: &str) {
    println!("{:?}", data);

}

struct LineItem{
    name: String,
    count: i32,
}
fn print_name(name: &str){
    println!("name: {:?}", name);
}

fn main(){
    print_it("a string slice");

    let owned_string = "owned string".to_owned();
    let another_owned = String::from("another");
    print_it(&owned_string);
    print_it(&another_owned);


    let receipt = vec![
        LineItem{
            name: "cereal".to_owned(),
            count: 1,
        },
        LineItem{
            name: String::from("fruit"),
            count: 3,
        },
    ];

   /* for item in receipt{
        println!("name: {:?}, count:{:?}", item.name, item.count);
    }*/

    for item in receipt{
        print_name(&item.name);
        println!("count: {:?}", item.count);
    }
}