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

// impl<T> fmt::Display for Array<T> {
//      fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
//         write!(f, "{}", self)
//      }
// }
// struct I32{
//     items:[i32;9]
// }

// struct Chars{
//     items:[char;9]
// }

// struct Slice{

// }

// trait Object {
//     fn show_items(&self){
//         for i in self{
//             println!("{:?}", i);
//         }
        
//     }
// }

// impl Object for I32 {
//     fn show_items(&self){
//         for i in self.items{
//             println!("{:?}", i)
//         }
//     }
// }

// impl Object for Chars {
//     fn show_items(&self){
//         for i in self.items{
//             println!("{:?}", i)
//         }
//     }
    
// }


fn main(){

    let arr = Array::new([1,2,3,4,5,6,7,8,9,10]);
    println!("{:?}",arr);

    let charrs:Array<char> = Array::new(['A', 'B', 'C', 'D', 'E', 'F', 'I', 'J', 'K','L']);
    println!("{:?}", charrs);

}