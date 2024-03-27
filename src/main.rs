extern crate clap;

use cat_util::{Cat, CatImpl};
use clap::Parser;

fn main() {
    let cat_cli = Cat::parse();
    if let Some(_) = &cat_cli.destination_file {
        if cat_cli.source_files.is_empty() {
            println!("\x1b[31m Can not use the destination file flag without one or more source file flags  \x1b[0m");
        }
    }
    let cat_instance = CatImpl::new(&cat_cli);
    if !cat_cli.source_files.is_empty() && cat_cli.destination_file == None {
        cat_instance.print_src_contents();
    } else {
        cat_instance.concat_files();
    }
}
