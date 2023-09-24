extern crate telegram_bot;
extern crate clap;

use telegram_bot::{Api, ChatMemberUpdated, CanLeaveChat, UpdateKind};
use tokio::runtime::Runtime;
use clap::{App, Arg};

fn main() {
    let matches = App::new("Telegram Bot to Remove Users")
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
        .get_matches();

    // Use value_of to get the date passed by the user and unwrap it since it's required.
    let user_date = matches.value_of("date").unwrap();
    let cut_off_date = chrono::NaiveDate::parse_from_str(user_date, "%Y-%m-%d")
        .expect("Failed to parse the date. Make sure it's in the format YYYY-MM-DD.");

    // Use a Tokio runtime to execute async code in a sync function.
    let mut rt = Runtime::new().unwrap();
    rt.block_on(async {
        let token = "YOUR_TELEGRAM_BOT_TOKEN";
        let api = Api::new(token);

        // Fetch new updates via long poll method.
        let mut stream = api.stream();

        while let Some(update) = stream.next().await {
            match update {
                Ok(update) => {
                    if let UpdateKind::ChatMemberUpdated(ChatMemberUpdated { date, .. }) = update.kind {
                        if date > cut_off_date.and_hms(0, 0, 0).timestamp() {
                            // If the member joined after the cut-off date, kick them out.
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
