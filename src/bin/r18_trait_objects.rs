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

fn borrow_noisy(obj: &dyn Noise){
    obj.make_noise();
}
//======= They are different
fn borrow_noisy_use_box(obj: Box<dyn Noise>){
    obj.make_noise();
}

fn main(){
    let reza: Box<dyn Noise>= Box::new(Person);
    reza.make_noise();

    let dog_obj:&dyn Noise=&Dog;
    dog_obj.make_noise();

    let p = Person;
    borrow_noisy(&p);

    let d = Box::new(Dog);
    borrow_noisy_use_box(d);
}