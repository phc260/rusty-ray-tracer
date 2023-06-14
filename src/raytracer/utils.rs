use indicatif::{ProgressBar, ProgressStyle};

pub fn get_progress_bar(len: u64) -> ProgressBar {
    let pbar = ProgressBar::new(len);
    pbar.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}]\
            [{wide_bar:.cyan/blue}] {percent}% ({eta})")
        .unwrap()
        .progress_chars("#>-"));
    return pbar;
}
