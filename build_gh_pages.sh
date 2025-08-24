#!/bin/bash

# Exit on error
set -e
VERSION=$(grep '^version' Cargo.toml | cut -d '"' -f2)

# Define variables
TARGET_BRANCH="gh-pages"
SOURCE_DIR="./target/dx/personal_website/release/web/public"
TMP_DIR=$(mktemp -d)

dx clean
dx bundle --platform web

if [ ! -d "$SOURCE_DIR" ]; then
    echo "Error: SOURCE_DIR '$SOURCE_DIR' does not exist."
    exit 1
fi

echo "Checking out $TARGET_BRANCH branch..."
git worktree prune
git fetch origin
git worktree add -B $TARGET_BRANCH $TMP_DIR origin/$TARGET_BRANCH

echo "Copying files from $SOURCE_DIR to $TARGET_BRANCH..."
rm -rf $TMP_DIR/*
cp -r $SOURCE_DIR/* $TMP_DIR/

cd $TMP_DIR

echo Copying index.html to 404.html...
cp index.html 404.html

echo "Committing and pushing to $TARGET_BRANCH..."
git add --all

if ! git diff --quiet; then
    git commit -m "web release for version $VERSION"
    git push origin $TARGET_BRANCH
else
    echo "No changes to publish."
fi

# Clean up
cd -
git worktree remove $TMP_DIR
echo gh-pages have been updated to $VERSION!
