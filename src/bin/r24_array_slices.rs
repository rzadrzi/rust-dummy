fn show_array(arr:&[i32]){
    for i in arr{
        println!("{:?}", i)
    }
}

fn main(){
    let arr:[i32; 9] = [1,2,3,4,5,6,7,8,10];
    show_array(&arr);
}