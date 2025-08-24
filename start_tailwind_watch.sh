#!/bin/bash

# If not installed, run:
# npm install tailwindcss @tailwindcss/cli

trap "echo 'Stopped Tailwind watcher'; exit 0" SIGINT
npx @tailwindcss/cli -i ./assets/css/tailwind_input.css -o ./assets/css/tailwind_output.css --watch
