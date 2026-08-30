fn main(){
    // Variable can be type annotated
    let logical:bool = true;


    // Regular annotation
    let a_float: f64 = 1.0;
    // Suffix Annotation
    let an_integer = 5i32;


    //  or a default will be used
    let default_float = 3.0;
    let default_integer = 7;



    // A Type can also be inferred from context
    let mut inferred_type = 12;
    inferred_type = 4294967296i64;


    //  A Mutable Variables value can be changed
    let mut mutable = 12;
    mutable = 21;


    // ERROR! The Type of a variable cannot be changed
    // mutable = True;


    // Variable can be overwritten with shadowing
    let mutable = true;

    /*Compound types -Array and Tuple*/

    // Array signature consists of Type T and Length as[T length];
    let my_array:[i32 5] = [1,2,3,4,5];
    
    // Tuple is a collection of values of different types
    //  and is constructed using parenthesis ()
    let my_tuple = (5u32, 1u8, true, -5.04f32);

}