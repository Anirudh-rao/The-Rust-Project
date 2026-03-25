/*
 Casting
 Rust provides no implict type conversion between primitive types but,
 explicit type conversion can be perfomed using `as` keyword.
 Rules of type conversion follows C conventions generally , except in cases where C has undefined behaviour.
*/

//Supress all errors from casts which overflow
#![allow(overflowing_literals)]

fn main(){
    let decimal =  65.4321_f32;

    //Error! No Implict Conversion
    //let integer:u8 =  decimal;
    

    //Explicit Conversion
    let integer = decimal as u8;
    let charecter =  integer as char;

    //Error! There are limitation in conversion rules
    //A Float cannot be directly converted to  a char
    //let charecter = decimal as char;
    //
    println!("Casting : {} -> {} -> {}", decimal, integer, charecter);

    //When casting any value to an unsigned type,T
    //T:: max + 1 is aded or subtracted until the alue
    //Fits into the type only when #![allow(overflowing_literals)]
    //lint is specified like above else there will be compiler error
    
    // 1000 already fits in u16;
    println!("10000 as u16 is  {} ",1000 as u16);


    //1000-256-256-256 = 323
    //Under the hood the first 8 least significatn bits are kept
    //while the rest toward the most significant bit get truncated
    println!("1000 as u8 is :{} ", 1000 as u8);

    //-1 + 256 = 255
    println!("-1 as u8 is :{} ",(-1i8)as u8);

    //For Positive Numbers this is the same as modulus
    println!("1000 mod 256 :{}",1000 % 256);


    //When casting to signied type , the bitwise result is the same are
    //first casting to the corresponding unsigned type. If the most significant
    //bit of the value is 1 , the value is negative.
    //
    //Unless is already fits, of course.
    println!("128 as i16 is :{} ", 128 as i16);


    //In boundary case 128 value in 8-bit twos complement representation
    println!("128 as i8 is :{}", 128 as i8);


    // Repeating the example above
    // 1000 as u8 -> 232
    println!("1000 as a u8 is :{} ", 1000 as u8);

    // and the value of 232 in 8-bit twos complement representation is -24
    println!("232 as a i8 is :{}", 232 as i8);

    //If the floating point value exceeds the uppder bound or is less than lower bound, 
    //the returned value will be equal to the bound crossed.
    //300.0 as u8 is 255

    println!("3000 as u8 is {}",300.0_f32 as u8);
    println!("-100 as u8 is {}", -100.0_f32 as u8);
    println!(" nan as u8 is {}",f32::NAN as u8);


    unsafe{
        println!("300.0 as u8 is :{}",300.0_f32.to_int_unchecked::<u8>());
        println!("-100.0 as u8 is :{}",(-100.0_f32).to_int_unchecked::<u8>());
        println!("nan as a u8 is {}",f32::NAN.to_int_unchecked::<u8>());
    }


}
