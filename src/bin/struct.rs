struct ShippingBox {
    depth: i32,
    width: i32,
    height: i32,
}

struct GroceryItem {
    stock: i32,
    price: f64,
}

fn main(){
    let my_box = ShippingBox {
        depth: 3, 
        width:2, 
        height:5,
    };
    let tall = my_box.height;
    println!("the box is {:?} units tall", tall);

    let cereal = GroceryItem{
        stock: 10,
        price: 2.99,
    };
    println!("stock:{:?}", cereal.stock);
    println!("price: {:?}", cereal.price);
}