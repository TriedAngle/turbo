use crate::config::{Args, RunOption};
use clap::Parser;
use turbo_md::TurboTree;

mod config;

fn main() {
    let args: Args = Args::parse();

    match args.option {
        RunOption::Html => {}
        RunOption::Ast => {
            let parse = turbo_md::parse_file(&args.entry_file);
            let ast = TurboTree::generate(parse);
            println!("{}", ast);
        }
    }
}
