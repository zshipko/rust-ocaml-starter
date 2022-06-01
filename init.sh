name=$1
public_name=${2-$name}

replace() {
  is_gnu=`sed --version`
  if [ $? -ne 0 ]; then
    sed -i "" $@
  else
    sed -i $@
  fi
}

replace 's/rust-ocaml-starter/'"$public_name"'/g' dune-project
replace 's/rust_ocaml_starter/'"$name"'/g' lib/dune
replace 's/rust-ocaml-starter/'"$public_name"'/g' lib/dune
replace 's/rust-ocaml-starter/'"$public_name"'/g' Cargo.toml
printf "# $public_name\n" > README.md
mv rust-ocaml-starter.opam $public_name.opam
mv lib/rust_ocaml_starter.ml lib/$name.ml
rm init.sh
