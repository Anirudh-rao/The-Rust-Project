fn reverse(pair :(i32, bool)) -> (bool,i32){
    // Let can be used to bind the members of the tuple variables
    let(int_param, bool_param) = pair;

    (bool_param,int_param)
}
// The following struct is for the activity
#[derive(Debug)]
struct Matrix(f32,f32,f32,f32);

fn main(){
    // A Tuple with a buncj of differnt types
    let long_tuple = (1u8, 2u16, 3u32, 4u64,
                      -1i8, -2i16, -3i32, -4i64,
                      0.1f32, 0.2f64,
                      'a', true);

    // values can be extracted from the tuples using indexing
    println!("Long tuple first value:{}", long_tuple.0);
    println!("Lond tuple second value{}", long_tuple.1);


    // Tuple can be tuple members
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    // Tuples are printable.
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    let pair=  (1, true);
    println!("Pair is {:?}", pair);

    println!("The reversed pair is {:?}", reverse(pair));

    // To create one element tuples, the comma is required to tell them apart
    // from a literal surrounded by parentheses.
    println!("One element tuple: {:?}", (5u32,));
    println!("Just an integer: {:?}", (5u32));

    // Tuples can be destructured to create bindings.
    let tuple = (1, "hello", 4.5, true);

    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);
}