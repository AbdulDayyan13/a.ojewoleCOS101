fn main() {
	let t:f64 = 450000.00;
	let m:f64 = 1500000.00;
	let h:f64 = 750000.00;
	let d:f64 = 2850000.00;
	let a:f64 = 250000.00;

	//sum and average
	let s = t + m + h + d + a;
	println!("Sum is {}", s);
	let v = s / 5.0 ;
	println!("Average is {}", v);

}