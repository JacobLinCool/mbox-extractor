use mbox_extractor::cli::cli;
use mbox_extractor::extractor::EmailExtractor;
use std::fs::create_dir_all;
use std::io::{self, BufReader};
use std::path::{Path, PathBuf};

const EML_EXTENSION: &str = "eml";

fn main() -> io::Result<()> {
    let matches = cli().get_matches();

    let mbox: Option<&PathBuf> = matches.get_one("mbox");
    let output: Option<&PathBuf> = matches.get_one("output");
    let mbox = mbox.unwrap();
    let output = output.unwrap();

    if !mbox.exists() && mbox != Path::new("-") {
        eprintln!("mbox file \"{}\" not found", mbox.to_str().unwrap());
        std::process::exit(1);
    }

    if !output.exists() {
        create_dir_all(output)?;
    }

    let emails = if mbox == Path::new("-") {
        let stdin = io::stdin();
        let reader = stdin.lock();

        let mut extractor = EmailExtractor::new(BufReader::new(reader));
        extractor.extract_emails()
    } else {
        let file = std::fs::File::open(mbox)?;
        let reader = BufReader::new(file);

        let mut extractor = EmailExtractor::new(reader);
        extractor.extract_emails()
    };

    let padding = (emails.len() - 1).to_string().len().max(1);
    for (index, email) in emails.into_iter().enumerate() {
        let mut output = output.to_path_buf();
        output.push(format!("{:0padding$}", index, padding = padding));
        output.set_extension(EML_EXTENSION);
        email.save(output.to_str().unwrap())?;
    }

    Ok(())
}
