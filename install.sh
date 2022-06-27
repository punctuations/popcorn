#!/bin/bash

########################################
# Blueberry command-line installation #
########################################

echo -e "\033[38;5;13m event - \033[0;0m Collecting data..."

# Get latest version of blueberry
RELEASE=$(curl --silent "https://api.github.com/repos/punctuations/blueberry/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')

# Get OS
if [[ "$OSTYPE" == "darwin"* ]]; then
  OSFILE="macos"
else
  OSFILE="ubuntu"
fi

# Get URL to latest release of blueberry
URL="https://github.com/punctuations/blueberry/releases/download/$RELEASE/blueberry-$OSFILE-latest.tar.gz"

echo -e "\033[38;5;13m event - \033[0;0m Data collected, asserting..."

# Setup tmp download
rm -Rf /tmp/blueberry-download
mkdir /tmp/blueberry-download

# Get latest tar
curl --silent $URL -L --output /tmp/blueberry-download/blueberry.tar.gz # Download blueberry archive
tar -zxf /tmp/blueberry-download/blueberry.tar.gz -C /tmp/blueberry-download # Extract blueberry archive

# move blueberry to .local/bin
cd /tmp/blueberry-download/blueberry-$OSFILE-latest && mv blueberry ~/.local/bin/

echo -e "\033[38;5;13m event - \033[0;0m assertion completed."

if ! [[ ":$PATH:" == *":$HOME/.local/bin:"* ]]; then
  IFS="/" read -ra RC <<<"$SHELL"
  if test -f "$HOME/.${RC[-1]}rc"; then
    echo "export PATH=\"$PATH:$HOME/.local/bin\"" >>"$HOME/.${RC[-1]}rc"
  else
    echo "export PATH=\"$PATH:$HOME/.local/bin\"" >>"$HOME/.profile"
  fi
fi

if [[ -f "$HOME/.local/bin/blueberry" ]]; then
  echo -e  "\033[38;5;48m 🫐  Blueberry successfully installed, please restart terminal for changes to apply. \033[0;0m"
else
  echo -e "\033[38;5;1m error - \033[0;0m something went wrong while installing."
fi
