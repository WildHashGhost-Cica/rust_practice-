struct Person;
impl Noise for Person {
    fn make_noise(&self){
        println!("hello");
    }
}

struct Dog;
impl Noise for Dog {
    fn make_noise(&self){
        println!("woof");
    }
}

trait Noise{
    fn make_noise (&self);
}

fn hello(noisy: impl Noise) {
    noisy.make_noise();
}

fn main(){
    hello(Person {});
    hello(Dog {});
}