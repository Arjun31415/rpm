mod governor;
use clap::Parser;
use governor::governor::Governor;
use std::{
    sync::{Arc, Mutex},
    thread, time,
};
#[derive(Parser, Debug)]
#[clap(author="Arjun ",version="0.0.1",about,long_about=None)]
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
    let gov_mutex = Arc::new(Mutex::new(Governor::new()));
    let gov_mutex_clone = Arc::clone(&gov_mutex);

    // Governor::_get_current_mode_v2(temp);
    const FILE_PATH: &str = "/sys/devices/system/cpu/cpu0/cpufreq/scaling_governor";
    thread::spawn(move || {
        Governor::_subscribe_to_file(&gov_mutex_clone, FILE_PATH);
    });
    if !args.mode.is_empty() && args.list_modes
        || args.list_modes && args.current_mode
        || args.current_mode && !args.mode.is_empty()
    {
        println!("You can only select one option");
        return;
    }
    if args.list_modes {
        let gov = gov_mutex.lock().unwrap();
        println!("{}", gov.get_modes().join(", ").to_string());
    } else if args.mode.len() > 0 {
        let gov = gov_mutex.lock().unwrap();
        gov.set_governor_file_mode(args.mode);
    } else if args.current_mode {
        loop {
            let gov = gov_mutex.lock().unwrap();
            println!("Current Mode: {:?}", gov.get_current_mode());
            std::mem::drop(gov);
            let ten_millis = time::Duration::from_millis(1000);
            thread::sleep(ten_millis);
        }
    }
}
