mod governor;
use clap::Parser;
#[derive(Parser, Debug)]
#[clap(author="Arjun ",version,about,long_about=None)]
struct Args {
    /// Set the governor mode
    #[clap(short = 's', long = "set_mode", value_parser, default_value = "")]
    mode: String,
    /// List the available modes supported by the governor
    #[clap(short, long)]
    list_modes: bool,
    /// Show the current governor mode
    #[clap(short, long = "get_current_mode")]
    current_mode: bool,
}
fn main() {
    let args = Args::parse();
    let gov = governor::governor::Governor::new();
    if !args.mode.is_empty() && args.list_modes
        || args.list_modes && args.current_mode
        || args.current_mode && !args.mode.is_empty()
    {
        println!("You can only select one option");
        return;
    }
    if args.list_modes {
        println!("{}", gov.get_modes().join(", ").to_string());
    } else if args.mode.len() > 0 {
        gov.set_mode(args.mode);
    } else if args.current_mode {
        println!("{:?}", gov.get_current_mode());
    }
}
