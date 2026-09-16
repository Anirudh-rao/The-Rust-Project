fn main(){
    let _immutable_binding = 1;
    let mut mutable_bindings = 1;

    println!("Before mutation:{}", mutable_bindings);


    //After mutated
    mutable_bindings += 1;
    println!("After mutation:{}", mutable_bindings);

     // Error! Cannot assign a new value to an immutable variable
    //_immutable_binding += 1;
}