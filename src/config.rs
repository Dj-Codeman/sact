
use serde::{Deserialize, Serialize};

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

// ! For generic user file

#[derive(Serialize, Deserialize)]
pub struct UserConfig<'a> {
    pub user_name: &'a str,
    pub user_id: u32,
    pub pass_level: &'a str, // ? None (no passwd needed), Some (password needed for some commands) All (Password needed for everything)
    pub command_nopass: &'a str,
}

pub fn create_root_user() -> bool {

    // ! Populating the root config

    let root_config: UserConfig = UserConfig {
        user_name: "root",
        user_id: 0,
        pass_level: "none",
        command_nopass: "none",
    };

    // Define path for root configs

    // create path to write the file 

    // make the json pretty 

    // write the json file to the config dir

    // encore encrypt the file

    return true;
}

pub fn create_new_user(name: &str) -> bool {

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
