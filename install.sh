#!/bin/bash

########################################
# popcorn command-line installation    #
########################################

echo -e "[38;5;13m event - [0;0m 1/3 Data collection."

# Get latest version of popcorn
RELEASE=$(curl --silent "https://api.github.com/repos/punctuations/popcorn/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')

# Get OS
if [[ "$OSTYPE" == "darwin"* ]]; then
  OSFILE="macos"
else
  OSFILE="ubuntu"
fi

# Get URL to latest release of popcorn
URL="https://github.com/punctuations/popcorn/releases/download/$RELEASE/popcorn-$OSFILE-latest.tar.gz"

echo -e "[38;5;13m event - [0;0m 2/3 Assertion."

# Setup tmp download
rm -Rf /tmp/popcorn-download
mkdir /tmp/popcorn-download

# Get latest tar
curl --silent $URL -L --output /tmp/popcorn-download/popcorn.tar.gz # Download popcorn archive
tar -zxf /tmp/popcorn-download/popcorn.tar.gz -C /tmp/popcorn-download # Extract popcorn archive

# move popcorn to .local/bin
cd /tmp/popcorn-download/popcorn-$OSFILE-latest && mv popcorn ~/.local/bin/

echo -e "[38;5;13m event - [0;0m 3/3 Cleanup."

if ! [[ ":$PATH:" == *":$HOME/.local/bin:"* ]]; then
  IFS="/" read -ra RC <<<"$SHELL"
  if test -f "$HOME/.${RC[-1]}rc"; then
    echo "export PATH=\"$PATH:$HOME/.local/bin\"" >>"$HOME/.${RC[-1]}rc"
  else
    echo "export PATH=\"$PATH:$HOME/.local/bin\"" >>"$HOME/.profile"
  fi
fi

if [[ -f "$HOME/.local/bin/popcorn" ]]; then
  echo -e  "[38;5;10m 🍿  popcorn successfully installed, please restart terminal for changes to apply. [0;0m"
else
  echo -e "[38;5;1m error - [0;0m something went wrong while installing."
fi
