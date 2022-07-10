#!/bin/bash
version=$(git tag --points-at HEAD)
if [[ "${version}" == "" ]]; then
  echo "cannot get version tag from HEAD commit"
  exit 1
fi
bin=$(basename "$(git rev-parse --show-toplevel)")
target="$*"

if [[ ! -d "release" ]]; then
 mkdir "release"
fi

for target in "$@"; do
  archive="${bin}_${version}_${target}"
  echo "## make archive ${archive}"
  if [[ "$target" == *"windows"* ]]; then
    zip "release/${archive}.zip" "target/${target}/release/${bin}.exe"
  else
    tar -zcvf "release/${archive}.tar.gz" "target/${target}/release/${bin}"
  fi
done
