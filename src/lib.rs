#![cfg_attr(feature = "nightly", deny(missing_docs))]
#![cfg_attr(feature = "nightly", feature(external_doc))]
#![cfg_attr(feature = "nightly", doc(include = "../README.md"))]
#![cfg_attr(test, deny(warnings))]
#![forbid(unsafe_code, missing_debug_implementations)]

#[macro_use]
extern crate failure;
extern crate git2;

mod error;

pub use error::{Error, ErrorKind, Result};

use failure::ResultExt;
use git2::Repository;
use std::convert::AsRef;
use std::path::Path;

/// Remote represenation.
#[derive(Debug, Clone)]
pub struct Remote {
  url: String,
  user: String,
  repo: String,
}

impl Remote {
  /// Get the url.
  pub fn url(&self) -> &str {
    &self.url
  }

  /// Get the user.
  pub fn user(&self) -> &str {
    &self.user
  }

  /// Get the repo.
  pub fn repo(&self) -> &str {
    &self.repo
  }
}

/// Find out what the GitHub url, user and repo are for a directory
pub fn stat(path: impl AsRef<Path>) -> ::Result<Remote> {
  let path = path.as_ref();
  let repo = Repository::open(path).context(::ErrorKind::Git)?;
  let remote = repo
    .find_remote("origin")
    .context(::ErrorKind::GitRemoteOrigin)?;
  let url = remote.url().ok_or(::ErrorKind::GitRemoteUrl)?;
  println!("url {:?}", url);
  unimplemented!();
}
