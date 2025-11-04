#!/usr/bin/env zsh

brew=$(whence -a brew 2>/dev/null)
if [[ -z "$brew" ]]; then
  echo "Installing Homebrew..." >&2
  /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
  brew=$(whence -a brew 2>/dev/null)
fi

just=$(whence -a just 2>/dev/null)
if [[ -z "$just" ]]; then
  echo "Installing just..." >&2
  $brew install just
  just=$(whence -a just 2>/dev/null)
fi

$just "${@}"
