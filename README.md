# loadingbar
A simple, customizable loading bar for the terminal written in rust

## Usage

### Default loading bar
```rust
use loadingbar::LoadingBar;

fn main() {
    let bar = LoadingBar::new();
    bar.start();
}
```

### Custom loading bar
```rust
use loadingbar::LoadingBar;

fn main() {
    let bar = LoadingBar::new_with_config(
        std::time::Duration::from_secs(10), // how long the bar will take to complete
        '*', // the character to use for the progress bar
        30, // the length of the bar in characters
        String::from("Loading.. "), // prefix message (Loading.. [##########      ])
    );
    bar.start();
}
