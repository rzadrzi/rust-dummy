trait Game {
    fn name(&self)->String{
        return String::from("Trait Game");
    }
}

enum BoardGame {
    Chess,
    Monopoly,
}

impl Game for BoardGame {
    fn name(&self)->String {
        match *self {
            Self::Chess=>String::from("Chess"),
            Self::Monopoly=>String::from("Monopoly"),
        }
    }
}

enum VideoGmae{
    PlayStation,
    Xbox,
}

impl Game for VideoGmae {
        fn name(&self)->String {
        match *self {
            Self::PlayStation=>String::from("PlayStation"),
            Self::Xbox=>String::from("Xbox"),
        }
    }
}

struct PlayRoom<T:Game> {
    game:T
}

impl PlayRoom<BoardGame> {
    fn cleanup(&self){
        println!("{:?}", "clean up please")
    }
    
}

impl <T:Game> PlayRoom<T> {
    fn restart(&self){
        println!("{:?} restarted!", self.game.name()); 
    }
    
}


fn main(){
    let game_01 = BoardGame::Chess;
    let game_02 = BoardGame::Monopoly;
    let game_03 = VideoGmae::PlayStation;
    let game_04 = VideoGmae::Xbox;

    let play_room_01 = PlayRoom{game:game_01};
    let play_room_02 = PlayRoom{game:game_02};
    let play_room_03 = PlayRoom{game:game_03};
    let play_room_04 = PlayRoom{game:game_04};

    play_room_01.cleanup();
    play_room_02.restart();
    play_room_03.restart();
    play_room_04.restart();

}