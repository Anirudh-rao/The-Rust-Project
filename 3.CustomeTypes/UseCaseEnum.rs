// Allow attribute to hide warnings for unused code
#![allow(dead_code)]


enum Stage{
    Beginner,
    Advanced,
}

enum Role{
    Student,
    Teacher,
}

fn main(){
    // Explicitly `use` each name so they are available without
    // manual typing
    use Stage::{Beginner, Advanced};

    // Automatically `use` each name inside `Role`
    use Role::*;

    // Equivalent to `Stage::Beginner`
    let stage = Beginner;

    // Equivalent to `Stage::Advanced`
    let role = Student;


    match stage{
        // Note the lack of scoping because of the explicit `use` above
        Beginner => println!("Beginners are starting their learning journey!"),
        Advanced => println!("Advanced learners are mastering their subject!"),

    }
    match role{
        // Note again the lack of scoping
        Student => println!("Students are aquiring knowledge"),
        Teacher => println!("Teachers are spreading knowledge!"),
    }

}