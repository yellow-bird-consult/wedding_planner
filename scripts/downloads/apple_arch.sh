#!/usr/bin/env bash
# 
# This script downloads an asset from latest or specific Github release of a
# private repo. Feel free to extract more of the variables into command line
# parameters.
#
# PREREQUISITES
#
# curl, wget, jq
#
# USAGE
#
# Set all the variables inside the script, make sure you chmod +x it, then
# to download specific version to my_app.tar.gz:
#
#     gh-dl-release 2.1.1 my_app.tar.gz
#
# to download latest version:
#
#     gh-dl-release latest latest.tar.gz
#
# If your version/tag doesn't match, the script will exit with error.

TOKEN="$3"
REPO="$4"
FILE="$2"      # the name of your release asset file, e.g. build.tar.gz
VERSION="$1"                       # tag name or the word "latest"
GITHUB="https://api.github.com"

alias errcho='>&2 echo'

function gh_curl() {
  curl -H "Authorization: token $TOKEN" \
       -H "Accept: application/vnd.github.v3.raw" \
       $@
}

parser=".assets | map(select(.name == \"$FILE\"))[0].id"
if [ "$VERSION" = "latest" ]; then
  # Github should return the latest release first.
  asset_id=`gh_curl -s $GITHUB/repos/$REPO/releases/latest | jq "$parser"`
else
  asset_id=`gh_curl -s $GITHUB/repos/$REPO/releases/tags/$VERSION | jq "$parser"`
fi;

if [ -z "$asset_id" ]; then
  errcho "ERROR: version not found $VERSION"
  exit 1
fi;
if [ "$asset_id" = "null" ]; then
  errcho "ERROR: file $FILE not found in version $VERSION"
  exit 2
fi;

wget -q --auth-no-challenge --header='Accept:application/octet-stream' \
  https://$TOKEN:@api.github.com/repos/$REPO/releases/assets/$asset_id \
  -O $2
