/*
Scope and Shadowing
Variable bindings have a scope and are constrained to live in a block.
A Block is a colletion of statments enclosed by {}
*/

fn main(){
	//This binding lives in the main function
	let long_lived_binding =1;
	
	//This is a block. and has a smaller scope than the main function
	{
	//This Binding only exists in this block
	let short_lived_binding  = 2;
	println!("inner short:{}",short_lived_binding);
	}

	//End of Block
	//ERROR !: Shor_lived_Binding
	//println!("Outer Short:{}",short_lived_binding);

	println!("Outer Long:{}",long_lived_binding);

}
