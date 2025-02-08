# Mini-badger

Shows dock notifications for selected apps in the MacOS menu bar. Bring your own icons.

<img width="1168" alt="Screenshot 2025-02-08 at 05 19 55" src="https://github.com/user-attachments/assets/f046e465-7c5d-408c-bb35-5b735f596ee3" />

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

## Roadmap

- [ ] Prevent mini-badger icon from appearing in Dock
