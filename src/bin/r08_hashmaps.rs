use std::collections::HashMap;

fn main(){

    let mut my_hashmap=HashMap::new();
    my_hashmap.insert("Reza", 1);
    my_hashmap.insert("Hamed", 23);

    for (i,v) in my_hashmap.iter(){
        println!("({:?}, {:?})", i, v)
    }

}