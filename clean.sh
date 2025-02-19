#!/bin/bash

function cargo_clean() {
  for entry in "$1"/*; do
    if [ -d "$entry" ]; then
      echo "$entry/"
      cd $entry
      if [ -f Cargo.toml ]; then
        cargo clean
      else
        cargo_clean "."
      fi
      cd ..
    fi
  done
}

# Start processing from the current directory
cargo_clean "."

