use crate::post_processing::post_processor::PostProcessor;
use std::{
    fs::{File, create_dir_all},
    io::{Error, copy},
    path::Path,
};

pub struct FileBuilder;

impl FileBuilder {
    pub fn open<P: AsRef<Path>>(source: P, on_error: impl Fn(Error)) -> File {
        let source = File::open(source);
        match source {
            Ok(file) => file,
            Err(error) => {
                on_error(error);
                panic!();
            }
        }
    }

    pub fn create(destination: &Path, on_error: impl Fn(Error)) -> File {
        let source = File::create(destination);
        match source {
            Ok(file) => file,
            Err(error) => {
                on_error(error);
                panic!();
            }
        }
    }

    pub fn copy_to(source: &mut File, target: &mut File, on_error: impl Fn(Error)) {
        let result = copy(source, target);
        match result {
            Ok(_) => {
                let sync = target.sync_all();
                match sync {
                    Ok(_) => {}
                    Err(err) => on_error(err),
                }
            }
            Err(err) => on_error(err),
        }
    }

    pub fn create_dir_all(path: &Path, on_error: impl Fn(Error)) {
        let source = create_dir_all(path);
        match source {
            Ok(result) => result,
            Err(error) => {
                on_error(error);
                panic!();
            }
        }
    }
}
