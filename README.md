# ShirazLugTelegramCleaner


## Telegram Bot to Remove Users Joining After a Specified Date

Follow the steps below to set up and execute a Rust-based Telegram bot that removes users joining after a given date.

## 1. Setting Up:

### a. Create a New Rust Project:

```bash
cargo new telegram_bot_remove_users
cd telegram_bot_remove_users
```

### b. Add Dependencies:

In your `Cargo.toml` file, add the following dependencies:

```toml
[dependencies]
telegram-bot = "0.7"
tokio = { version = "1", features = ["full"] }
chrono = "0.4"
clap = "2.33"
```

### c. Create a New Bot on Telegram:

1. Open the Telegram app.
2. Search for [BotFather](https://core.telegram.org/bots#botfather) and start a chat with it.
3. Use the `/newbot` command to create a new bot.
4. Follow the instructions given by BotFather to set a name and username for your bot.
5. Note down the token provided by BotFather, you'll need it later.

## 2. Code the Bot:

Replace the code in `src/main.rs` with the code provided in the previous answers. Make sure to replace `"YOUR_TELEGRAM_BOT_TOKEN"` with the token you received from the BotFather.

## 3. Compile and Run:

### a. Compile the Bot:

```bash
cargo build --release
```

### b. Run the Bot:

```bash
./target/release/telegram_bot_remove_users -d 2023-09-24
```

> Note: Replace `2023-09-24` with any date of your choice.

## 4. Bot Permissions:

### a. Add Your Bot to the Telegram Channel or Group:

1. Navigate to the desired channel or group settings.
2. Add your bot as an administrator.
3. Ensure you grant the bot the "Can manage chat" permission, which will allow it to remove users.

## 5. Start Monitoring:

With everything set up and running, the bot will begin monitoring the channel or group. It will automatically remove users joining after the specified cut-off date.

> **Important**: This setup is basic, and the bot might behave unexpectedly if users have permissions to add other users or if other bots and configurations exist in the group. Ensure thorough testing and include adequate error handling for a production-level application.

---

Feel free to use, share, or modify this guide as needed!
