fn main(){
    let string01 = String::from("abcd");
    let string02 = String::from("abcdefg");

    let long_string = longest(&string01, &string02);
    println!("{:?}", long_string)

}

fn longest<'a>(x:&'a str, y:&'a str)->&'a str{
    if x.len() > y.len(){
        x
    }else{
        y
    }
}