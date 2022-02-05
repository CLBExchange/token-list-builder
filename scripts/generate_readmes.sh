#!/usr/bin/env bash

cd $(dirname $0)/..
cargo readme -r pubkey >pubkey/README.md
cargo readme -r builder >builder/README.md
cargo readme -r tokenlist >tokenlist/README.md
