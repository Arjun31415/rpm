pub mod governor {
    use glob::glob;
    use std::fs;
    use std::io::{ErrorKind, Write};
    /**
     * Governor struct which will handle the PC's governor
     *
     */
    #[derive(Debug)]
    pub struct Governor {
        modes: Vec<String>,
    }
    impl Governor {
        pub fn new() -> Governor {
            // read the file `/sys/devices/system/cpu/cpu0/cpufreq/scaling_available_governors`
            // and return a vector of governors
            let file_path = "/sys/devices/system/cpu/cpu0/cpufreq/scaling_available_governors";
            // remove new line from the string
            let contents = fs::read_to_string(file_path).unwrap().trim().to_string();
            return Governor {
                modes: contents.split(' ').map(|s| s.to_string()).collect(),
            };
        }
        pub fn get_modes(&self) -> Vec<String> {
            return self.modes.clone();
        }
        pub fn get_current_mode(&self) -> String {
            // read the file `/sys/devices/system/cpu/cpu0/cpufreq/scaling_governor`
            // and return the current governor
            let file_path = "/sys/devices/system/cpu/cpu0/cpufreq/scaling_governor";
            // remove new line from the string
            let contents = fs::read_to_string(file_path).unwrap().trim().to_string();
            return contents;
        }
        fn write_mode(&self, path: String, mode: &str) {
            let source_list = fs::OpenOptions::new().write(true).open(path);
            match source_list {
                Err(ioerr) => {
                    let msg: String;
                    match ioerr.kind() {
                        ErrorKind::PermissionDenied => {
                            msg = "permission denied. Run with sudo".to_string();
                        }
                        _ => {
                            msg = format!("Error: {:?}", ioerr);
                        }
                    }
                    panic!("{}", msg);
                }
                Ok(mut source_list) => {
                    // println!("{:?}", source_list);
                    source_list.write(mode.as_bytes()).unwrap();
                    // do things
                }
            }
        }
        pub fn set_mode(&self, mode: String) {
            // write the mode to /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor
            // using globs
            let file_path = "/sys/devices/system/cpu/cpu*/cpufreq/scaling_governor";
            for entry in glob(file_path).expect("Failed to read glob pattern") {
                match entry {
                    Ok(path) => {
                        // println!("{:?}", path.display());
                        self.write_mode(path.display().to_string(), &mode);
                    }
                    Err(e) => println!("{:?}", e),
                }
            }
        }
    }
}
