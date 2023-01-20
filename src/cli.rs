
use std::env;

pub fn args() -> Vec<String> {
  return env::args().collect();
}
