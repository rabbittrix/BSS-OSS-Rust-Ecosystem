# Kubernetes Deployment Guide

This directory contains Kubernetes manifests for deploying the BSS/OSS Rust ecosystem.

## Prerequisites

- Kubernetes cluster (v1.20+)
- `kubectl` configured to access your cluster
- Docker image built and available in a registry (or use local image)

## Quick Start

### 1. Create Namespace

```bash
kubectl apply -f namespace.yaml
```

### 2. Create Secrets

**Important:** Update the secrets in `secret.yaml` with production values before applying:

```bash
kubectl apply -f secret.yaml
```

### 3. Create ConfigMap

```bash
kubectl apply -f configmap.yaml
```

### 4. Deploy PostgreSQL

```bash
kubectl apply -f postgres-deployment.yaml
```

Wait for PostgreSQL to be ready:

```bash
kubectl wait --for=condition=ready pod -l app=postgres -n bss-oss --timeout=300s
```

### 5. Deploy Redis

```bash
kubectl apply -f redis-deployment.yaml
```

Wait for Redis to be ready:

```bash
kubectl wait --for=condition=ready pod -l app=redis -n bss-oss --timeout=300s
```

### 6. Deploy Application

```bash
kubectl apply -f app-deployment.yaml
```

### 7. Check Status

```bash
# Check all pods
kubectl get pods -n bss-oss

# Check services
kubectl get svc -n bss-oss

# Check logs
kubectl logs -f deployment/bss-oss-app -n bss-oss
```

## Accessing the Application

### Get Service URL

```bash
# For LoadBalancer type
kubectl get svc bss-oss-app -n bss-oss

# For NodePort or ClusterIP, use port-forward
kubectl port-forward svc/bss-oss-app 8080:80 -n bss-oss
```

Then access:

- **API**: <http://localhost:8080>
- **Swagger UI**: <http://localhost:8080/swagger-ui>
- **GraphQL Playground**: <http://localhost:8080/graphql>
- **Health Check**: <http://localhost:8080/health>
- **Metrics**: <http://localhost:8080/metrics>

## Health Checks

The deployment includes:

- **Liveness Probe**: `/live` - Checks if the container is running
- **Readiness Probe**: `/ready` - Checks if the app is ready to serve traffic (includes database check)
- **Health Check**: `/health` - General health status

## Monitoring

### Prometheus Metrics

The application exposes Prometheus metrics at `/metrics` endpoint. You can scrape these metrics using Prometheus.

### View Metrics

```bash
# Port-forward to access metrics
kubectl port-forward svc/bss-oss-app 8080:80 -n bss-oss

# View metrics
curl http://localhost:8080/metrics
```

## Scaling

The deployment includes a HorizontalPodAutoscaler (HPA) that automatically scales based on CPU and memory usage:

- **Min Replicas**: 2
- **Max Replicas**: 10
- **CPU Target**: 70%
- **Memory Target**: 80%

### Manual Scaling

```bash
kubectl scale deployment bss-oss-app --replicas=5 -n bss-oss
```

## Troubleshooting

### Check Pod Status

```bash
kubectl describe pod <pod-name> -n bss-oss
```

### Check Logs

```bash
# Application logs
kubectl logs -f deployment/bss-oss-app -n bss-oss

# PostgreSQL logs
kubectl logs -f deployment/postgres -n bss-oss

# Redis logs
kubectl logs -f deployment/redis -n bss-oss
```

### Check Events

```bash
kubectl get events -n bss-oss --sort-by='.lastTimestamp'
```

### Database Connection Issues

```bash
# Check PostgreSQL pod
kubectl exec -it deployment/postgres -n bss-oss -- psql -U bssoss -d bssoss

# Test connection from app pod
kubectl exec -it deployment/bss-oss-app -n bss-oss -- curl http://localhost:8080/ready
```

## Cleanup

To remove all resources:

```bash
kubectl delete namespace bss-oss
```

**Warning:** This will delete all data including persistent volumes. Make sure to backup data before deletion.

## Production Considerations

1. **Secrets Management**: Use a proper secrets management solution (e.g., Sealed Secrets, External Secrets Operator, Vault)
2. **Database Backups**: Set up regular backups for PostgreSQL
3. **Monitoring**: Integrate with Prometheus and Grafana
4. **Logging**: Set up centralized logging (e.g., ELK, Loki)
5. **Ingress**: Configure Ingress controller for external access
6. **TLS**: Enable TLS/HTTPS for all endpoints
7. **Resource Limits**: Adjust resource requests/limits based on your workload
8. **Network Policies**: Implement network policies for security
9. **Pod Disruption Budget**: Add PDB for high availability
10. **Image Security**: Use image scanning and signed images
