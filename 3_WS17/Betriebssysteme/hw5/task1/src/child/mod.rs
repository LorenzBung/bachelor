use nix::sys::wait::wait;
use nix::sys::wait::WaitStatus;
use nix::unistd::ForkResult::{Child, Parent};
use nix::unistd::{fork, getpid};

mod pstree;

/// Required function.
/// Accepts parameter *start_pid* which will be root-process in the printed pstree.
/// Parses parameter *arg* as the number of forked processes.
pub fn run_childs(start_pid: i32, arg: &str) -> Result<(), String> {
    let count = arg.parse::<u8>();
    match count {
        Ok(value) => {
            if value > 0 {
                fork_children(0, value - 1, start_pid);
            }
            Ok(())
        }
        Err(_) => Err("Failed to parse arguments. PIDs must be decimal.\n".to_string()),
    }
}

/// Private function, which forks specified amount of processes (*count*) through recursion
fn fork_children(count: u8, to: u8, start_pid: i32) {
    let pid = fork();
    match pid {
        Ok(Child) => {
            println!("hello, I am child (pid:{})", getpid());
            if count < to {
                fork_children(count + 1, to, start_pid);
            } else {
                println!();
                pstree::print(start_pid);
            }
        }

        Ok(Parent { child }) => {
            if let Ok(ws) = wait() {
                if let WaitStatus::Exited(child_pid, exit_code) = ws {
                    println!(
                        "I am {} and my child is {}. After I waited for {}, it sent me status {:?}",
                        getpid(),
                        child,
                        child_pid,
                        exit_code
                    );
                }
            }
        }

        Err(_) => {}
    }
}
