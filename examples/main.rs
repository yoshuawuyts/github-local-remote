extern crate github_local_remote;

fn main() {
  let res = github_local_remote::stat(".").unwrap();
  println!("result {:?}", res);
}
