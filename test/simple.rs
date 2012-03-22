use std;
use helloworld;

#[test]
fn simple(){
  assert(helloworld::helloworld() == "helloworld");
}
