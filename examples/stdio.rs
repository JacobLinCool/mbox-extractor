use mbox_extractor::extractor::EmailExtractor;
use std::io::{self, BufReader};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut extractor = EmailExtractor::new(BufReader::new(reader));

    for (index, email) in extractor.extract_emails().into_iter().enumerate() {
        println!(
            "Email #{}: {}",
            index,
            email.subject.clone().unwrap_or_default()
        );
        // let output = format!("{}.eml", index);
        // email.save(&output).unwrap();
    }
}
