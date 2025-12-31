/*
Clipboard utility
*/

use arboard::{ Clipboard };
use std::thread;
use std::time::Duration;

/// copy text to clipboard
pub fn clip(text: &String) -> Result<(), arboard::Error> {
    let mut clipboard = Clipboard::new()?;

    clipboard.set_text(text.to_owned())?;
    // Keep the clipboard alive for a short duration to ensure the data is retained
    thread::sleep(Duration::from_millis(500));

    Ok(())
}