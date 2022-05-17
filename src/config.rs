use clap::Parser;

#[derive(Debug, Parser)]
#[clap(about = "Turbo-md parser, -h for help")]
pub struct Args {
    #[clap(help = "test")]
    pub entry_file: String,

    #[clap(arg_enum, default_value_t, required = false)]
    pub option: RunOption,
}

#[derive(Debug, clap::ArgEnum, Clone)]
pub enum RunOption {
    Html,
    Ast,
}

impl Default for RunOption {
    fn default() -> Self {
        Self::Html
    }
}
