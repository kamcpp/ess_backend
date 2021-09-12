#!/bin/bash

sleep 5

cd /work
echo "DATABASE_URL=postgres://$POSTGRES_USER:$POSTGRES_PASSWORD@$POSTGRES_ADDR/$POSTGRES_DB" > .env
cat .env
diesel migration run
