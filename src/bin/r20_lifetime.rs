
#[derive(Debug)]
enum Part{
    Bolt,
    Panel
}

#[derive(Debug)]
struct RobotArm<'a>{
    part: & 'a Part
}

#[derive(Debug)]
struct AssemblyLine{
    parts: Vec<Part>
}

fn show_robot_arm<'a>(item: &'a RobotArm){
    println!("{:?}", item.part);

}

fn main(){
    let assembly_line = AssemblyLine{
        parts : vec![Part::Bolt, Part::Panel]
    };

    let robot_arm = RobotArm{
        part: &assembly_line.parts[0]
    };

    // println!("{:?}", robot_arm.part);
    show_robot_arm(&robot_arm);



}