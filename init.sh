#!/bin/bash

cd /work
echo "DATABASE_URL=postgres://$POSTGRES_USER:$POSTGRES_PASSWORD@pgsql/$POSTGRES_DB" > .env
cat .env
diesel migration run
