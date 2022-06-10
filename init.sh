#!/bin/sh -eu

usage() {
  cat >&2 <<EOF
usage: init NAME PUBLIC_NAME
Note that NAME will be the library name and thus must be non-empty and composed only of the following
characters: 'A'..'Z', 'a'..'z', '_' or '0'..'9'.

Setup a project
EOF
  exit 1
}

if [ $# -lt 1 ] || [ $# -gt 2 ]; then
  usage
fi

case $(uname | tr '[:upper:]' '[:lower:]') in
linux*)
  OS_NAME=linux
  ;;
darwin*)
  OS_NAME=osx
  ;;
*)
  OS_NAME=notset
  ;;
esac

replace() {
  if [[ "$OS_NAME" = "osx" ]]; then
    sed -i '' $@
  else
    sed -i $@
  fi
}

## Replace hyphens in $name so that we have a valid library name
name="$(echo "$1" | sed 's/-/_/g')"
public_name="${2-$1}"

replace 's/rust-ocaml-starter/'"$public_name"'/g' dune-project
replace 's/rust_ocaml_starter/'"$name"'/g' lib/dune
replace 's/rust-ocaml-starter/'"$public_name"'/g' lib/dune
replace 's/rust-ocaml-starter/'"$public_name"'/g' Cargo.toml
printf "# $public_name\n" >README.md
rm rust-ocaml-starter.opam ## regenerate opam file from dune-project
mv lib/rust_ocaml_starter.ml lib/$name.ml || :

while true; do
  read -p "Cleaning up - remove init.sh (y/n)? " -r yn
  case "${yn}" in
  [Yy]*)
    rm init.sh
    break
    ;;
  [Nn]*) exit ;;
  *) echo "" ;;
  esac
done
