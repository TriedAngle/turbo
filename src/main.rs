use crate::config::{Args, RunOption};
use clap::Parser;
use std::io::Write;
use turbo_md::TurboTree;

mod config;

fn main() {
    let args: Args = Args::parse();

    match args.option {
        RunOption::Html => {
            let file_name = args.entry_file.split(".").collect::<Vec<&str>>()[0];
            let parse = turbo_md::parse_file(&args.entry_file);
            let ast = TurboTree::generate(parse);
            let html = ast.to_html();
            let mut output = std::fs::OpenOptions::new()
                .write(true)
                .truncate(true)
                .create(true)
                .open(&format!("{file_name}.html"))
                .unwrap();
            output.write_all(html.as_bytes()).expect("lmao?");
        }
        RunOption::Ast => {
            let parse = turbo_md::parse_file(&args.entry_file);
            let ast = TurboTree::generate(parse);
            println!("{}", ast);
        }
    }
}
