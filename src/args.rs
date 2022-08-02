use clap::Parser;

#[derive(Parser)]
#[clap(name = "mdtoc")]
#[clap(author = "Maksym Y. <maks@revent.studio>")]
#[clap(version = "0.1.0")]
#[clap(about = "Generate table of contents in a markdown file")]
pub struct Cli {
    /// Location of the markdown file
    #[clap(long, short, value_parser)]
    pub input_file: String,
}
