#! /bin/sh
set -e

VERSION=$(grep '^version' Cargo.toml | cut -d '"' -f2)

dx clean
dx bundle --platform web

git checkout gh-pages
git pull
rm -rf assets index.html wasm
cp -r target/dx/personal_website/release/web/public/* .
git add -A
git commit -m "web release for version $VERSION"
git push

git checkout master
[ -d wasm ] && rm -rf wasm

echo gh-pages have been updated to $VERSION!
