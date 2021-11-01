# Encryptizer Simurgh Software (ESS) Backend Project

## Build the init docker image
```shell
$ make build-init-image
```

## Build the web service project
```shell
$ make build-builder-image
$ make build-web-service
```

## Build the web service's docker image
```shell
$ cd cert
$ make
$ cd ..
$ make build-web-service-image
```

## Deploy the backend
```
$ cd deploy/compose/ess
$ docker-compose up -d
```

## Check the deployment
You can run use the scripts located at `deploy/scripts` to test your deployment.

## Author
Kam Amini (kam.cpp@gmail.com) 2020-2021
