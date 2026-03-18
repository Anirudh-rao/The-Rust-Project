fn main(){
	let shadowed_binding =1;
	
	{
	println!("before being shadowed :{}", shadowed_binding);
	let shadowed_binding  = "abc";
	println!("shadowed in inner blocl:{}",shadowed_binding);
	}
	println!("Outside inner block :{}", shadowed_binding);
	
	let shadowed_binding = 2;
	println!("Shadowed in outer block :{}", shadowed_binding);
	
}
