use std::{
    convert::*,
    io,
    thread,
    time::Duration
};
use console::Term;

struct ProgressBar {
    target: u64,
    current: u64,
    term: Term,
    progress_chars: String,
    format: String,
}

impl ProgressBar {
    fn new(target: u64) -> ProgressBar {
        ProgressBar {
            target,
            current: 0,
            term: Term::stdout(),
            format: String::from("{progress}"),
            progress_chars: String::from("#>-"),
        }
    }

    fn set_format(&mut self, format: &str) -> &mut ProgressBar {
        self.format = String::from(format);
        self
    }

    fn set_progress_chars(&mut self, chars: &str) -> &mut ProgressBar {
        self.progress_chars = String::from(chars);
        self
    }

    fn draw(&self) -> io::Result<()> {
        let size = self.term.size_checked();

        if let Some((_h, w)) = size {

            let progress_length = u64::try_from(w).unwrap();
            let progress_length: u64 = progress_length.wrapping_sub(self.format.replace("{progress}", "").len().try_into().unwrap());

            let numeric_progress: u64 = (self.current * (progress_length)) / self.target;
            let progress_left = progress_length - numeric_progress;
            let progress = format!("{}{}{}", 
                self.progress_chars[0..1].to_string().repeat(usize::try_from(numeric_progress).unwrap() - 1usize),
                self.progress_chars[1..2].to_string(),
                self.progress_chars[2..3].to_string().repeat(progress_left.try_into().unwrap())
            );
            let formated = self.format.replace("{progress}", progress.as_str());
            self.term.clear_line()?;
            self.term.write_str(&formated)?;
        } else {
            println!("Unable to get terminal size!")
        }

        Ok(())
    }

    fn increment(&mut self, amount: u64) {
        self.current = (self.current + amount).min(self.target);
        self.draw().unwrap();
    }
}


fn main() -> io::Result<()>{

    let mut progress_bar = ProgressBar::new(1000);
    progress_bar
        .set_format("Hello {progress} World")
        .set_progress_chars("M>~");

    loop {

        progress_bar.increment(100);
        thread::sleep(Duration::from_secs(1));
        if progress_bar.current == progress_bar.target {
            break
        }
    }

    Ok(())
}
