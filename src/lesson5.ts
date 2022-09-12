type Foo = {
  bar?: string;
};

function doSomething(foo: Foo) {
  if (foo.bar) {
    return true;
  } else {
    return false;
  }
}
