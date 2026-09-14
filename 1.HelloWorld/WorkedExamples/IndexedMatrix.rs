use std::fmt;


struct Vector3D([f64;3]);


impl fmt::Display for Vector3D{
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result{
        write!(f,"[")?;
        for (index, val) in self.0.iter().enumerate(){
            if index > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}: {}", index, val)?;
        }
        write!(f,"]")
    }
}

fn main(){
    let vec = Vector3D([1.5, 2.0, -2.4]);
    println!("Vector:{}", vec);
}