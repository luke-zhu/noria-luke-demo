# TODO: build deps then add source files as described in the thread
# TODO: release build instead for performance
build-noria-server:
	gcloud builds submit --timeout 1h --async --tag gcr.io/gothic-list-253223/noria-server:dev ./deploy/docker/noria-server

build-noria-mysql:
	gcloud builds submit --timeout 1h --async --tag gcr.io/gothic-list-253223/noria-mysql:dev ./deploy/docker/noria-mysql

build-demo:
	gcloud builds submit --ignore-file .dockerignore --timeout 1h --async --tag gcr.io/gothic-list-253223/noria-luke-demo:dev ./

start-noria:
	helm install --name noria ./deploy/k8s/noria

stop-noria:
	helm delete noria

cleanup-noria:
	helm delete noria --purge
	kubectl get pvc -o name | xargs kubectl delete

start-demo:
	kubectl apply -f ./deploy/k8s/demo.yaml

stop-demo:
	kubectl delete -f ./deploy/k8s/demo.yaml