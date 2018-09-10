# github-local-remote
[![crates.io version][1]][2] [![build status][3]][4]
[![downloads][5]][6] [![docs.rs docs][7]][8]

Find the GitHub url, repo and username for a local directory.

- [Documentation][8]
- [Crates.io][2]
- [Releases][9]

## Usage
```rust
extern crate github_local_remote;

fn main() {
  let res = github_local_remote::stat(".").unwrap();
  println!("result {:?}", res);
}
```

## Installation
```sh
$ cargo add github-local-remote
```

## License
[MIT](./LICENSE-MIT) OR [Apache-2.0](./LICENSE-APACHE)

[1]: https://img.shields.io/crates/v/github-local-remote.svg?style=flat-square
[2]: https://crates.io/crates/github-local-remote
[3]: https://img.shields.io/travis/yoshuawuyts/github-local-remote.svg?style=flat-square
[4]: https://travis-ci.org/yoshuawuyts/github-local-remote
[5]: https://img.shields.io/crates/d/github-local-remote.svg?style=flat-square
[6]: https://crates.io/crates/github-local-remote
[7]: https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square
[8]: https://docs.rs/github-local-remote
[9]: https://github.com/yoshuawuyts/github-local-remote/releases
