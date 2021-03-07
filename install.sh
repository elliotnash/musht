#!/bin/env bash

function main() {
    
    local DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
    cargo install --force --path $DIR

    # make changes to .profile for tab complete

}

main
