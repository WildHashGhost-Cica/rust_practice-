struct Temperature{
    degrees_f: f64,
}
impl Temperature{
    fn freezing() -> Self {
        Self{ degrees_f: 32.0}
    }

    fn show_temp(&self){
    println!("{:?} degrees F", self.degrees_f);
    }

    fn boiling() -> Self {
        Self{ degrees_f: 212.0}
    }

    
}   
// Use an enum for the box color
enum Color {
    Brown,
    Red
}

impl Color {
    fn print (&self){
        match self{
        Color::Brown => println!("brown"),
        Color::Red => println!("red")
        }

    }
}
impl Dimensions{
    fn print(&self){
        println!("width:{:?}", self.width);
        println!("height:{:?}", self.height);
        println!("depth:{:?}", self.depth);
    }
}

struct Dimensions {
    width: f64,
    height: f64,
    depth: f64,
}

struct ShippingBox{
    color: Color,
    weight: f64,
    dimensions: Dimensions,
}

//Implement functionality on the box struct to create a box

impl ShippingBox{
    fn new(weight: f64, color: Color, dimensions: Dimensions) -> Self{
        Self{
            weight,
            color,
            dimensions
        }
    }
    fn print(&self){
        self.color.print();
        self.dimensions.print();
        println!("weight: {:?}", self.weight);
        
    }

}

fn main(){
    let hot =  Temperature{ degrees_f:99.9};
    hot.show_temp();

    let cold = Temperature::freezing();
    cold.show_temp();
    cold.show_temp();
    cold.show_temp();

    let boiling =  Temperature::boiling();
    boiling.show_temp();

    let small_dimensions =  Dimensions{
        width: 1.0,
        height: 2.0,
        depth: 3.0,
    };
    let small_box = ShippingBox::new( 5.0, Color::Red, small_dimensions);
    small_box.print();
}
