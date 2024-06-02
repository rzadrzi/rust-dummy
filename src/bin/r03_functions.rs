
fn add(a:i32, b:i32)->i32{
    return a + b;
}

fn factorial(n:u128)->u128{
    match n {
        0 => 1,
        1 => 1,
        _ => n * factorial(n-1)
        
    }
}

fn main(){
    let a:i32 = 23;
    let b:i32 = 13;
    println!("{:?} + {:?} = {:?}", a, b, add(a, b));

    println!("factorial: {:?}", factorial(5))

}