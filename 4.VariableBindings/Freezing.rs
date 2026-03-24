/*
 When data is bound by the same name immutably,it also freezes.
Frozen data cannot be modified until the immutable binding goes out of scope.
 */ 

fn main(){
    let mut _mutable_integer = 7i32;

    {
        //Shadowing by immutable `_mutable_integer`
        let _mutable_integer =  _mutable_integer;

        //Error _mutable_integer is frozen
        //_mutable_integer = 50;
        
    }
    //Ok `_mutable_integer` is not frozen in this scope
    _mutable_integer = 3;
    println!("The value of Mutable Integer : {}",_mutable_integer);
}
