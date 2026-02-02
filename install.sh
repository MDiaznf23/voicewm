#!/bin/bash
# install.sh - Installer for voicewm

echo "--- Tahap 1: Instalasi Library Vosk ---"
wget https://github.com/alphacep/vosk-api/releases/download/v0.3.45/vosk-linux-x86_64-0.3.45.zip
unzip -o vosk-linux-x86_64-0.3.45.zip
sudo cp vosk-linux-x86_64-0.3.45/libvosk.so /usr/local/lib/
sudo ldconfig
rm -rf vosk-linux-x86_64-0.3.45* 

echo "--- Tahap 2: Kompilasi voicewm ---"
LIBRARY_PATH=/usr/local/lib cargo build --release
sudo cp target/release/voicewm /usr/local/bin/

echo "--- Tahap 3: Setup Konfigurasi ---"
# Membuat direktori config di home user jika belum ada
CONFIG_DIR="$HOME/.config/voicewm"
mkdir -p "$CONFIG_DIR"

# Memindahkan file config.toml dari root project ke folder .config
if [ -f "config.toml" ]; then
    cp -n config.toml "$CONFIG_DIR/config.toml"
    echo "Konfigurasi disalin ke $CONFIG_DIR/config.toml"
else
    echo "Peringatan: config.toml tidak ditemukan di root project."
fi

echo "--- Tahap 4: Pilihan Model Vosk (English) ---"
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
        echo "Pilihan tidak valid. Melewati download model."
        exit 1
        ;;
esac

unzip model.zip -d models/
# Menghapus akhiran versi agar konsisten di config.toml
mv models/vosk-model-* models/vosk-en
rm model.zip

echo "--- Instalasi Selesai ---"
echo "Silakan edit $CONFIG_DIR/config.toml dan arahkan model_path ke $PWD/models/vosk-en"
