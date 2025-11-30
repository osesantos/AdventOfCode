#!/bin/bash

# This script creates a new directory for a given day and populates it with template files.
# Usage: ./create_day.sh <day_number>
# Example: ./create_day.sh 5
# This will create a directory named "day05" with template files inside.
# Make sure to run this script from the root directory of your project.

DAY_NUM=$1
DAY_DIR=$(printf "day%02d" "$DAY_NUM")
TEMPLATE_DIR="template_files"

if [ -z "$DAY_NUM" ]; then
    echo "Usage: $0 <day_number>"
    exit 1
fi
if [ -d "$DAY_DIR" ]; then
    echo "Directory $DAY_DIR already exists. Aborting."
    exit 1
fi

# Create the new day directory and copy template files
mkdir "src/$DAY_DIR"
cp "$TEMPLATE_DIR/"* "src/$DAY_DIR/"

# Add the `pub mod dayXX;` line to src/main.rs
echo "pub mod $DAY_DIR;" >> src/main.rs

echo "Created directory $DAY_DIR with template files."
