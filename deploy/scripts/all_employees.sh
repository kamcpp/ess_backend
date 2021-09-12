#!/bin/bash

. conf.inc

curl -s "$admin_base_url/api/admin/employee/all" --key $admin_key --cert $admin_cert -k ; echo
# curl -s "$admin_base_url/api/admin/employee/all" -k ; echo
