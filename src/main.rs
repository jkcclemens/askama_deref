#[macro_use]
extern crate askama;

use askama::Template;

mod base;

use base::*;

// remove `pub` and this compiles
#[derive(Template)]
#[template(path = "test.html")]
pub struct Test {
  pub _parent: Base,
}

fn main() {
  let t = Test {
    _parent: Base {
      henlo: Some("hi".into()),
    },
  };
  println!("{}", t.render().unwrap());
}
