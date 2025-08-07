#! /bin/sh
set -e

VERSION=$(grep '^version' Cargo.toml | cut -d '"' -f2)

dx clean
dx bundle --platform web

git checkout gh-pages
rm -rf assets index.html wasm
cp -r target/dx/personal_website/release/web/public/* .
cp index.html 404.html
git add -A
git commit -m "web release for version $VERSION"
git push

git checkout master
[ -d wasm ] && rm -rf wasm
[ -d 404.html ] && rm -rf 404.html

echo gh-pages have been updated to $VERSION!
