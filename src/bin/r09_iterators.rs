

fn main(){
    let vec01 = vec![1,2,3,4,5,6,7,8,9];
    let vec02:Vec<i32> = vec01
    .iter()
    .map(|a| a+1)
    .collect();

    for (_, v) in vec02.iter().enumerate(){
        println!("{:?}", v);
    }

}