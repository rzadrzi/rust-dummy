fn show_array(arr:&[i32]){
    for i in arr{
        println!("{:?}", i)
    }
}

fn show_slice(slice:&[char]){
    for i in slice{
        println!("{:?}", i)
    }
}

fn main(){
    let arr:[i32; 9] = [1,2,3,4,5,6,7,8,10];
    show_array(&arr);

    let charss: [char; 9] = ['A', 'B', 'C', 'D', 'E', 'F', 'I', 'J', 'K'];
    let s:&[char]=&charss;
    show_slice(s);
}