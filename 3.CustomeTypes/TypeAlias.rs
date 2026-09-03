enum VeryVerboseEnumOfThingsToDoWithNumbers{
    Add,
    Subtract,
}

// Create a Type Alias
type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

fn main(){
    // We can refer each variant via its alias not it long and inconvenitent 
    // name
    let x = Operations::Add;
}