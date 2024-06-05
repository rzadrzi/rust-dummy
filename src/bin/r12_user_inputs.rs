use std::io;

fn get_input() -> io::Result<String>{
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_owned())
}

fn main(){
    match  get_input(){
        Ok(name)=> println!("hello {:?}", name),
        Err(e)=>println!("ERR: {:?}", e)
    }
}