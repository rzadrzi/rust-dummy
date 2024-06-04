
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


fn fibonacci(n:usize)->Vec<i32>{
    let mut list = vec![0,1];
    for i in 2..n{
        list.push( list[i-2] + list[i-1] );
    }
    return list;
}


fn main(){
    let a:i32 = 23;
    let b:i32 = 13;
    println!("{:?} + {:?} = {:?}", a, b, add(a, b));

    // cloture
    let add_closure = |a:i32, b:i32|->i32{
        a + b
    };

    println!("closure: {:?}",add_closure(20, 45));

    println!("factorial: {:?}", factorial(5));

    let n: usize = 10;

    let fib_list = fibonacci(n);

    for (i, v) in fib_list.iter().enumerate(){
        println!("{:?}  ->  {:?}", i, v);
    }

}