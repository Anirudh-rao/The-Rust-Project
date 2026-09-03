enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}


impl VeryVerboseEnumOfThingsToDoWithNumbers{
    fn run(&self, x:i32, y:i32) -> i32{
        match self{
            Self::Add => x + y,
            Self:: Subtract => x - y,
        }
    }
}

type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;


fn main(){
    // We can refer each variant via its alias not it long and inconvenitent 
    // name
    let x = Operations::Add;
}