mod config;
mod pretty;

use std::process::exit;

fn main() {
    let username = sact::get_username().unwrap_or_else(|| {
        eprintln!("Failed to get username");
        exit(1);
    });
    let cfg = config::get_config(&username).unwrap_or_else(|| {
        eprintln!("sact is not configured for {}.", username);
        exit(1);
    });
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <cmd> <args...>", args[0]);
        exit(1);
    }
    let cmdp = std::fs::canonicalize(
        if args[1].starts_with("./")
            || args[1].starts_with("../")
            || args[1].starts_with('/')
            || args[1].starts_with('~')
        {
            args[1].clone()
        } else {
            sact::find_bin(&args[1]).unwrap_or_else(|| {
                eprintln!("{}: command not found", args[1]);
                exit(1)
            })
        },
    );
    let cmdp = cmdp
        .unwrap_or_else(|_| {
            eprintln!("{}: command not found", args[1]);
            exit(1);
        })
        .display()
        .to_string();
    if !sact::is_root() {
        eprintln!("sact needs setuid to work");
        exit(1);
    }
    match cfg.get_perm(&cmdp) {
        config::Perm::Disallow => {
            eprintln!("{} is not allowd to execute {}", username, cmdp)
        }
        config::Perm::AllowPass => {
            let mut correct = false;
            let max_attempts = 4;
            let nopass = sact::check_pass_time(&username).unwrap_or_else(|| {
                eprintln!("Failed to check last password time, asking for password");
                false
            });
            if !nopass {
                for i in 0..max_attempts {
                    let pass = sact::read_password(&format!(
                        "[rhun] password for {}, attempt {} / {}",
                        username,
                        i + 1,
                        max_attempts
                    ))
                    .unwrap_or_else(|| {
                        eprintln!("Failed to read password");
                        exit(1);
                    });
                    let passed = sact::check_password(&username, &pass).unwrap_or_else(|| {
                        eprintln!("Failed to verify password");
                        exit(1);
                    });
                    if passed {
                        correct = true;
                        break;
                    } else {
                        eprintln!("Incorrect password, try again")
                    }
                }
                if !correct {
                    eprintln!("Failed to enter correct password");
                    exit(1);
                }
            }
            sact::update_pass_time(&username)
                .or_else(|| Some(eprintln!("Failed to update last password time")));
        }
        _ => {}
    };
    unsafe { libc::setuid(0) };
    let ret = std::process::Command::new(cmdp)
        .args(&args[2..])
        .spawn()
        .unwrap_or_else(|_| {
            eprintln!("Failed to execute command");
            exit(1)
        })
        .wait()
        .unwrap_or_else(|_| {
            eprintln!("Failed to wait for command");
            exit(1)
        })
        .code();
    exit(ret.unwrap_or_else(|| {
            eprintln!("Failed to get command return value");
            exit(1)
    }));
}
