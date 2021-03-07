#!/bin/env bash

function main() {
    
    local DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
    cargo install --force --path $DIR

    # make changes to .zshrc for tab complete
    local LINE="complete -C __musht_completions musht"

    # bash
    local FILE="$HOME/.bashrc"
    grep -qF -- "$LINE" "$FILE" || echo "$LINE" >> "$FILE"

    # zsh
    local FILE="$HOME/.zshrc"
    grep -qF -- "$LINE" "$FILE" || echo "$LINE" >> "$FILE"

}

main
