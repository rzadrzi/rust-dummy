struct Sedan;
impl LandCapable for Sedan {
    fn drive(&self) {
        println!("Sedan");
    }
}

struct SUV;
impl LandCapable for SUV{
    fn drive(&self){
        println!("SUV");
    }
}

trait LandCapable {
    fn drive(&self) {
        println!("drive on the road")
    }
}

trait WaterCapable {
    fn float(&self){
        println!("dirve on the water")
    }
}

// trait Amphibious:LandCapable+WaterCapable {}
struct Amphibious<T:LandCapable+WaterCapable>{
    vehicle:T
}

// impl <T:LandCapable+WaterCapable> Amphibious<T> {
    
// }
// struct Howecraft;

// impl Amphibious for Howecraft {
//     fn drive(&self){
//         println!("Howercraft")
//     }
    
// }

fn road_trip(vehicle:&dyn LandCapable){
    vehicle.drive();
}

fn howercraft<T:LandCapable+WaterCapable>(vehicle:&Amphibious<T>){
    vehicle.drive();
    vehicle.float();
}

fn main(){

    let sedan = Sedan;
    let suv = SUV;

    // sedan.drive();
    // suv.drive();
    road_trip(&sedan);
    road_trip(&suv);
}