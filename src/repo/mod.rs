use std::fmt::{Debug, Formatter};
use std::fs;
use std::io::Write;

use git2::{Error as GitError, ErrorCode, ErrorCode::NotFound, Remote, Repository};
use log::{error, info, warn};

use crate::error::Error;
use crate::error::Error::Generic;

pub struct GitRepo {
    pub repo_url: String,
    pub repo_directory: String
}

pub trait Repo {
    // Static method signature; `Self` refers to the implementor type.
    fn new() -> Self;
}

impl GitRepo {
    pub fn is_available(&self) -> bool {
        true
    }

    pub fn discover(&self) -> Result<Repository, Error> {
        // Remove all of the previous repo
        info!("---- North Repository {:} ----", self.repo_url);

        match Repository::discover(&self.repo_directory) {
            Ok(repo) => {
                // GitRepo::check_remote_is_the_same();
                Ok (repo)
            }
            Err(_) => Err(Generic)
        }

        // Clone the repo into the repo_directory
    }

    pub fn sync(&self) -> Result<Repository, Error> {
        // Remove all of the previous repo
        info!("---- Syncing Repository {:}", self.repo_url);

        // match Repository::open(&self.repo_directory).unwrap().branch("master").unwrap().

        Ok(Repository::open(&self.repo_directory).unwrap())
    }

    fn check_remote_is_the_same(remote: &Remote, origin: &str) -> bool {
        remote.url().unwrap() == origin
    }

    pub fn clone(&self) -> Result<Repository, Error> {
        info!("Cloning into directory {}", &self.repo_directory);

        match Repository::clone(self.repo_url.as_str(), &self.repo_directory).map_err(|_| ErrorCode::Directory) {
            Ok(repo) => {
                info!("Successfully cloned repo {}", self.repo_url);
                Ok(repo)
            },
            Err(error_code) => {
                error!("Error cloning repo {:?}", error_code);
                Err(Generic)
            }
        }
    }

    pub fn set_repo_url(&mut self, repo_url: String) -> &GitRepo {
        self.repo_url = repo_url;
        self
    }

    pub fn set_repo_directory(&mut self, repo_directory: String) -> &GitRepo {
        self.repo_directory = repo_directory;
        self
    }

}

impl Repo for GitRepo {
    fn new() -> Self {
        GitRepo {
            repo_url: String::from("Test"),
            repo_directory: String::from("")
        }
    }
}