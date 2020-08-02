REPO_NAME = kamcpp
PROJECT_NAME = manning-simurgh-web-service
IMAGE_TAG = latest
IMAGE_NAME = ${REPO_NAME}/${PROJECT_NAME}:${IMAGE_TAG}
INIT_IMAGE_NAME = ${REPO_NAME}/${PROJECT_NAME}-init:${IMAGE_TAG}

.PHONY: build-image
build-image:
	docker build -t ${IMAGE_NAME} .

.PHONY: build-init-image
build-init-image:
	docker build -f Dockerfile.init -t ${INIT_IMAGE_NAME} .

.PHONY: push
push:
	docker push ${IMAGE_NAME}
	docker push ${INIT_IMAGE_NAME}
