# C2 Server

This is a C2 server allowing to communicate with a discord server connected to a client 
bot running on a distant machine.
This server is basically a CLI for the discord server connected to the 
[client bot](https://github.com/loouis-t/c2-discord).

## Why discord?

Discord is a communication platform that has fewer chances of being blocked by a
firewall than a custom C2 server.

## Installation

### Configuration

- Go to [Discord Developer Portal](https://discord.com/developers/applications),
  create a new application, and a bot. Copy the token and paste it in the
  `.env` file along with the channel id of the channel you want to use as the command channel.
- Go to the `OAuth2` tab, select the `bot` scope, and copy the generated URL
  to invite the bot to your server.
- Make sure your bot has the `Send Messages` and `Read Message History` permissions.
- Make sure all "Privileged Gateway Intents" are enabled in the bot settings
  (in developer portal).

### Build

```bash
cargo build --release
```

### Execution

```bash
cargo run --release
```

## Usage

The bot will wait for you to provide a command in the terminal, and send it to 
the server. The client bot will then execute the command on the distant machine, and
send the result back to the server.

The server will then display the result in the terminal.


### Commands

- `!download <path>`: Download a file from the given path.
- Anything else: will be interpreted and sent as a bash command.

---

CY Tech - Louis Travaux, Yan Arresseguet, Léo Portet, Baptiste Hennuy.
