#[doc="this is hello section."]

fn hello() -> str {
  ret "hello";
}

#[test]
fn test_hello() {
  assert(hello() == "hello");
}