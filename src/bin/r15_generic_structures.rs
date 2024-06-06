
trait Seat{
    fn show(&self){}
}

struct Ticket<T:Seat>{
    location:T
}

#[derive(Clone, Copy, Debug)]
enum ConcertSeat {
    FrontRow,
    MidSection,
    Back,
    
}

impl Seat for ConcertSeat {
    fn show(&self) {
        
        match *self {
            Self::FrontRow=>println!("{:?}", "FrontRow"),
            Self::MidSection=>println!("{:?}","MidSection"),
            Self::Back=>println!("{:?}", "Back"),

        }
    }
    
}

#[derive(Clone, Copy, Debug)]
enum AirlineSeat{
    BusinessClass,
    Economy,
    FirstClass,
}

impl Seat for AirlineSeat {
    fn show(&self) {
        match *self {
            Self::BusinessClass=>println!("{:?}", "Business Class!"),
            Self::Economy=>println!("{:?}", "Economy"),
            Self::FirstClass=>println!("{:?}", "FirstClass"),
        }
    }
}

fn ticket_info<T:Seat>(ticket:Ticket<T>){
    ticket.location.show();
}

fn main(){

    let airline = Ticket{location:AirlineSeat::BusinessClass};
    let concert = Ticket{location:ConcertSeat::FrontRow};
    ticket_info(airline);
    ticket_info(concert);
}