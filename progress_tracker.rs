use std::fs::{self, File};
use std::io::{self, BufRead, Write};

const TOTAL_EXERCISES: u32 = 94;

fn main() -> io::Result<()> {
    // Read the progress file
    let progress = fs::read_to_string(".rustlings-state.txt")?;

    // Count the number of completed exercises
    let mut completed = progress.lines().count() as u32;
    completed = completed - 4;

    // Calculate progress percentage
    let progress_percentage = (completed as f32 / TOTAL_EXERCISES as f32) * 100.0;

    // Generate the progress bar
    let progress_bar = format!(
        "`================> {:.0}/{}`",
        progress_percentage, TOTAL_EXERCISES
    );

    // Read the existing README.md
    let readme_content = fs::read_to_string("README.md")?;

    // Replace the placeholder with the progress bar
    let updated_readme = readme_content.replace("<<PROGRESS-BAR-HERE>>", &progress_bar);

    // Write the updated content back to README.md
    let mut readme_file = File::create("README.md")?;
    write!(readme_file, "{}", updated_readme)?;

    println!("README updated with progress: {}", progress_bar);

    Ok(())
}
