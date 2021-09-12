REPO_NAME = kamcpp
PROJECT_NAME = ess
IMAGE_TAG = latest

INIT_IMAGE_TAG = ${REPO_NAME}/${PROJECT_NAME}-init:${IMAGE_TAG}
BUILDER_IMAGE_TAG = ${REPO_NAME}/${PROJECT_NAME}-builder:${IMAGE_TAG}
WEB_SERVICE_IMAGE_TAG = ${REPO_NAME}/${PROJECT_NAME}-web-service:${IMAGE_TAG}

.PHONY: build-web-service
build-web-service:
	docker run -v ${PWD}:/work ${BUILDER_IMAGE_TAG} build --release --bin web-service

.PHONY: build-images
build-images: build-builder-image build-init-image build-web-service-image

.PHONY: build-builder-image
build-builder-image:
	docker build -f Dockerfile.builder -t ${BUILDER_IMAGE_TAG} .

.PHONY: build-init-image
build-init-image:
	docker build -f Dockerfile.init -t ${INIT_IMAGE_TAG} .

.PHONY: build-web-service-image
build-web-service-image: cert/ess.cryptizer.com.key cert/ess.cryptizer.com.crt
	# we need to this copy in order to keep the 'target' directory ignored when doing the docker build
	mkdir -p .tmp
	cp target/release/web-service .tmp
	docker build -f Dockerfile.web-service -t ${WEB_SERVICE_IMAGE_TAG} .
	rm -rf .tmp

.PHONY: push-images
push-images:
	docker push ${INIT_IMAGE_TAG}
	docker push ${WEB_SERVICE_IMAGE_TAG}
