#!/bin/bash
# install.sh - Installer for voicewm

echo "--- Phase 1: Download Library Vosk ---"
wget https://github.com/alphacep/vosk-api/releases/download/v0.3.45/vosk-linux-x86_64-0.3.45.zip
unzip -o vosk-linux-x86_64-0.3.45.zip
sudo cp vosk-linux-x86_64-0.3.45/libvosk.so /usr/local/lib/
sudo ldconfig
rm -rf vosk-linux-x86_64-0.3.45* 

echo "--- Phase 2: Compile voicewm ---"
LIBRARY_PATH=/usr/local/lib cargo build --release
sudo cp target/release/voicewm /usr/local/bin/

echo "--- Phase 3: Setup Configuration ---"
# Make directory config
CONFIG_DIR="$HOME/.config/voicewm"
mkdir -p "$CONFIG_DIR"

# Move file config.toml from root project to folder .config
if [ -f "config.toml" ]; then
    cp -n config.toml "$CONFIG_DIR/config.toml"
    echo "Configuration move to $CONFIG_DIR/config.toml"
else
    echo "Warning: config.toml not found in root project."
fi

echo "--- Phase 4: Choose Model Vosk (English) ---"
echo "Choose models that you want to try:"
echo "1) Small (40MB)"
echo "2) Medium (128MB)"
echo "3) Big (1.8GB)"
read -p "Choose Numbers (1-3): " choice

mkdir -p models

case $choice in
    1)
        wget https://alphacephei.com/vosk/models/vosk-model-small-en-us-0.15.zip -O model.zip
        ;;
    2)
        wget https://alphacephei.com/vosk/models/vosk-model-en-us-0.22-lgraph.zip -O model.zip
        ;;
    3)
        wget https://alphacephei.com/vosk/models/vosk-model-en-us-0.22.zip -O model.zip
        ;;
    *)
        echo "Not valid. Skipping download model."
        exit 1
        ;;
esac

unzip model.zip -d models/
mv models/vosk-model-* models/vosk-en
rm model.zip

echo "--- Installation done ---"
echo "Please edit $CONFIG_DIR/config.toml and mmake model_path to $PWD/models/vosk-en"
