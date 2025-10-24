use std::io;

fn main(){
	let mut input1 = String::new();
	let mut input2 = String::new();
	let mut input3 = String::new();

	println!("General Form of a Quadratic Equation is ax\u{00B2} + bx + c");
	println!("Enter the value for a:");
	io::stdin().read_line(&mut input1).expect("Not a valid string");
	let a:f32 = input1.trim().parse().expect("Not a valid number.");


	println!("Enter a value for b:");
	io::stdin().read_line(&mut input2).expect("Not a valid string");
	let b:f32 = input2.trim().parse().expect("Not a valid number");

	println!("Enter a value for c:");
	io::stdin().read_line(&mut input3).expect("Not a valid string");
	let c:f32 = input3.trim().parse().expect("Not a valid number");

	let d:f32= b.powf(2.0) - 4.0 * a * c;
	if d > 0.0 
	{
	let sqrt_d = d.sqrt();

	let x1:f32 = (-b + sqrt_d) / (2.0 * a);
	let x2:f32 = (-b - sqrt_d) / (2.0 * a);

	println!("Root 1 (+ve): {}", x1);
	println!("Root 2 (-ve): {}", x2); 
	}
else if d == 0.0
{
	let x:f32 = -b / (2.0 * a);

	println!("d = {} .", d);
	println!("The quadratic equation has only one root");
	println!("Root: {}", x);
}

else {
	println!("d = {} is negative.", d);
	println!("The quadratic equation has no real roots (only complex roots)");
} 
}