#[derive(Debug)]
enum Direction{
    Up,
    Down,
    Left, 
    Right
}

// Be careful: &
fn show_direction(direction:&Direction){
    match direction {
        Direction::Up => println!("Up!"),
        Direction::Down => println!("Down!"),
        Direction::Left => println!("Left!"),
        Direction::Right => println!("Right!"),
    }
}

fn main(){
    let new_direction:Vec<Direction>=vec![
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right
        ];

    for d in new_direction.iter(){
        show_direction(d);

    }                             
}