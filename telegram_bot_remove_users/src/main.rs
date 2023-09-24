extern crate telegram_bot;
extern crate clap;

use telegram_bot::{Api, ChatMemberUpdated, UpdateKind};
use tokio::runtime::Runtime;
use clap::{App, Arg};

/// Parse a date string in the format YYYY-MM-DD into a chrono::NaiveDate.
/// Returns a Result containing either the parsed date or an error.
fn parse_date(input_date: &str) -> Result<chrono::NaiveDate, chrono::ParseError> {
    chrono::NaiveDate::parse_from_str(input_date, "%Y-%m-%d")
}

/// Parse the command line arguments using the clap library.
/// Returns a matches object with the parsed arguments.
fn parse_args() -> clap::ArgMatches<'static> {
    App::new("Telegram Bot to Remove Users")
        .version("1.0")
        .author("ShirazLug")
        .about("Removes users who join after a specific date.")
        .arg(Arg::with_name("date")
            .short("d")
            .long("date")
            .value_name("DATE")
            .help("Specifies the cut-off date in YYYY-MM-DD format.")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("token")
            .short("t")
            .long("token")
            .value_name("TOKEN")
            .help("Your Telegram bot API token.")
            .takes_value(true)
            .required(true))
        .get_matches()
}

fn main() {
    // Parse command line arguments.
    let matches = parse_args();

    // Extract the provided date and token from the parsed arguments.
    let user_date = matches.value_of("date").unwrap();
    let token = matches.value_of("token").unwrap();

    // Parse the user's date string into a chrono::NaiveDate.
    let cut_off_date = parse_date(user_date)
        .expect("Failed to parse the date. Make sure it's in the format YYYY-MM-DD.");

    // Initialize a Tokio runtime for asynchronous operations.
    let mut rt = Runtime::new().unwrap();
    rt.block_on(async {
        // Initialize the Telegram bot API with the provided token.
        let api = Api::new(token);

        // Fetch new updates using the long poll method.
        let mut stream = api.stream();

        // Process each incoming update.
        while let Some(update) = stream.next().await {
            match update {
                Ok(update) => {
                    if let UpdateKind::ChatMemberUpdated(ChatMemberUpdated { date, .. }) = update.kind {
                        if date > cut_off_date.and_hms(0, 0, 0).timestamp() {
                            // If a member joined after the specified date, remove them.
                            let chat = update.chat;
                            let user = update.from;

                            api.send(chat.leave()).await.expect("Failed to leave the chat");
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error while processing update: {:?}", e);
                }
            }
        }
    });
}

// Unit tests module.
#[cfg(test)]
mod tests {
    use super::*;

    // Test the date parsing function.
    #[test]
    fn test_date_parsing() {
        // A valid date string should be parsed correctly.
        let valid_date_str = "2023-09-24";
        let parsed_date = parse_date(valid_date_str).unwrap();
        assert_eq!(parsed_date, chrono::NaiveDate::from_ymd(2023, 9, 24));

        // An invalid date string should return an error.
        let invalid_date_str = "2023-24-09";
        assert!(parse_date(invalid_date_str).is_err());
    }
}
