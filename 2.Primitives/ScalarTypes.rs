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
}