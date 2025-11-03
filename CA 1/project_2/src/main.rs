use std::io;

fn main(){
	let mut input1 = String::new();
	let mut input2 = String::new();
	let mut input3 = String::new();

	println!("Enter the Principal:");
	io::stdin().read_line(&mut input1).expect("Not a valid number");
	let p:f32 = input1.trim().parse().expect(" Input a valid number");

	println!("Enter the Rate:");
	io::stdin().read_line(&mut input2).expect("Not a valid number");
	let r:f32 = input2.trim().parse().expect(" Input a valid number");

	println!("Enter the Time: ");
	io::stdin().read_line(&mut input3).expect("Not a valid number");
	let t:f32 = input3.trim().parse().expect("Input a valid number");

	let a:f32 = p * (1.0 + (r/100.0)).powf(t);
	let c:f32 = a - p;

	println!("Amount is {}",a);
	println!("Compound interest is {}", c);

	loop{
		let mut input1 = String::new();
		let mut input2 = String::new();
		let mut input3 = String::new();

	println!("Enter the Principal:");
	io::stdin().read_line(&mut input1).expect("Not a valid number");
	let p:f32 = input1.trim().parse().expect(" Input a valid number");

	println!("Enter the Rate:");
	io::stdin().read_line(&mut input2).expect("Not a valid number");
	let r:f32 = input2.trim().parse().expect(" Input a valid number");

	println!("Enter the Time: ");
	io::stdin().read_line(&mut input3).expect("Not a valid number");
	let t:f32 = input3.trim().parse().expect("Input a valid number");

	let a:f32 = p * (1.0 + (r/100.0)).powf(t);
	let c:f32 = a - p;

	println!("Amount is {}",a);
	println!("Compound interest is {}", c);
}
}