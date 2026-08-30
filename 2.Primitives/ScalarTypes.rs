fn main(){
    // ==========================================
    // 1. INTEGERS (Signed: i8, i16, i32, i64, i128, isize)
    // ==========================================
    // Signed integers can hold both positive and negative numbers.
    let signed_8:i8 = -128;
    let signed_8: i8 = -128;
    let signed_16: i16 = 32_767;
    let signed_32: i32 = -2_147_483_648; // Default integer type in Rust
    let signed_64: i64 = 9_223_372_036_854_775_807;
    let signed_128: i128 = -170_141_183_460_469_231_731_687_303_715_884_105_728;
    let signed_arch: isize = -42; // Depends on CPU architecture (32-bit or 64-bit)

    println!("--- Signed Integers ---");
    println!("i8:    {}", signed_8);
    println!("i16:   {}", signed_16);
    println!("i32:   {}", signed_32);
    println!("i64:   {}", signed_64);
    println!("i128:  {}", signed_128);
    println!("isize: {}\n", signed_arch);

    // ==========================================
    // 2. INTEGERS (Unsigned: u8, u16, u32, u64, u128, usize)
    // ==========================================
    // Unsigned integers can only hold non-negative numbers.
    let unsigned_8: u8 = 255;
    let unsigned_16: u16 = 65_535;
    let unsigned_32: u32 = 4_294_967_295;
    let unsigned_64: u64 = 18_446_744_073_709_551_615;
    let unsigned_128: u128 = 340_282_366_920_938_463_463_374_607_431_768_211_455;
    let unsigned_arch: usize = 1000; // Commonly used to index arrays or collections

    println!("--- Unsigned Integers ---");
    println!("u8:    {}", unsigned_8);
    println!("u16:   {}", unsigned_16);
    println!("u32:   {}", unsigned_32);
    println!("u64:   {}", unsigned_64);
    println!("u128:  {}", unsigned_128);
    println!("usize: {}\n", unsigned_arch);

    // ==========================================
    // 3. FLOATING-POINT NUMBERS (f32, f64)
    // ==========================================
    // Floats represent numbers with decimal points.
    let float_32: f32 = 3.14159; // Single precision
    let float_64: f64 = 2.718281828459045; // Double precision (Default float type)

    println!("--- Floating-Point ---");
    println!("f32: {}", float_32);
    println!("f64: {}\n", float_64);

    // ==========================================
    // 4. BOOLEAN (bool)
    // ==========================================
    // Booleans represent true or false values. They take up 1 byte.
    let is_rust_fun: bool = true;
    let is_failing: bool = false;

    println!("--- Booleans ---");
    println!("True value:  {}", is_rust_fun);
    println!("False value: {}\n", is_failing);

    // ==========================================
    // 5. CHARACTER (char)
    // ==========================================
    // Specified with single quotes. It represents a 4-byte Unicode scalar value.
    let letter: char = 'R';
    let emoji: char = '🦀'; // Can store emojis and accented characters

    println!("--- Characters ---");
    println!("Letter: {}", letter);
    println!("Emoji:  {}", emoji);


}