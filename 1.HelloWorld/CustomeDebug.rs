use std::fmt;

struct SecretAgent{
    codename:String,
    real_name: String,
}

impl fmt::Debug for SecretAgent{
    fn fmt(&self, f:&mut fmt::Formatter <'_>) -> fmt::Result{
        f.debug_struct("SecretAgent")
        .field("codename", &self.codename)
        .field("real_name", &"[REDACTED]")
        .finish()
    }
}

fn main(){
    let agent = SecretAgent{
        codename: String::from("007"),
        real_name: String::from("Jame Bond"),
    };
    println!("{:?}", agent)
}