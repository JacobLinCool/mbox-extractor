use std::path::PathBuf;

use clap::{arg, value_parser, Command};

pub fn cli() -> Command {
    Command::new("mbox-extractor")
        .version(format!(
            "{} {}",
            env!("VERGEN_GIT_SHA"),
            env!("VERGEN_CARGO_TARGET_TRIPLE")
        ))
        .about("Extract emails from mbox file")
        .author("Jacob Lin <jacob@csie.cool>")
        .args(&[
            arg!([mbox] "mbox file to extract")
                .default_value("-")
                .value_parser(value_parser!(PathBuf)),
            arg!(--output -o <output> "output directory")
                .default_value("./output")
                .value_parser(value_parser!(PathBuf)),
        ])
}
