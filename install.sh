#!/bin/bash

########################################
# Strawberry command-line installation #
########################################

git clone https://github.com/punctuations/strawberry "$HOME/.local/bin/strawberry"

if ! [[ ":$PATH:" == *":$HOME/.local/bin:"* ]]; then
  IFS="/" read -ra RC <<<"$SHELL"
  if test -f "$HOME/.${RC[-1]}rc"; then
    echo "export PATH=\"$PATH:$HOME/.local/bin\"" >>"$HOME/.${RC[-1]}rc"
  else
    echo "export PATH=\"$PATH:$HOME/.local/bin\"" >>"$HOME/.profile"
  fi
fi

if [[ -d "$HOME/.local/bin/strawberry" ]]; then
  echo "\u001b[38;5;48m 🍓 Strawberry successfully installed, please restart terminal for changes to apply. \u001b[0;0m"
else
  echo "\u001b[38;5;1m error - \u001b[0;0m something went wrong while installing."
