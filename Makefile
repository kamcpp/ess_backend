REPO_NAME = kamcpp
PROJECT_NAME = manning-simurgh
IMAGE_TAG = latest

WEB_SERVICE_IMAGE_NAME = ${REPO_NAME}/${PROJECT_NAME}-web-service:${IMAGE_TAG}
INIT_IMAGE_NAME = ${REPO_NAME}/${PROJECT_NAME}-init:${IMAGE_TAG}

.PHONY: build-web-service-image
build-web-service-image:
	cd web-service && docker build -t ${WEB_SERVICE_IMAGE_NAME} .

.PHONY: build-init-image
build-init-image:
	cd init && docker build -t ${INIT_IMAGE_NAME} .

.PHONY: push
push:
	docker push ${WEB_SERVICE_IMAGE_NAME}
	docker push ${INIT_IMAGE_NAME}
