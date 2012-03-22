import hello::*;
import world::*;

fn helloworld() -> str {
  ret hello() + world();
}

#[test]
fn test_helloworld() {
  assert(helloworld() == "helloworld");
}