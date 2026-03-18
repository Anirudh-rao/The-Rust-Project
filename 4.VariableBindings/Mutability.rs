/*
Variable bindings are immutable by default
But this can be overrridern using the "mut" modifier
*/

fn main(){
let _immutable_binding = 1;
let mut mutable_binding = 1;

println!("Before mutation :{} ",mutable_binding);

//ok
mutable_binding += 1;

println!("After mutation:{}", mutable_binding);

//Error cannot assign a new value to an immutable variable
//_immutable_binding += 1;

}
