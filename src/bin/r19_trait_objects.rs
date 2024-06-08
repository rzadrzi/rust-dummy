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
        
    }
}

fn road_trip(vehicle:&dyn LandCapable){
    vehicle.drive();
}

fn main(){

    let sedan = Sedan;
    let suv = SUV;

    // sedan.drive();
    // suv.drive();
    road_trip(&sedan);
    road_trip(&suv);
}