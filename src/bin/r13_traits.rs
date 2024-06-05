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


fn main(){
    let p:Person=Person;
    let d:Dog=Dog;

    p.make_noise();
    d.make_noise();
}