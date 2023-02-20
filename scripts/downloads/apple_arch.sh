#!/bin/bash

# Set the variables for the GitHub repo, release, and the operating system and chip you're targeting
repo="yellow-bird-consult/wedding_planner"
release="v1.0.0"
os="linux"
chip="amd64"

# Construct the URL of the release asset you want to download
url="https://github.com/$repo/releases/download/$release/$repo-$release-$os-$chip.tar.gz"

# Use curl to download the release asset to the current directory
curl -L -o "$repo-$release-$os-$chip.tar.gz" "$url"

# https://github.com/maxwellflitton/nan-services-build-tool/releases/download/v0.0.8/build_tool-aarch64-apple-darwin.tar.gz




# curl --location --request GET 'https://github.com/maxwellflitton/nan-services-build-tool/releases/download/v0.0.8/build_tool-aarch64-apple-darwin.tar.gz' --output file