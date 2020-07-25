REPO_NAME = kamcpp
PROJECT_NAME = manning-simurgh-web-service
IMAGE_TAG = latest
IMAGE_NAME = ${REPO_NAME}/${PROJECT_NAME}:${IMAGE_TAG}

.PHONY: build-image
build-image:
	docker build -t ${IMAGE_NAME} .

.PHONY: push
push:
	docker push ${IMAGE_NAME}
