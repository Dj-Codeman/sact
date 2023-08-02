mod config;
mod pretty;

use std::process::exit;
use crate::pretty::*;

fn main() {
    let username = sact::get_username().unwrap_or_else(|| {
        output("RED", "=== Failed to get username \n");
        exit(1);
    });
    let cfg = config::get_config(&username).unwrap_or_else(|| {
        output("RED", "=== sact is not configured for this user.");
        exit(1);
    });
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        if sact::first_run() {
            // create the root user

        }
        let mut msg:String = String::new();
        msg.push_str("Usage: ");
        msg.push_str(&args[0].to_string());
        msg.push_str(" <cmd> <args...>");
        output("BLUE", &msg);
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
                let mut msg = String::new();
                msg.push_str("=== ");
                msg.push_str(&args[1]);
                msg.push_str(" command not found");
                output("RED", &msg);
                exit(1)
            })
        },
    );
    let cmdp = cmdp
        .unwrap_or_else(|_| {
            let mut msg = String::new();
            msg.push_str("=== ");
            msg.push_str(&args[1]);
            msg.push_str(" command not found");
            output("RED", &msg);
            exit(1);
        })
        .display()
        .to_string();
    if !sact::is_root() {
        warn("sact needs setuid to work");
        exit(1);
    }
    match cfg.get_perm(&cmdp) {
        config::Perm::Disallow => {
            let mut msg = String::new();
            msg.push_str("=== ");
            msg.push_str(&username);
            msg.push_str(" is not allowed to execute : ");
            msg.push_str(&cmdp);
            warn(&msg);
        }
        config::Perm::AllowPass => {
            let mut correct = false;
            let max_attempts = 4;
            let nopass = sact::check_pass_time(&username).unwrap_or_else(|| {
                notice("Failed to check last password time, asking for password");
                false
            });
            if !nopass {
                for i in 0..max_attempts {
                    let pass = sact::read_password(&format!(
                        "[sact] password for {}, attempt {} / {}",
                        username,
                        i + 1,
                        max_attempts
                    ))
                    .unwrap_or_else(|| {
                        output("RED", "=== Failed to read password");
                        exit(1);
                    });
                    let passed = sact::check_password(&username, &pass).unwrap_or_else(|| {
                        output("RED", "=== Failed to verify password");
                        exit(1);
                    });
                    if passed {
                        correct = true;
                        break;
                    } else {
                        output("RED", "=== Incorrect password, try again");
                    }
                }
                if !correct {
                    output("RED", "=== Incorrect password, try again");
                    exit(1);
                }
            }
            sact::update_pass_time(&username)
                .or_else(|| Some(warn("Failed to update last password time")));
        }
        _ => {}
    };
    // ! the superuser doing part
    unsafe { libc::setuid(0) };
    let ret = std::process::Command::new(cmdp)
        .args(&args[2..])
        .spawn()
        .unwrap_or_else(|_| {
            output("RED", "=== Failed to execute command");
            exit(1)
        })
        .wait()
        .unwrap_or_else(|_| {
            output("RED", "=== Failed to wait for command");
            exit(1)
        })
        .code();
    exit(ret.unwrap_or_else(|| {
            warn("Failed to get command return value");
            exit(1)
    }));
}
