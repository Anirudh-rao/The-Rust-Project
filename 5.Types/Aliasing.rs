/*
 The type statement can be used to give a new name to existing type.
Types must have UpperCamelCase names or the compiler will raise warnings
 */ 
type NanoSecond = u64;
type Inch = u64;
type U64 =  u64;


fn main(){
    //Nano Sechond = Inch = U64 = u64
    let nanoseconds:NanoSecond =  5 as u64;
    let inches:Inch =  2 as U64;


    println!("{} NanoSeconds + {} inches = {} Unit?",
        nanoseconds,inches, nanoseconds + inches);
}
