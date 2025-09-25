# DevOps Demonstration

## Overview

The goal of this project is to demonstrate how to setup a Jenkins Pipeline to deliver a web service to a staging/production environment.
The pipeline will automatically build, run tests, and perform security scan before deployment. After passed all checks, it will push the image (build from source code) to image registry and apply new deployment settings to the destination environment (k8s cluster).

### Folder structure
```
root
├── jenkins           # The Dockerfile for building and start a Jenkins instance
│   └── cicd          # Main pipeline script
├── k8s               # All necessary file for deployment on k8s cluster
│   └── base          # PostgreSQL and configs setup for k8s cluster, only run 1 time
├── migrations        # Sqlx migrations - the changes would be applied on PostgreSQL
├── scripts           # Custom script to run migrations
├── src               # Source code & unit tests
│   ├── config
│   ├── model
│   ├── repository
│   ├── router
│   ├── service
│   └── util
└── tests             # Integration tests
```

### Tech Stacks
Below are the list of tech stacks used in this projects:
- Rust programming language - with Axum framework
- Tools:
  - Unit & integration tests: cargo test
  - Static code check: cargo clippy
  - License check: cargo license
  - Security scan: cargo audit
  - Vulnerability scan for image: Trivy
- Database: PostgreSQL
- Containerization: Docker
- Deployment:
  - Automation server: Jenkins
  - Repository and image registry: Github, Github Container Registry
  - Deployment environment: Kubernetes

## Requirements
- Linux
- Docker
- k3s (Optional for deployment)

## Installation

### Setup Jenkins

The Jenkins instance used for this demonstration is built from docker image and it contains a custom build script in order to build and test Rust project
```
# All files required to build and start a Jenkins instance is in /jenkins folder
cd jenkins

# Build a custom image with `jenkins-custom` tag, see the Dockerfile in the folder for more information
docker buildx build --platform linux/amd64 --tag jenkins-custom --load .

# Start the Jenkins instance, see the docker-compose.yml file
docker compose up
```

An important requirement for this project is that the Jenkins instance must be able to use docker to build image
```
# Access the running Jenkins instance
docker exec -it --user root jenkins-custom /bin/sh

# Check if the docker.sock file is granted to `jenkins` group or not
ls -al /var/run/docker.sock

# If the docker.sock file is not granted, change the group to `jenkins`
chgrp jenkins /var/run/docker.sock
```

### Setup deployment environment
Any k8s cluster can be used for this project, we can use any cloud vendor such as EKS, GKE or your own k8s cluster using k3s, Kind, etc.. as long as the cluster can be connected from the Jenkins instance.

For the demonstration, we will use k3d to create a Kubernetes cluster
```
# Create a new cluster
k3d cluster create staging

# Setup configs and PostgreSQL on k8s
kubectl apply -f k8s/base

# Setup connection to Github
# It is important that the github token must have permissions to checkout the code and publish the image
kubectl create secret docker-registry ghcr-secret \
  --docker-server=ghcr.io \
  --docker-username=YOUR_USERNAME \
  --docker-password=YOUR_TOKEN \
  --docker-email=YOUR_EMAIL
```

### Setup Jenkins Pipeline
To setup the Jenkins Pipeline, we need to add Github and K8s credentials
- For Github: Add a new `Username and password` credential
- For k8s: Add a new `Secret file` credential, and upload the kubeconfig.yml file. Make sure that the Jenkins instance can access the cluster with the credential file.

Finally, we can create a new Jenkins Pipeline with the script located in `/jenkins/cicd/Jenkinsfile`, try to run build and see the result
