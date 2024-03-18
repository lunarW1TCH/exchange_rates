use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, required = false)]
    test: Option<String>,

    #[arg(short, long, exclusive = true)]
    list: bool,
}
