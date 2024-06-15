#[derive(Debug)]
struct Employee<State>{
    name: String,
    state: State
}

impl<State> Employee<State>{
    fn transition<NextState>(self, state:NextState)->Employee<NextState>{
        Employee{
            name:self.name, 
            state: state 
        }

    }
}
#[derive(Debug)]
struct Agreement;

#[derive(Debug)]
struct Signature;

#[derive(Debug)]
struct Training;

#[derive(Debug)]
struct FailedTraining{
    score: u8
}

struct OnBoardedCompelte{
    score: u8
}

impl Employee<Agreement>{
    fn new(name:&str)->Self{
        Self{
            name:name.to_string(),
            state:Agreement
        }
    }

    fn read_agreement(self)->Employee<Signature>{
        self.transition(Signature)
    }
}

impl Employee<Signature> {
    fn sig(self)->Employee<Training>{
        self.transition(Training)
    }
}

impl Employee<Training>{
    fn train(self, score:u8)->
    Result<Employee<OnBoardedCompelte>, Employee<FailedTraining>>
    {
        if score > 10 {
            Ok(self.transition(OnBoardedCompelte{score:score}))
        }else {
            Err(self.transition(FailedTraining{score:score}))
        }
    }
}

fn main(){

    let employee = Employee::new("Raza Darzi");
    let onboarded = employee.read_agreement().sig().train(34);
    match onboarded {
        Ok(complete)=>println!("completed: {:?}, score: {:?}", complete.name, complete.state.score),
        Err(e)=>println!("FAILED: {:?} state: {:?}", e.name, e.state.score)
    }


}