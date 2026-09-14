use std::fmt; 
struct Fahrenheit(f64); 

impl fmt::Display for Fahrenheit { 
    fn fmt(&self, f: &mut fmt::Formatter)-> fmt::Result { 
        let temp = self.0; 
        let status = if temp < 32.0 { "COLD" } 
                    else if temp > 85.0 { "HOT" } 
                    else { "MODERATE" }; 
        // Format with exactly 2 decimal places and the status tag 
        write!(f, "{:.2}°F [{}]", temp, status) 
    } 
} 

fn main() 
{ 
    let t1 = Fahrenheit(15.5); 
    let t2 = Fahrenheit(72.0); 
    let t3 = Fahrenheit(98.6); 
    println!("t1: {}", t1); 
    println!("t2: {}", t2); 
    println!("t3: {}", t3); 
}