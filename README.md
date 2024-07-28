# Oxid Gateway

Oxid gateway is an API Gateway prototype written in Rust. This project is not meant to be used in production, and It was only created for me to understand the inner workings of API Gateway.

> Note that this is only the backend/proxy, the frontend repository can be found at: [Oxid Gateway Manager](https://github.com/rosa-gabriel/oxid-gateway-manager).

## College project first semester 2024 PAC IV

## Deployment

To run this project you will need to have Kubernetes to run a `keycloak` instance:

```sh
kubectl create namespace oxid-gateway 
kubectl apply -f ./k8s/keycloak-postgres.yml
kubectl apply -f ./k8s/keycloak.yml
```

Also, create a `postgres` instance as the gateway's database:

```sh
kubectl apply -f ./oxid-gateway-postgres.yml
```
Finally, you can build the docker image and run the project

```sh
docker build -t oxid-gateway .
kubectl apply -f ./k8s/oxid-gateway.yaml
```
