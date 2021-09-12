#!/bin/bash

if [ "$#" -ne 3 ]; then
    echo "Error: not enough input arguments"
    exit 1
fi

. conf.inc

first_name=$1
second_name=$2
username=$3

body="{\"firstName\": \"$first_name\", \"secondName\": \"$second_name\", \"username\": \"$username\"}"

curl -s -H "Content-Type: application/json" -d "$body" -X POST "$admin_base_url/api/admin/employee" --key $admin_key --cert $admin_cert -k; echo
