use std::fmt::{self, Debug};


#[derive(Debug, Clone)]
struct Array<T>
{
    items:[T; 10]
}


impl <T> Array<T> {
    pub fn new(itemss:[T;10]) -> Self{
        Self { items: itemss }
    }

    // pub fn show_items(&self){
    //     for i in &self.items{
    //         println!("{:?}", i);
    //     }
    // }
    
}


fn main(){

    let arr = Array::new([1,2,3,4,5,6,7,8,9,10]);
    println!("{:?}",arr);

    let charrs:Array<char> = Array::new(['A', 'B', 'C', 'D', 'E', 'F', 'I', 'J', 'K','L']);
    println!("{:?}", charrs);

}