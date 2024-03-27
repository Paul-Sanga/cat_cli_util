#![allow(dead_code)]

extern crate clap;
use clap::Parser;
use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};

#[derive(Parser)]
pub struct Cat {
    #[arg(short, long, value_name = "SOURCE FILE")]
    pub source_files: Vec<PathBuf>,
    #[arg(short, long, value_name = "DESTINATION FILE")]
    pub destination_file: Option<PathBuf>,
}

pub struct CatImpl<'a> {
    cat_cli: &'a Cat,
}

impl<'a> CatImpl<'a> {
    pub fn new(cat_cli: &'a Cat) -> Self {
        Self { cat_cli }
    }

    fn dest_file_existance(&self) -> File {
        if let Some(dest_file) = &self.cat_cli.destination_file {
            if !dest_file.exists() {
                if let Ok(file) = File::create(dest_file) {
                    println!("Creating destination file");
                    file
                } else {
                    panic!("\x1b[31m Error reading file. \x1b[0m")
                }
            } else {
                if let Ok(file) = File::open(dest_file) {
                    file
                } else {
                    panic!("\x1b[31m Error reading file. \x1b[0m")
                }
            }
        } else {
            panic!("\x1b[31m Destination file flag was not specified \x1b[0m")
        }
    }

    pub fn print_src_contents(&self) {
        let mut output: String = String::new();
        self.cat_cli.source_files.iter().for_each(|e| {
            if e.exists() {
                if let Ok(mut file) = File::open(e) {
                    match file.read_to_string(&mut output) {
                        Ok(_) => {}
                        Err(error) => {
                            println!("Error reading file: {error}")
                        }
                    }
                }
            } else {
                println!(
                    "\x1b[31m {}: No such file or directory \x1b[0m",
                    e.to_str().unwrap()
                )
            }
        });
        println!("\x1b[32m {output} \x1b[0m");
    }

    pub fn concat_files(&self) {
        let mut result: String = String::new();
        self.cat_cli.source_files.iter().for_each(|path| {
            if !path.exists() {
                panic!(
                    "\x1b[31m {}: No such file or directory \x1b[0m",
                    path.to_str().unwrap()
                );
            } else {
                if let Ok(mut file) = File::open(path) {
                    match file.read_to_string(&mut result) {
                        Ok(_) => {}
                        Err(error) => {
                            println!("\x1b[31m Error reading file: {} \x1b[0m", error)
                        }
                    }
                }
            }
        });
        
        if !result.is_empty(){
            let mut dest_file = self.dest_file_existance();
            match dest_file.write_all(&mut result.as_bytes()) {
                Ok(_) => {}
                Err(error) => {
                    println!("\x1b[31m Error reading file: {} \x1b[0m", error)
                }
            }
        }
    }
}
