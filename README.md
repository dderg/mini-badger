# Mini-badger

Lightweight app to display notification icon from the dock in the macos top bar

## Installation

```bash
brew tap dderg/tap
brew install mini-badger
```

To run on startup
```bash
brew services start mini-badger
```

## Configuration

The app expects to see the config file at `~/.config/mini-badger/mini-badger.toml`, example:

```toml
[apps.Slack]
interval_secs = 2
icon_path = "~/.config/mini-badger/icons/Slack.png"

[apps.Mail]
interval_secs = 10
```

