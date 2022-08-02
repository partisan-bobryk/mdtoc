use clap::Parser;

#[derive(Parser)]
#[clap(name = "mdtoc")]
#[clap(author = "Maksym Y. <maks@revent.studio>")]
#[clap(version = "0.1.0")]
#[clap(about = "Generate table of contents in a markdown file")]
pub struct Cli {
    #[clap(long, value_parser)]
    pub inbound_source: String,
}
