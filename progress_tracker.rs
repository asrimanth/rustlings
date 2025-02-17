use std::fs::{self, File};
use std::io::{self, Write};

const TOTAL_EXERCISES: u32 = 94; // Total number of exercises
const PROGRESS_BAR_WIDTH: usize = 50; // Width of the progress bar

fn main() -> io::Result<()> {
    let progress = fs::read_to_string(".rustlings-state.txt")?;
    let completed = (progress.lines().count() - 4) as u32;
    let progress_percentage = (completed as f32 / TOTAL_EXERCISES as f32) * 100.0;

    let filled = (progress_percentage / 100.0 * PROGRESS_BAR_WIDTH as f32) as usize;
    let empty = PROGRESS_BAR_WIDTH - filled;
    let progress_bar = format!(
        "Progress: [{}{}>{}]  {}/{}",
        "#".repeat(filled),
        "#".repeat(if filled < PROGRESS_BAR_WIDTH { 0 } else { 1 }),
        "-".repeat(empty),
        completed,
        TOTAL_EXERCISES
    );

    let readme_content = fs::read_to_string("README.md")?;
    // Replace the existing progress tracker (if any)
    let updated_readme = if let Some(start) = readme_content.find("Progress: [") {
        let end = readme_content[start..]
            .find('\n')
            .unwrap_or(readme_content.len() - start);
        readme_content.replacen(&readme_content[start..start + end], &progress_bar, 1)
    } else {
        // If no progress tracker exists, replace the placeholder
        readme_content.replace("<<PROGRESS-BAR-HERE>>", &progress_bar)
    };

    let mut readme_file = File::create("README.md")?;
    write!(readme_file, "{}", updated_readme)?;

    println!("README updated with progress: {}", progress_bar);

    Ok(())
}
