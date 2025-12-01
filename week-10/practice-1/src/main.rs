fn main(){
	
	let v = vec![101, 250, 330, 400];
	//vector v owns the object in heap

	//onl asingle variable owns the heap memory at a given time
	let v2 = v.clone();
	//here two variables own the heap value
	//two pointers to the same content is not allowed in Rust

	//Rust is very smart in terms of memory access, so it detects a race condition
	//as two variables point to same heap

	println!("{:?}",v);
}