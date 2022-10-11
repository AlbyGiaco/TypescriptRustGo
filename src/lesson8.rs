fn error_me(throw: bool) -> Result<(), usize> {
    if throw {
        return Err(5);
    } 
       return Ok(());
    
}

fn main() -> Result<(), usize> {
    print!("Hello, world!");

    let value  = error_me(false)?;

    // let value = match error_me(true) {
    //     Ok(v) => Ok((v)),
    //     Err(e) => return (Err(e)),
    // };

    if error_me(true).is_ok() {
        println!("Value: exists");
    } else {
        println!("Value: does not exist");
    }

    return Ok(());
}