// Create an `enum` to classify a web event. Note how both
// names and type information together specify the variant:
// `PageLoad != PageUnload` and `KeyPress(char) != Paste(String)`.
// Each is different and independent.

enum Webevent{
    // An Enum variant may either be unit-like
    PageLoad,
    PageUnload,
    // Like tuple structs
    KeyPress(char),
    Paste(String),
    // or c-like structures
    Click {x:i64, y:i64},
}

// A Function which takes a Webevent enu as an argument and
// returns nothing
fn inspect(event: Webevent){
    match event{
        Webevent::PageLoad => println!("page loaded"),
        Webevent::PageUnload => println!("page unloaded"),
        // Destructure `c` from inside the `enum` variant.
        Webevent::KeyPress(c) => println!("pressend '{}'.",c),
        Webevent::Paste(s) => println!("pased \"{}\".",s),
        // Destructure `Click` into `x` and `y`
        Webevent::Click{x,y} =>{
            println!("clicked at x={}, y={}.",x,y);
        }
    }
}

fn main(){
    let pressed = Webevent::KeyPress('x');
    // to_owned() creates an owned `string` from a string slice.
    let pasted = Webevent::Paste("my text".to_owned());
    let click = Webevent::Click{x:30, y:80};
    let load = Webevent::PageLoad;
    let unload = Webevent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);
}