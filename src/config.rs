use std::collections::HashMap;
pub struct ConfigItem {
    /// An array of paths to binaries that the user can execute without entering
    /// a password, if None, all commands will be no_pass.
    pub no_pass: Option<&'static [&'static str]>,
    /// An array of paths to binaries that the user can execute by entering
    /// a password, if None, all commands will be pass. No pass is checked before
    /// pass
    pub pass: Option<&'static [&'static str]>,
}

/// Creates the configuration. It should return a hashmap between the username and
/// the user's configuration item.
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

// cut here for user config

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
pub fn get_perm(cfg: ConfigItem, cmd: &str) -> Perm {
    fn allow(cfg: Option<&'static [&'static str]>, cmd: &str) -> bool {
        match cfg {
            Some(x) => x.contains(&cmd),
            None => true,
        }
    }
    if allow(cfg.no_pass, cmd) {
        Perm::AllowNoPass
    } else if allow(cfg.pass, cmd) {
        Perm::AllowPass
    } else {
        Perm::Disallow
    }
}
