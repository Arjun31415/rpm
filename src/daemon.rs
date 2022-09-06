use crate::{error::Result, utils};
use daemonize::Daemonize;
use std::fs::OpenOptions;

pub fn daemonize(f: fn()) -> Result<()> {
    let running_as_sudo = utils::running_as_sudo();

    let username = if running_as_sudo {
        "root".into()
    } else {
        utils::get_username().unwrap_or_else(|| "nobody".into())
    };

    let open_opts = OpenOptions::new()
        .truncate(false)
        .create(true)
        .write(true)
        .to_owned();

    let (stdout_path, stderr_path, pidfile_path) = if running_as_sudo {
        ("/var/log/rpm.out", "/var/log/rpm.err", "/var/run/rpm.pid")
    } else {
        ("/tmp/rpm.out", "/tmp/rpm.err", "/tmp/rpm.pid")
    };

    let stdout = open_opts.open(stdout_path)?;
    let stderr = open_opts.open(stderr_path)?;

    let daemonize = Daemonize::new()
        .user(&*username)
        .pid_file(pidfile_path)
        .chown_pid_file(false)
        .working_directory("/tmp")
        .stdout(stdout)
        .stderr(stderr);
    println!("Daemonizing");
    match daemonize.start() {
        Ok(_) => {
            log::info!("User {} has started the daemon successfully.", username);
            f();
        }
        Err(err) => {
            eprintln!("Error, {}", err);
        }
    }

    Ok(())
}
