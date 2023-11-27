<div align="center">
	<h1>Swin Squad Bot</h1>
	<img src="https://media.discordapp.net/attachments/1079608708067233902/1135430160745775224/SQ1.png">
</div>


Swin Squad Bot is a Discord bot designed specifically for educational communities on Discord. It's developed using Rust and leverages the Serenity framework for interaction with the Discord API. The bot's main feature is to allow users to select roles corresponding to elective units, granting them access to specific text channels.

## Features

- **Role Selection:** Users can choose roles related to their elective units, which in turn grants them access to exclusive text channels.
- **Rust-Powered:** Developed using Rust, ensuring efficient and safe code execution.
- **Serenity Framework:** Utilizes Serenity, a powerful library for Discord bots.

## Prerequisites

Before you begin, ensure you have met the following requirements:

- Rust Programming Language (latest stable version)
- Cargo (Rust's package manager)

## Installation and Setup

1. **Clone the Repository:**

```bash
git clone https://yourrepositorylink.com/swin_squad_bot.git
cd swin_squad_bot
```

2. **Set Up Environment Variables:**

Create a `.env` file in the root directory and add your Discord bot token:

```bash
DISCORD_TOKEN=your_bot_token_here
```

3. **Build in Releaes mode:**

Run the following command to build in release mode:

```bash
cargo build --release
```

4. **Run the bot:**

To run the release version that we build above:

```bash
./target/release/swin_squad_bot
```

Or you can run it in development version with this command:

```bash
cargo run
```

## Configuration

To configure the bot for your specific server and elective units, follow these steps:

- Edit the configuration files (if any) to match your server's structure and elective units.
- Ensure the bot has appropriate permissions on your Discord server to manage roles and channels.

## Contributing

Contributions to the Swin Squad Bot are welcome! If you have a suggestion or an improvement, feel free to fork the repository and create a pull request.

## License

This project is licensed under the BSD 3-Clause License - see the [LICENSE](LICENSE) file for details.
