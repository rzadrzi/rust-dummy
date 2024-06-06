trait Noise{
    fn make_noise(&self){}
}

struct Person;
impl Noise for Person {
    fn make_noise(&self) {
        println!("A Person")
    }
}

struct Dog;
impl Noise for Dog {
    fn make_noise(&self) {
        println!("A Dog");
    }
}

fn make_noise<T:Noise>(thing:T){
    thing.make_noise();
    
}

fn main(){

    let p = Person;
    let d = Dog;

    make_noise(p);
    make_noise(d);

}