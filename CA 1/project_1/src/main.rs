use std::io;

fn main (){
	let mut name = String::new();
	let mut score_1 = String::new();
	let mut score_2 = String::new();
	let mut score_3= String::new();

	let ga = "A";
	let gb = "B";
	let gc = "C";
	let gd = "D";
	let gf = "F";

	println!("Enter your name:");
	io::stdin().read_line(&mut name).expect("Not a valid input");

	println!("Enter First_score: ");
	io::stdin().read_line(&mut score_1).expect("Not a valid string");
	let first_score:f32 = score_1.trim().parse().expect("Input a valid test score");

	println!("Enter Second_score: ");
	io::stdin().read_line(&mut score_2).expect("Not a valid string");
	let second_score:f32 = score_2.trim().parse().expect("Input a valid test score");

	println!("Enter Third_score: ");
	io::stdin().read_line(&mut score_3).expect("Not a valid string");
	let third_score:f32 = score_3.trim().parse().expect("Input a valid test score");

	let a:f32 = (first_score + second_score + third_score) / 3.0 ;
	println!("Average is {}", a);

	if a >=70.0 && a == 100.0 {
		println!("Your grade is {}", ga);
		println!("{}", name);
}
	else if a >=60.0 && a <=69.0 {
		println!("Your grade is {}", gb);
		println!("{}", name);
}
	else if a >=50.0 && a <=59.0 {
 		println!("Your grade is {}", gc);
		println!("{}", name);
}
	else if a >=45.0 && a <=49.0 {
		println!("Your grade is {}", gd);
		println!("{}", name);
} 
	else if a>=0.0 && a <=44.0 {
		println!("Your grade is {}", gf);
		println!("{}", name);
}
	else if a<0.0 && a > 100.0 {
		println!("This is not a valid score");
	}
	else {
		println!("You do not have a grade");
		println!("Meet your program coordinator");
	}

}