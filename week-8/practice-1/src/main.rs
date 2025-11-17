fn main(){
	//Using Vec::new()
	let v : Vec<i64> = Vec::new();

	//printing the size of the vector
	println!("\n The length of the Vec::new is: {}",v.len());

	//Using macro
	let v = vec!["Grace", "Effiong", "Basil", "Kareem", "Susan"];

	//printing the size of the vector
	println!("\n The length of the vec macro is: { }", v.len());
}
