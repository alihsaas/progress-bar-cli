use console::Term;
use std::{convert::*, io, thread, time::Duration, time::Instant};

#[derive(Debug)]
struct ProgressBar {
    target: u64,
    current: u64,
    term: Term,
    progress_chars: String,
    format: String,
    elapsed_time: Instant,
}

impl ProgressBar {
    fn new(target: u64) -> ProgressBar {
        ProgressBar {
            target,
            current: 0,
            term: Term::stdout(),
            format: String::from("{progress}"),
            progress_chars: String::from("#>-"),
            elapsed_time: Instant::now(),
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
            let elapsed = self.elapsed_time.elapsed().as_secs();
            let (hour, min, sec) = (elapsed / 3600, (elapsed / 60) % 60, elapsed % 60);

            let formated = self
                .format
                .replace(
                    "{elapsed_time}",
                    &format!("{:02}:{:02}:{:02}", hour, min, sec),
                )
                .replace("{target}", &self.target.to_string())
                .replace("{current}", &self.current.to_string())
                .replace(
                    "{percentage}",
                    &format!("{}%", (self.current * 100) / self.target),
                );

            let progress_length = u64::try_from(w).unwrap();
            let progress_length: u64 = progress_length
                .wrapping_sub(formated.replace("{progress}", "").len().try_into().unwrap());

            let numeric_progress: u64 = (self.current * (progress_length)) / self.target;
            let progress_left = progress_length - numeric_progress;
            let mut progress_chars = self.progress_chars.chars();
            let progress = format!(
                "{}{}{}",
                progress_chars
                    .next()
                    .unwrap()
                    .to_string()
                    .repeat(usize::try_from(numeric_progress).unwrap() - 1usize),
                progress_chars.next().unwrap().to_string(),
                progress_chars
                    .next()
                    .unwrap()
                    .to_string()
                    .repeat(progress_left.try_into().unwrap())
            );
            let formated = formated.replace("{progress}", &progress);

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

fn main() -> io::Result<()> {
    let mut progress_bar = ProgressBar::new(1000);
    progress_bar
        .set_format("[{elapsed_time}] {progress} {current}/{target} : {percentage}")
        .set_progress_chars("█>░");

    loop {
        progress_bar.increment(100);
        thread::sleep(Duration::from_secs(1));
        if progress_bar.current == progress_bar.target {
            break;
        }
    }

    Ok(())
}
