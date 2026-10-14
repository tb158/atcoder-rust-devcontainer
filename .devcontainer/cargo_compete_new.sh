#!/bin/bash

# Stop on any error
set -e

# Script requires exactly one argument
if [ "$#" -ne 1 ]; then
  echo "Error: Exactly one argument required."
  echo "Usage: cargo_compete_new.sh <contest>"
  exit 1
fi

# Change the working directory to the workspace folder
cd $WORKSPACE_FOLDER

# Set variables
contest=$1
contest_directory="./src/contest"

# --- Generate the contest directory using cargo-compete ---
# The new package is picked up automatically by the root Cargo.toml workspace
# glob (members = ["src/contest/*"]), so there is no settings.json to edit.
cargo compete n "$contest" --open || exit 1

# --- Create a .gitkeep file inside the testcases directory ---
# create_gitkeep_in_testcases.sh "$contest_directory/$contest"

# --- Remember the contest directory for abc_cd ---
contest_dir="$WORKSPACE_FOLDER/src/contest/$contest"
echo "$contest_dir" > "$WORKSPACE_FOLDER/.abc_last_contest"

echo ""
echo "Contest directory: $contest_dir"
echo "Run 'abc_cd' to change the current shell directory."
