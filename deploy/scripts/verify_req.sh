#!/bin/bash

if [ "$#" -ne 2 ]; then
    echo "Error: wrong number of input arguments"
    exit 1
fi

. conf.inc

username=$1
totpCode=$2

body="{\"username\": \"$username\", \"totpCode\": \"$totpCode\"}"

curl -s -H "Content-Type: application/json" -d "$body" -X POST "$base_url/api/pam/verify" -k; echo
