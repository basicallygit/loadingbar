//! A simple, customizable loading bar for the terminal.
//! 
//! This crate provides a simple loading bar for the terminal that
//! allows for easy customization.
//! 
//! The loading bar will only work on terminals that support ANSI escape codes.
//! (Most modern terminals do except for Windows Command Prompt)
//! 
//! # Examples
//! 
//! ### Default configuration
//! ```
//! use loadingbar::LoadingBar;
//! 
//! fn main() {
//!     let bar = LoadingBar::new(); // create a new default loading bar
//!     bar.start(); // starts the loading animation
//! }
//! ```
//! 
//! ### Custom configuration
//! ```
//! use loadingbar::LoadingBar;
//! 
//! fn main() {
//!    let bar = LoadingBar::new_with_config(
//!       std::time::Duration::from_secs(10), // how long the bar will take to complete
//!      '*', // the character to use for the progress bar
//!      30, // the length of the bar in characters
//!      String::from("Loading.. "), // prefix message (Loading.. [##########      ])
//!   );
//! 
//!   bar.start(); // starts the loading animation
//! }
use std::time::Duration;
use std::io::{Write, stdout};

#[derive(Debug)]
pub struct LoadingBar {
    duration: Duration,
    progress_char: char,
    length: u32,
    prefix: String,
}

impl LoadingBar {
    /// Returns a new LoadingBar with default configuration.
    /// 
    /// #### Default values:
    /// * `duration` - 5 seconds
    /// * `progress_char` - '#'
    /// * `length` - 20
    /// * `prefix` - empty string
    /// 
    /// # Example
    /// ```
    /// use loadingbar::LoadingBar;
    /// 
    /// let bar = LoadingBar::new();
    /// ```
    pub fn new() -> Self {
        Self {
            duration: Duration::from_secs(5),
            progress_char: '#',
            length: 20,
            prefix: String::new(),
        }
    }

    /// Returns a new LoadingBar with the specified configuration.
    /// 
    /// # Arguments
    /// 
    /// * `duration` - The duration of the loading bar.
    /// * `progress_char` - The character to use for the progress bar.
    /// * `length` - The length of the progress bar in chars.
    /// * `prefix` - The prefix message to print before the progress bar.
    /// 
    /// # Example
    /// ```
    /// use loadingbar::LoadingBar;
    /// 
    /// let bar = LoadingBar::new_with_config(
    ///    std::time::Duration::from_secs(10), // how long the bar will take to complete
    ///    '*', // the character to use for the progress bar
    ///    30, // the length of the bar in characters
    ///    String::from("Loading.. "), // prefix message (Loading.. [##########      ])
    /// );
    /// ```
    pub fn new_with_config(duration: Duration, progress_char: char, length: u32, prefix: String) -> Self {
        Self {
            duration,
            progress_char,
            length,
            prefix,
        }
    }

    /// Starts the loading animation.
    /// 
    /// # Example
    /// ```
    /// use loadingbar::LoadingBar;
    /// 
    /// let bar = LoadingBar::new();
    /// bar.start();
    /// ```
    pub fn start(&self) {
        let each_duration = self.duration / self.length;

        (1..=self.length).for_each(|i| {
            print!("{}", self.prefix);
            print!("[");
            (1..=i).for_each(|_| print!("{}", self.progress_char));
            (i..self.length).for_each(|_| print!(" "));
            print!("]");
            stdout().flush().unwrap();

            if i != self.length {
                print!("\r");
            } else {
                println!();
            }
            stdout().flush().unwrap();
            std::thread::sleep(each_duration);
        });
    }
}