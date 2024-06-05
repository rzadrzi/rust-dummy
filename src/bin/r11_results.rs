struct SoundData {
    name: String,
}

impl SoundData {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
        }
    }
}

fn get_sound(name: &str) -> Result<SoundData, String> {
    if name == "alert" { Ok(SoundData::new(name)) } else { Err("not good".to_owned()) }
}

fn main() {
    let new_sound = "alert34";
    let r: Result<SoundData, String> = get_sound(new_sound);
    match r {
        Ok(s) => println!("OK: {:?}", s.name),
        Err(e) => println!("Err: {:?}", e),
    }
}
