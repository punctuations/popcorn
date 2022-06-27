#!/bin/bash

########################################
# Blueberry command-line installation #
########################################

# Get latest version of blueberry
RELEASE=$(curl --silent "https://api.github.com/repos/punctuations/blueberry/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')

# Get OS
if [[ "$OSTYPE" == "darwin*" ]]; then
  OSFILE="macos"
else
  OSFILE="ubuntu"
fi

# Get URL to latest release of blueberry
URL="https://github.com/punctuations/blueberry/releases/download/$RELEASE/blueberry-$OSFILE-latest.tar.gz"

# Setup tmp download
rm -Rf /tmp/blueberry-download
mkdir /tmp/blueberry-download

# Get latest tar
curl --silent $URL -L --output /tmp/blueberry-download/blueberry.tar.gz # Download blueberry archive
tar -zxf /tmp/blueberry-download/blueberry.tar.gz -C /tmp/blueberry-download # Extract blueberry archive

# move blueberry to .local/bin
cd /tmp/blueberry-download/blueberry-$OSFILE-latest && mv blueberry ~/.local/bin/

if ! [[ ":$PATH:" == *":$HOME/.local/bin:"* ]]; then
  IFS="/" read -ra RC <<<"$SHELL"
  if test -f "$HOME/.${RC[-1]}rc"; then
    echo "export PATH=\"$PATH:$HOME/.local/bin\"" >>"$HOME/.${RC[-1]}rc"
  else
    echo "export PATH=\"$PATH:$HOME/.local/bin\"" >>"$HOME/.profile"
  fi
fi

if [[ -f "$HOME/.local/bin/blueberry" ]]; then
  echo "\u001b[38;5;48m 🫐 Blueberry successfully installed, please restart terminal for changes to apply. \u001b[0;0m"
else
  echo "\u001b[38;5;1m error - \u001b[0;0m something went wrong while installing."
fi
