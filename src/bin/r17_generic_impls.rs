trait Body {
    
}

trait Color {
    
}

#[derive(Clone, Copy, Debug)]
struct Vehicle<B, C>
where B:Body, C:Color
{
    body: B,
    color: C
}

impl<B, C> Vehicle<B, C> 
where B:Body, C:Color
{
    pub fn new(body:B, color:C)->Self{
        Self { body: body, color: color }
    }
}

#[derive(Clone, Copy, Debug)]
struct Car;
impl Body for Car {
    
}

#[derive(Clone, Copy, Debug)]
struct Truck;
impl Body for Truck {
    
}

#[derive(Clone, Copy, Debug)]
struct Red;
impl Color for Red {
    
}

#[derive(Clone, Copy, Debug)]
struct Blue;
impl Color for Blue {
    
}
fn main(){

    let vehicle_01 = Vehicle::new(Car, Red);
    let vehicle_02=Vehicle::new(Truck, Blue);

    println!("{:?}", vehicle_01);
    println!("{:?}", vehicle_02);

}