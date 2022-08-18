pub mod governor {
    extern crate notify;
    use notify::{watcher, RecursiveMode, Watcher};
    use std::sync::mpsc::channel;
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::time::Duration;
    use glob::glob;
    use std::fs;
    use std::io::{ErrorKind, Write};
    /**
     * Governor struct which will handle the PC's governor
     *
     */
    #[derive(Debug, Clone)]
    pub struct Governor {
        modes: Vec<String>,
        current_mode: String,
    }
    impl Governor {
        pub fn get_modes(&self) -> Vec<String> {
            return self.modes.clone();
        }
        fn _set_current_mode(&mut self, mode: String) {
            self.current_mode = mode;
        }

        pub fn _subscribe_to_file(gov_mutex: &Arc<Mutex<Governor>>, file_path: &str) {
            let (tx, rx) = channel();
            let mut watcher = watcher(tx, Duration::from_millis(100)).unwrap();
            watcher
                .watch(file_path, RecursiveMode::NonRecursive)
                .unwrap();
            println!("Watching {}", file_path);
            loop {
                println!("Something: {:?}", rx.recv().unwrap());
                match rx.recv() {
                    Ok(event) => {
                        println!("{:?}", event);
                        let mut gov = gov_mutex.lock().unwrap();
                        let mode = Self::_get_current_mode();
                        gov._set_current_mode(mode);
                    }
                    Err(e) => println!("{:?}", e),
                }
            }
        }
        fn _get_current_mode() -> String {
            // read the file `/sys/devices/system/cpu/cpu0/cpufreq/scaling_governor`
            // and return the current governor
            const FILE_PATH: &str = "/sys/devices/system/cpu/cpu0/cpufreq/scaling_governor";
            let mode_mutex = Arc::new(Mutex::new(String::new()));
            let mode_mutex_clone = Arc::clone(&mode_mutex);
            let read_thread = thread::spawn(move || {
                let mut mode_val = mode_mutex_clone.lock().unwrap();
                let contents = fs::read_to_string(FILE_PATH).unwrap().trim().to_string();
                *mode_val = contents;
            });
            read_thread.join().unwrap();
            let mode = Arc::try_unwrap(mode_mutex).unwrap().into_inner().unwrap();
            return mode;
        }
        pub fn get_current_mode(&self) -> String {
            return self.current_mode.clone();
        }

        pub fn new() -> Governor {
            // read the file `/sys/devices/system/cpu/cpu0/cpufreq/scaling_available_governors`
            // and return a vector of governors
            let file_path = "/sys/devices/system/cpu/cpu0/cpufreq/scaling_available_governors";
            // remove new line from the string
            let contents = fs::read_to_string(file_path).unwrap().trim().to_string();
            return Governor {
                modes: contents.split(' ').map(|s| s.to_string()).collect(),
                current_mode: Governor::_get_current_mode(),
            };
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
        pub fn set_governor_file_mode(&self, mode: String) {
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
