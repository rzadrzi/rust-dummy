// options.rs

#[derive(Debug)]
enum Kind {
	Jung,
	Maedchen,
}

#[derive(Debug)]
struct Kinder{
	geschlecht: Option<Kind>,
	alter: i32
}

fn check(kid:Kinder){
	match kid.geschlecht{
		Some(g)=>println!("{:?}", g),
		None => println!("{:?}", "WWHHHHAYYYYY")
	}
	println!("{:?}", kid.alter);
}


fn main(){
	let my_kid = Kinder{
		geschlecht: Some(Kind::Maedchen),
		alter: 32
	};
	check(my_kid);

	let my_kid2 = Kinder{
		geschlecht: Some(Kind::Jung),
		alter: 32
	};

	check(my_kid2)
}