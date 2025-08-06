#! /bin/sh
set -e

VERSION=$(grep '^version' Cargo.toml | cut -d '"' -f2)

dx clean

# mv Dioxus.toml Dioxus.toml.bak
# cp Dioxus.ghpages.toml Dioxus.toml
dx bundle --platform web
# mv Dioxus.toml.bak Dioxus.toml

git checkout gh-pages
rm -rf assets index.html wasm
cp -r target/dx/personal_website/release/web/public/* .
git add -A
git commit -m "web release for version $VERSION"
git push

git checkout master
[ -d wasm ] && rm -rf wasm

echo gh-pages have been updated to $VERSION!
