# TraceGuard Deployment Guide

This guide outlines the deployment options and processes for TraceGuard.

## Deployment Options

TraceGuard supports three main deployment scenarios:

1. Cloud Deployment
2. On-Premises Deployment
3. Off-Grid Deployment

### 1. Cloud Deployment

For cloud deployments, we use Kubernetes for orchestration and scaling.

#### Prerequisites:
- Kubernetes cluster (e.g., EKS, AKS, GKE)
- `kubectl` configured to access your cluster
- Helm (optional, for easier deployment)

#### Steps:
1. Build and push the Docker image:
   ```bash
   docker build -t ghcr.io/mchorfa/traceguard:latest -f deployment/docker/Dockerfile .
   docker push ghcr.io/mchorfa/traceguard:latest
   ```

2. Deploy using kubectl:
   ```bash
   kubectl apply -f deployment/kubernetes/deployment.yaml
   kubectl apply -f deployment/kubernetes/service.yaml
   ```

   Or using Helm (if a Helm chart is available):
   ```bash
   helm install traceguard ./helm/traceguard
   ```

### 2. On-Premises Deployment

For on-premises deployments, we use vCluster for tenant isolation.

#### Prerequisites:
- Kubernetes cluster on-premises
- vCluster installed
- `kubectl` and `vcluster` CLI tools

#### Steps:
1. Create a vCluster for TraceGuard:
   ```bash
   vcluster create traceguard-vcluster
   ```

2. Deploy TraceGuard in the vCluster:
   ```bash
   vcluster connect traceguard-vcluster
   kubectl apply -f deployment/kubernetes/deployment.yaml
   kubectl apply -f deployment/kubernetes/service.yaml
   ```

### 3. Off-Grid Deployment

For off-grid deployments, we use Porter to package TraceGuard as a bundle.

#### Prerequisites:
- Porter CLI installed
- Docker (for building the bundle)

#### Steps:
1. Build the Porter bundle:
   ```bash
   porter build
   ```

2. Install the bundle:
   ```bash
   porter install --credential-set kubernetes
   ```

This will deploy TraceGuard along with HashiCorp Vault for secret management.

## Post-deployment steps

1. Initialize Vault:
   ```bash
   kubectl exec -it vault-0 -- vault operator init
   ```

2. Unseal Vault:
   ```bash
   kubectl exec -it vault-0 -- vault operator unseal
   ```

3. Configure Vault for TraceGuard:
   ```bash
   kubectl exec -it vault-0 -- /bin/sh
   vault login
   vault secrets enable -path=secret kv-v2
   vault policy write traceguard-policy -<<EOF
   path "secret/data/traceguard/*" {
     capabilities = ["create", "read", "update", "delete", "list"]
   }
   EOF
   vault token create -policy=traceguard-policy
   ```

   Save the generated token for use in TraceGuard configuration.

## Updating TraceGuard

To update TraceGuard, modify the `porter.yaml` file and run:

```bash
porter upgrade --credential-set kubernetes
```

## Uninstalling TraceGuard

To uninstall TraceGuard and its components:
```bash
porter uninstall --credential-set kubernetes
```
