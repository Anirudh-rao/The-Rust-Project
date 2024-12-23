fn main(){
    //Vairable can be annotated
    let logical : bool = true;

    let a_float: f64 =  1.0;
    let an_integer: i32 =  5i32;


    //Or Default will be used
    let default_float = 3.0;
    let default_integer =  7;

    //A Type can also be inferred from the context
    let mut inferred_type =  12;
    inferred_type =  4294967296i64;

    // A mutable variable's value can be changed.
    let mut mutable = 12; // Mutable `i32`
    mutable = 21;

    // Error! The type of a variable can't be changed.
    //mutable = true;

    // Variables can be overwritten with shadowing.
    let mutable = true;

    //Comput Types  - Arrays and Tuples

    // Array signature consists of Type T and length as [T; length].
    let my_array:[i32;5] = [1,2,3,4,5];

     // Tuple is a collection of values of different types 
    // and is constructed using parentheses ().
    let my_tuple = (5u32, 1u8, true, -5.04f32);


}