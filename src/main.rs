

fn main() {
  let foo = Some(5);

  if let Some(value) = foo {
    println!("Value: {}", value);
  }
  
  match foo {
    Some(value) => {
      println!("Value: {}", value);
    },
    None => {
      println!("No value");
    }
  }

 
  foo.map(|x| {
    println!("Value: {}", x);
  });

  foo.filter(|x| *x< 10);
 
}
  