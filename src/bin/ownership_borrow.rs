enum Light{
    Bright, 
    Dull,
}
struct Book {
    pages: i32,
    rating: i32,
}

// Use a struct for the grocery item
struct GroceryItem{
    quantity: i32,
    id: i32,

}
fn display_page_count(book: &Book) {
    println!("pages = {:?}", book.pages);
}

fn display_rating(book: &Book ){
    println!("rating = {:?}", book.rating);
}

fn display_light(light: &Light) {
    match light {
        Light::Bright => println!("bringht"),
        Light::Dull => println!("dull"),
    }
}

// Create a function to display the quantity
fn display_quantity(item:&GroceryItem){
    println!("quantity: {:?}",item.quantity);
}

// Create a function to display the id
fn display_id(item:&GroceryItem){
    println!("id: {:?}", item.id);
}

fn main() {
    let dull = Light::Dull;
    display_light(&dull);
    display_light(&dull);

    let book  = Book{
        pages: 5,
        rating: 9,
    };
    display_page_count(&book);
    display_rating(&book);

    let my_item = GroceryItem {
        quantity: 3,
        id:99,
    };
    display_quantity(&my_item);
    display_id(&my_item);
}