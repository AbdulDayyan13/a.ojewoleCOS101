use std::io;

fn main(){
	let _l = 550_000;
	let _m = 120_000;
	let _k = 15_000;
	let _h = 25_000;

	println!("Laptop (L): 550_000");
	println!("Monitor (M): 120_000");
	println!("Keyboard (K): 15_000" );
	println!("Headset (H): 25_000");

	let mut input1 = String::new();
	println!("Enter product code:");
	io::stdin().read_line(&mut input1).expect("Failed to read input");

	let mut input2 = String::new();
	println!("Enter quantity: ");
	io::stdin().read_line(&mut input2).expect("Failed to read input");
	let q:u32 = input2.trim().parse().expect("Not a valid number");

}