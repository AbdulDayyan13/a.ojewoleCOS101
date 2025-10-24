use std::io;

fn main(){
	let mut input1 = String::new();
	let mut input2 = String::new();

	println!("Are you a veteran ? Yes/No");
	io::stdin().read_line(&mut input1).expect("Not a valid string");
	let veteran = input1.trim().to_lowercase();

	if veteran == "no" 
	{
		println!("Your incentive is \u{20A6}100,000");
		return;
	}

	println!("Enter your age:");
	io::stdin().read_line(&mut input2).expect("Not a valid string");
	let age:u32 = input2.trim().parse().expect("Not a valid number");

 	if veteran == "yes" && age >=40
	{
		println!("Congratulations! Your incentive is \u{20A6}1,560,000 ");
	}
	else if veteran == "yes" && age >=30 
	{
		println!("Congratulations! Your incentive is \u{20A6}1,480,000 ");
	}
	else if veteran == "yes" && age < 28
	{
		println!("Congratulations! Your incentive is \u{20A6}1,300,000");
	}
	else if age >=28  
	{
		println!("Your age is not included in the incentive/age bracket.");
		print!("Please see management for your incentive ");
	}
	}