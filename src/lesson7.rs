enum Option2<T> {
    None,
    Some(T),
}

impl<T> Option2<T> {
    fn is_some(&self) -> bool {
        return match self {
            Option2::None => false,
            Option2::Some(_) => true,
        }
    }
}

fn main() {
    let foo = Option2::Some(5);

    if foo.is_some() {
        println!("Value: exists");
    } else {
        println!("Value: does not exist");
    }
 }