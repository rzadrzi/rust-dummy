struct NeverZero(i32);

impl NeverZero {
    fn new(i:i32)->Result<Self, String>{
        if i == 0{
            Err(String::from("Zero"))
        }else{
            Ok(Self(i))
        }
    }
}

fn divided(a: i32, b: NeverZero) -> i32{
    let b = b.0;
    a/b

}

fn main(){

    let a = 10;

    match NeverZero::new(0) {
        Ok(nz)=>println!("{:?}", divided(a, nz)),
        Err(e)=>println!("{:?}", e)        
    }

}