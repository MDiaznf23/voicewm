# VoiceWM

A lightweight, universal voice control for Linux window managers (i3wm, Sway, KDE, etc.) built with **Rust** and **Vosk**. It runs 100% locally and is significantly fast for real-time commands.

## Features

- 100% offline voice recognition using Vosk
- Fast real-time command processing
- Universal support for Linux window managers (i3wm, Sway, KDE, etc.)
- Fully customizable voice commands via TOML config
- Built with Rust for performance and reliability

## Prerequisites

This program requires the **Vosk** C-library installed on your system.

### Installing Vosk (Arch Linux / CachyOS)

```bash
# Download Vosk library
wget https://github.com/alphacep/vosk-api/releases/download/v0.3.45/vosk-linux-x86_64-0.3.45.zip

# Extract and install
unzip vosk-linux-x86_64-0.3.45.zip
sudo cp vosk-linux-x86_64-0.3.45/libvosk.so /usr/local/lib/
sudo ldconfig
```

## Quick Start

### 1. Clone & Install

```bash
git clone https://github.com/MDiaznf23/voicewm.git
cd voicewm
chmod +x install.sh
./install.sh
```

The installation script will:

- Compile the binary
- Move it to `/usr/local/bin`
- Help you download a language model

### 2. Configuration

Your config file is located at `~/.config/voicewm/config.toml`.

**Note:** You must use absolute paths for the `model_path` (tilde `~` expansion is not supported).

```toml
[general]
model_path = "/home/youruser/voicewm/models/vosk-en"

[commands]
"terminal" = "alacritty &"
"close window" = "i3-msg kill"
"firefox" = "firefox &"
"workspace one" = "i3-msg workspace 1"
"workspace two" = "i3-msg workspace 2"
```

### 3. Run

Simply run the program:

```bash
voicewm
```

Or use the provided bridge script (see i3wm integration below).

## i3wm Integration (Optional)

You can use the included `bridge.sh` script to trigger Voice Mode and receive notifications via Dunst.

### Setup Bridge Script

1. Copy the `bridge.sh` script to your i3 config directory:

```bash
cp bridge.sh ~/.config/i3/bridge.sh
chmod +x ~/.config/i3/bridge.sh
```

2. Make sure the script has the correct library path. The `bridge.sh` should look like this:

```bash
#!/bin/bash
export LD_LIBRARY_PATH="/usr/local/lib:$LD_LIBRARY_PATH"

# Initial notification that system is ready
dunstify -r 999 -u low "🎙️ Voice Mode" "Listening..."

# Run voicewm with unbuffered output
stdbuf -oL voicewm --verbose 2>&1 | while read -r line; do
    echo "RAW: $line"

    if [[ "$line" == *"Detected speech:"* ]]; then
        text=$(echo "$line" | sed "s/.*Detected speech: '\(.*\)'/\1/" | xargs)

        if [[ -n "$text" ]]; then
            echo "-----------------------------------"
            echo "MATCHED SPEECH: $text"
            echo "-----------------------------------"

            # Update Dunst notification instantly
            dunstify -r 999 -u low "🎙️ Detected" "$text"
        fi
    fi
done
```

### i3 Configuration

Add this to your i3 config (`~/.config/i3/config`):

```bash
# Voice Control Mode
mode "voice" {
    bindsym Return mode "default"; exec --no-startup-id pkill -f bridge.sh; exec --no-startup-id pkill voicewm; exec --no-startup-id dunstctl close 999
    bindsym Escape mode "default"; exec --no-startup-id pkill -f bridge.sh; exec --no-startup-id pkill voicewm; exec --no-startup-id dunstctl close 999
}

# Trigger to enter voice mode (Mod+M)
bindsym $mod+m mode "voice"; exec --no-startup-id ~/.config/i3/bridge.sh
```

**Usage:**

- Press `$mod+m` (e.g., `Super+M`) to activate voice mode
- Speak your commands (notifications will appear via Dunst)
- Press `Enter` or `Escape` to exit voice mode and stop listening

Replace `$mod` with your modifier key (usually `Mod4` for Super/Windows key).

### Requirements for i3 Integration

- **dunst**: For desktop notifications
- **i3-msg**: Should be available with i3wm installation

Install dunst if not already installed:

```bash
# Arch Linux
sudo pacman -S dunst

# Ubuntu/Debian
sudo apt install dunst
```

## Usage Examples

Once running, you can speak commands like:

- "terminal" - Opens Alacritty
- "close window" - Closes the current window
- "workspace one" - Switches to workspace 1

Customize these commands in your `config.toml` file!

## Troubleshooting

### Library not found error

If you get an error about `libvosk.so` not being found:

```bash
sudo ldconfig
```

### Model path issues

Make sure your model path in `config.toml` uses absolute paths:

- ✅ `/home/username/voicewm/models/vosk-en`
- ❌ `~/voicewm/models/vosk-en`

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests.

## License

MIT License - see LICENSE file for details

## Acknowledgments

- Built with [Vosk](https://alphacephei.com/vosk/) for speech recognition
- Written in [Rust](https://www.rust-lang.org/)
