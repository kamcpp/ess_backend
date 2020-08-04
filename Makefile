REPO_NAME = kamcpp
PROJECT_NAME = manning-simurgh
IMAGE_TAG = latest

INIT_IMAGE_NAME = ${REPO_NAME}/${PROJECT_NAME}-init:${IMAGE_TAG}
WEB_SERVICE_IMAGE_NAME = ${REPO_NAME}/${PROJECT_NAME}-web-service:${IMAGE_TAG}
NOTIFIER_IMAGE_NAME = ${REPO_NAME}/${PROJECT_NAME}-notifier:${IMAGE_TAG}

.PHONY: build-images
build-images: build-init-image build-web-service-image build-notifier-image

.PHONY: build-init-image
build-init-image:
	docker build -f Dockerfile.init -t ${INIT_IMAGE_NAME} .

.PHONY: build-web-service-image
build-web-service-image:
	docker build -f Dockerfile.web-service -t ${WEB_SERVICE_IMAGE_NAME} .

.PHONY: build-notifier-image
build-notifier-image:
	docker build -f Dockerfile.notifier -t ${NOTIFIER_IMAGE_NAME} .

.PHONY: push-images
push-images:
	docker push ${INIT_IMAGE_NAME}
	docker push ${WEB_SERVICE_IMAGE_NAME}
	docker push ${NOTIFIER_IMAGE_NAME}
