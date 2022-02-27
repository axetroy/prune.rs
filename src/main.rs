mod ruler;
mod walker;

use clap::{Arg, Command};
use futures::executor;
use ruler::parse_rules;
use std::{env, path::Path};
use walker::walk;

fn main() {
    let matches = Command::new("prune")
        .bin_name("prune")
        .version("v0.1.0")
        .author("Axetroy <axetroy.dev@gmail.com>")
        .about("Streamline your disk space and delete some unnecessary files")
        .arg(
            Arg::new("remove")
                .short('r')
                .long("remove")
                .takes_value(false)
                .help("remove file, defaults check only"),
        )
        .arg(
            Arg::new("ROOT")
                .help("The target directory you want to prune")
                .required(true)
                .index(1),
        )
        .get_matches();

    // You can check the value provided by positional arguments, or option arguments
    let root = if let Some(i) = matches.value_of("ROOT") {
        Path::new(i).to_path_buf()
    } else {
        env::current_dir().unwrap()
    };

    let mut rulers = parse_rules();

    // You can check the value provided by positional arguments, or option arguments
    if matches.is_present("remove") {
        rulers.check_only = &false;
    } else {
        rulers.check_only = &true;
    }

    let f = walk(root, &rulers);

    executor::block_on(f);
}
