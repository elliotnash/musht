#!/bin/env bash

function main() {
    local DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
    cargo uninstall musht

    # remove tab complete rcs
    local LINE="complete -C __musht_completions musht"

    # bash
    local FILE="$HOME/.bashrc"
    echo "$(grep -v "$LINE" $FILE)" > $FILE

    # zsh
    local FILE="$HOME/.zshrc"
    echo "$(grep -v "$LINE" $FILE)" > $FILE

}

main
