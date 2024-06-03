/*

in vecotors each element has same datatype 

*/


fn main(){


    let v1: Vec<i32> = vec![1, 2, 3];
    // let v2: Vec<_> = Vec::new()
    let mut v2: Vec<i32> = Vec::new();
    v2.push(4);
    v2.push(5);

    // for i in v1 {
    //     println!("{:?}", i);
    // }

    for j in v2 {
        for i in &v1 {
            println!("{:?}", i * j);
        }
    }
}