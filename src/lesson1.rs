fn main() {
    let mut a = vec![];
    let b = &mut a;

    b.push(1);
    
    println!("{:?}", b);
}