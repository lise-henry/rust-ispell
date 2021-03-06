// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with
// this file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::process::Command;
use std::process::Stdio;

use spell_checker::SpellChecker;
use error::{Result, Error};

/// Spell Launcher wizard (ah, ah). A builder for `SpellChecker`.
///
/// Runs `ispell` or one of its variant.
///
/// # Examples
///
/// * Launches `ispell` with `british` dictionary:
///
/// ```no_run
/// use ispell::SpellLauncher;
/// let checker = SpellLauncher::new()
///               .dictionary("british")
///               .launch()
///               .unwrap();
/// ```
///
/// * Launches `aspell` with french (France) language:
/// 
/// ```
/// use ispell::SpellLauncher;
/// let checker = SpellLauncher::new()
///               .aspell()
///               .dictionary("fr_FR")
///               .launch()
///               .unwrap();
/// ```
#[derive(Debug)]
pub struct SpellLauncher {
    lang: Option<String>,
    command: Option<String>,
    mode: Mode,
    timeout: u64,
}

#[derive(Debug)]
enum Mode {
    Ispell,
    Aspell,
    Hunspell,
}

impl SpellLauncher {
    /// Creates a new spell checker with default options
    pub fn new() -> SpellLauncher {
        SpellLauncher {
            lang: None,
            command: None,
            mode: Mode::Ispell,
            timeout: 1000,
        }
    }

    /// Sets mode to aspell instead of ispell.
    ///
    /// Will run `aspell` as the command if it is not set
    pub fn aspell(&mut self) -> &mut SpellLauncher {
        self.mode = Mode::Aspell;
        self
    }

    /// Sets compatibility mode to hunspell instead of ispell.
    ///
    /// Will run `hunspell` as the command if it is not set
    pub fn hunspell(&mut self) -> &mut SpellLauncher {
        self.mode = Mode::Hunspell;
        self
    }

    /// Sets compatibility mode to ispell
    ///
    /// Will run `ispell` as the command if it is not set
    /// (default setting)
    pub fn ispell(&mut self) -> &mut SpellLauncher {
        self.mode = Mode::Ispell;
        self
    }

    
    /// Sets the timeout when checking ispell
    ///
    /// If the spawned process takes longer than this timeout to answer to a query,
    /// it will be killed and an error will be returned, preventing your program
    /// from freezing indefinitely.
    ///
    /// The timeout is set in milliseconds, and is 1000 (a second) by default.
    pub fn timeout(&mut self, timeout: u64) -> &mut SpellLauncher {
        self.timeout = timeout;
        self
    }
    
    /// Set the name of the command to run
    ///
    /// By default, it inferred from the mode (which is `ispell` by default).
    ///
    /// Unless you want to run a specific (ispell-compatible) command, you shouldn't use this method directly, but rather
    /// use the `aspell` or `hunspell` methods, since this also allow the the library to know which actual program
    /// is runned and to set encoding options accordingly.
    pub fn command<S: Into<String>>(&mut self, command: S) -> &mut SpellLauncher {
        self.command = Some(command.into());
        self
    }

    /// Determine the dictionary that should be used.
    ///
    /// Note that `ispell`, `hunspell` and `aspell` have different naming schemes:
    ///
    /// * `ispell` accepts full names, e.g. "american", "british", "french", ...
    /// * `hunspell` accepts unicode language codes, e.g. "fr_FR", "en_GB", ...
    /// * `aspell` accepts both.
    ///
    /// # Example
    ///
    /// ```
    /// use ispell::SpellLauncher;
    /// let checker = SpellLauncher::new()
    ///               .aspell()
    ///               .dictionary("en_GB")
    ///               .launch()
    ///               .unwrap();
    /// ```
    pub fn dictionary<S: Into<String>>(&mut self, lang: S) -> &mut SpellLauncher {
        self.lang = Some(lang.into());
        self
    }
    
    /// Launch `ispell` (or `aspell` or `hunspell`) and return a `SpellChecker`
    pub fn launch(&self) -> Result<SpellChecker> {
        let command_name: &str = if let Some(ref command) = self.command {
            command
        } else {
            match self.mode {
                Mode::Ispell => "ispell",
                Mode::Aspell => "aspell",
                Mode::Hunspell => "hunspell",
            }
        };
        let mut command = Command::new(command_name);
        command.arg("-a")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped());
        if let Some(ref lang) = self.lang {
            command.arg("-d")
                .arg(lang);
        }
        // Try to set encoding to utf-8
        if self.command.is_none() { // only do this if it isn't a custom command
            match self.mode {
                Mode::Hunspell => command.args(&["-i", "utf-8"]),
                Mode::Aspell => command.arg("--encoding=utf-8"),
                Mode::Ispell => command.arg("-Tutf8"),
            };
        }

        let res = command.spawn();

        match res {
            Ok(child) => SpellChecker::new(child, self.timeout),
            Err(err) => Err(Error::process(format!("could not successfully spawn process '{}': {}", command_name, err)))
        }
    }
}
