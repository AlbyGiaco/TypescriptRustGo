enum RSEnum {
    Foo(fn() -> i32),
   
    Foo2(Option<i32>),
    Bar(String),
    Baz(Vec<String>),
  }

  fn bar() -> i32 {
    return 5;
  }

fn main() {
  let foo = RSEnum::Foo(bar);

  if let RSEnum::Foo(value) = foo {
    println!(value);
  }
  
  match foo {
    RSEnum::Foo2(Some(value)) => println(value()),
    RSEnum::Foo2(None) => println!("foo not"),
    RSEnum::Foo(value) => println!(value),
    _ => println!("default value"),
  }
}
  