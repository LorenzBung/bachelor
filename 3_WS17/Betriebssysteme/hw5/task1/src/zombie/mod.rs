use nix::sys::signal::*;
use nix::unistd::{fork, ForkResult};
use std::process::Command;
use std::{thread, time};

/// This function creates a **Zombie Process** and shows it using `ps t`.
pub fn run_zombie() {
    match fork().expect("Fork failed") {
        // In case this is the Parent process, kill the new child.
        ForkResult::Parent { child } => {
            kill(child, SIGKILL).expect("Kill failed");
        }

        // In case this is the Child process, wait until killed.
        ForkResult::Child => {
            thread::sleep(
                time::Duration::new(5000, 0), // Wait for a long time
            );
        }
    }

    // Run `ps t` to show the Zombie Process
    let output = Command::new("ps")
        .arg("t")
        .output()
        .expect("Error in run_zombie");

    // Print the output of `ps t` to `stdout`
    println!("{}", String::from_utf8_lossy(&output.stdout));
}
