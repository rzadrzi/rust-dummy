

struct I32{
    items:[i32;9]
}

struct Chars{
    items:[char;9]
}

trait Object {
    fn show_items(&self){}
}

impl Object for I32 {
    fn show_items(&self){
        for i in self.items{
            println!("{:?}", i)
        }
    }
}

impl Object for Chars {
    fn show_items(&self){
        for i in self.items{
            println!("{:?}", i)
        }
    }
    
}


fn main(){

    let arr = I32{
        items:[1,2,3,4,5,6,7,8,10]
    };

    arr.show_items();
    
    
    let charss = Chars{
        items:['A', 'B', 'C', 'D', 'E', 'F', 'I', 'J', 'K']
    };
    charss.show_items();
}