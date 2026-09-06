fn main(){
    println!("{:<5} | {:^12} | {:>6} |","ID","Name", "Grade");
    println!("{} |","-".repeat(29));

     // Print students using a mix of positional and named arguments below
    // Student 1: ID = 1, Name = "Alice", Grade = 92.54
    println!("{:<5} | {:^12} | {:>6} |",id=1,name="Alice", grade=92.54);
    // Student 2: ID = 12, Name = "Bob", Grade = 85.0
    println!("{:<5} | {:^12} | {:>6} |",id=2,name="Bob", grade=85.0);
    // Student 3: ID = 103, Name = "Charlie", Grade = 78.99 
    println!("{:<5} | {:^12} | {:>6} |",id=2,name="Charlie", grade=78.99);
}