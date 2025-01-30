#!/usr/bin/env bash

[[ -z $1 ]] && echo "Usage: $0 file" && exit 1

surreal import --namespace finanalize --database db -u root -p root --endpoint http://localhost:8000 "$1"
