use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[clap(short, long, default_value_t = String::from("none"), help="possible values [none, utf8, ascii]")]
    pub border_type: String,
}
