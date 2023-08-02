use std::{fs::OpenOptions, io::Write};

use recs::encrypt::create_hash;
use serde::{Deserialize, Serialize};

// ? for use in recs calls
pub const PROG: &str = "sact";
pub const PATH: &str = "/etc/sact/";

#[derive(Serialize, Deserialize)]
pub struct UserConfig<'a> {
    pub user_name: &'a str,
    pub user_id: u32,
    pub pass_level: &'a str, // ? None (no passwd needed), Some (password needed for some commands) All (Password needed for everything)
    pub command_nopass: &'a str,
}

pub fn write_config(data: UserConfig) -> Option<bool> {
    // Initialize the recs
    recs::initialize();

    // Getting the config data
    let entry: UserConfig = data;

    // creating the user path
    let mut config_path: String = String::new();
    config_path.push_str(PATH);
    config_path.push_str(&create_hash(&entry.user_name.to_string()));
    config_path.push_str(".s");

    // Make the json pretty
    let pretty_config: String = serde_json::to_string_pretty(&config_path).unwrap();

    // Opening the config file
    let mut config_file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .append(true)
        .open(&config_path.to_string())
        .expect("File could not written to");

    if let Err(_e) = writeln!(config_file, "{}", pretty_config) {
        eprintln!("An error occoured");
        return Some(false);
    };

    // encrypting after writting
    if !recs::insert(
        config_path,
        PROG.to_string(),
        create_hash(&entry.user_name.to_string()),
    )
    .unwrap()
    {
        return Some(false);
    }

    return Some(true);
}

pub fn _create_root_user() -> bool {
    // ! Populating the root config

    let root_config: UserConfig = UserConfig {
        user_name: "root",
        user_id: 0,
        pass_level: "none",
        command_nopass: "none",
    };

    if write_config(root_config).unwrap() { return true }
    return false;
}

pub fn _create_new_user(_name: &str) -> bool {
    // confirm username

    // prompt for password level

    // make new struct

    // create path to write the file

    // make the json pretty

    // write the json file to the config dir

    // encore encrypt the file

    return true;
}

// ! end of the config setup

/// Represents if a command is in no_pass, pass or, neither
pub enum Perm {
    ///The user is not allowed to execute the command
    Disallow,
    ///The user is must type in a password to execute the command
    AllowPass,
    ///The user can execute the command without typing in a password
    AllowNoPass,
}

/// Returns a Perm variant based on the config item corresponding the *user*
/// in *config* for *cmd*
impl ConfigItem {
    pub fn get_perm(&self, cmd: &str) -> Perm {
        fn allow(cfg: Option<&'static [&'static str]>, cmd: &str) -> bool {
            match cfg {
                Some(x) => x.contains(&cmd),
                None => true,
            }
        }
        if allow(self.no_pass, cmd) {
            Perm::AllowNoPass
        } else if allow(self.pass, cmd) {
            Perm::AllowPass
        } else {
            Perm::Disallow
        }
    }
}

// ! DELETE THIS STUFF
pub struct ConfigItem {
    /// An array of paths to binaries that the user can execute without entering
    /// a password, if None, all commands will be no_pass.
    no_pass: Option<&'static [&'static str]>,
    /// An array of paths to binaries that the user can execute by entering
    /// a password, if None, all commands will be pass. No pass is checked before
    /// pass
    pass: Option<&'static [&'static str]>,
}

/// Creates the configuration. Given a username it should return the user's
/// configuration item.
#[inline]
pub fn get_config(name: &str) -> Option<ConfigItem> {
    match name {
        "rhl120" => Some(ConfigItem {
            no_pass: Some(&["/bin/poweroff"]),
            pass: None,
        }),
        "root" => Some(ConfigItem {
            no_pass: None,
            pass: None,
        }),
        _ => None,
    }
}
