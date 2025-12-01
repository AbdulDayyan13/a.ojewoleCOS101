fn main(){

	let v = vec![10,20,30];
	//vector v owns the object in heap

	let v2 = v;  	//moves ownershop to v2
	display (v2.clone ());
	//v2 is moved tp display and v2 is invalidated

	println!("In main {:?}", v2);
	//v2 is no longer usable here
}

fn display(v:Vec<i32>){
	println!("inside display {:?}", v);
}