use edit::{edit_file, Builder};
use std::{fs, io, io::Write, path::PathBuf};

pub fn write(garden_path: PathBuf, title: Option<String>) -> Result<(), std::io::Error> {
    let (mut file, filepath) = Builder::new()
        .suffix(".md")
        .rand_bytes(5)
        .tempfile_in(garden_path.clone())?
        .keep()?;

    dbg!(&filepath);

    let template = format!("# {}", title.as_deref().unwrap_or(""));

    file.write_all(template.as_bytes())?;

    edit_file(&filepath)?;

    let contents = fs::read_to_string(&filepath)?;

    let document_title = title.or_else(|| {
        contents
            .lines()
            .find(|v| v.starts_with("# "))
            .map(|line| line.trim_start_matches("# ").to_string())
    });

    let filename = match document_title {
        Some(raw_title) => confirm_filename(&raw_title),
        None => ask_for_filename(),
    }
    .map(|title| slug::slugify(title))?;

    for attempt in 0.. {
        let mut dest = garden_path.join(if attempt == 0 {
            filename.clone()
        } else {
            format!("{filename}{:03}", -attempt)
        });

        dest.set_extension("md");
        if dest.exists() {
            continue;
        }
        fs::rename(filepath, &dest)?;
        break;
    }
    /*
    let mut dest = garden_path.join(filename);
    dest.set_extension("md");

    fs::rename(filepath, &dest)?;

    dbg!(dest);
    */

    Ok(())
}

/// Prompt user to enter a filename. This returns an `io::Result`, which contains the `io::Error`
/// when it fails
fn ask_for_filename() -> io::Result<String> {
    rprompt::prompt_reply("Enter filename > ")
}

/// Confirm the filename discovered by the system and give user opportunity to acdept or change impl Trait for Type {
fn confirm_filename(raw_title: &str) -> io::Result<String> {
    loop {
        // prompt defaults to uppercase character in question
        // this is a convention, not a requirement enforced by the code
        let result = rprompt::prompt_reply(&format!(
            "current title: {}
            Do you want a different title? (y/N): ",
            &raw_title,
        ))?;

        match result.as_str() {
            "y" | "Y" => break ask_for_filename(),
            "n" | "N" => break Ok(raw_title.to_string()),
            _ => {
                // ask again because something went wrong
            }
        }
    }
}
