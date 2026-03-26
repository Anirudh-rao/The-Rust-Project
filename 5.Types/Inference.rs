/*
The Type inference engine is pretty smart. 
If looks at how variable is used afterwards to infer its type
*/

fn main(){
    //Because of the annotation the complier knows elem has the type u8
    let elem =  5u8;

    //Create a empty Vector
    let mut vec = Vec::new();
    //At this point compiler does not know what is vec

    //Insert elem in the vector
    vec.push(elem);

    //Now compiler knows that the element is a vector of u8
    println!("{:?}",vec);


}
