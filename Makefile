

build-docker:
	cargo build --release
	docker build --tag docker.io/panov17/tcp-prometheus:latest .

push:
	docker push docker.io/panov17/tcp-prometheus:latest

docker-run:
	docker run --rm -p 3000:3000 kubeapi-prometheus:latest

port-forward-prom:
	kubectl port-forward service/prometheus-service 9090:9090

patch-kubeapi:
	kubectl patch deployment kube-apiserver --patch-file kube-apiserver-patch.yml

patch-networkpolicy:
	kubectl patch networkpolicy allow-kube-apiserver --patch-file networkpolicy-patch.yml

run1:
	kubectlrun test1 --image 35.195.44.59:5000/kubeapi-prometheus:latest --port 3000

#example
prom-conf-deploy:
	kubectl create configmap prometheus-config --from-file prometheus.yml

fix1:
	sudo echo '{ "insecure-registries":["35.195.44.59:5000"] }' > /etc/docker/daemon.json