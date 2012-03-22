#[doc="this is world section"]

fn world() -> str {
  ret "world";
}

#[test]
fn test_world() {
  assert(world() == "world");
}