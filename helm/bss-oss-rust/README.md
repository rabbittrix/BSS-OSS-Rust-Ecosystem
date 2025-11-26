# BSS/OSS Rust Helm Chart

A Helm chart for deploying the BSS/OSS Rust ecosystem on Kubernetes.

## Prerequisites

- Kubernetes 1.20+
- Helm 3.0+
- kubectl configured to access your cluster

## Installation

### Add the Chart Repository (if published)

```bash
helm repo add bss-oss-rust https://charts.example.com
helm repo update
```

### Install from Local Chart

```bash
# Install with default values
helm install bss-oss-rust ./helm/bss-oss-rust

# Install with custom values
helm install bss-oss-rust ./helm/bss-oss-rust -f my-values.yaml

# Install with specific namespace
helm install bss-oss-rust ./helm/bss-oss-rust --create-namespace --namespace bss-oss
```

### Install with Custom Configuration

```bash
helm install bss-oss-rust ./helm/bss-oss-rust \
  --set image.tag=v0.2.5 \
  --set replicaCount=3 \
  --set secrets.JWT_SECRET=your-secret-key \
  --set postgres.persistence.size=20Gi
```

## Configuration

The following table lists the configurable parameters:

| Parameter                   | Description         | Default        |
| --------------------------- | ------------------- | -------------- |
| `replicaCount`              | Number of replicas  | `2`            |
| `image.repository`          | Image repository    | `bss-oss-rust` |
| `image.tag`                 | Image tag           | `latest`       |
| `image.pullPolicy`          | Image pull policy   | `IfNotPresent` |
| `service.type`              | Service type        | `LoadBalancer` |
| `service.port`              | Service port        | `80`           |
| `postgres.enabled`          | Enable PostgreSQL   | `true`         |
| `postgres.persistence.size` | PostgreSQL PVC size | `10Gi`         |
| `redis.enabled`             | Enable Redis        | `true`         |
| `redis.persistence.size`    | Redis PVC size      | `5Gi`          |
| `autoscaling.enabled`       | Enable HPA          | `true`         |
| `autoscaling.minReplicas`   | Min replicas        | `2`            |
| `autoscaling.maxReplicas`   | Max replicas        | `10`           |
| `resources.limits.cpu`      | CPU limit           | `1000m`        |
| `resources.limits.memory`   | Memory limit        | `1Gi`          |
| `resources.requests.cpu`    | CPU request         | `500m`         |
| `resources.requests.memory` | Memory request      | `512Mi`        |

## Upgrading

```bash
# Upgrade with new values
helm upgrade bss-oss-rust ./helm/bss-oss-rust -f my-values.yaml

# Upgrade with specific values
helm upgrade bss-oss-rust ./helm/bss-oss-rust \
  --set image.tag=v0.2.6 \
  --reuse-values
```

## Uninstallation

```bash
helm uninstall bss-oss-rust
```

## Accessing the Application

After installation, get the service URL:

```bash
# For LoadBalancer
kubectl get svc bss-oss-rust -n default

# Port forward
kubectl port-forward svc/bss-oss-rust 8080:80
```

Access:

- **API**: <http://localhost:8080>
- **Swagger UI**: <http://localhost:8080/swagger-ui>
- **GraphQL Playground**: <http://localhost:8080/graphql>
- **Health**: <http://localhost:8080/health>
- **Metrics**: <http://localhost:8080/metrics>

## Monitoring

The chart exposes Prometheus metrics at `/metrics`. Configure Prometheus to scrape:

```yaml
scrape_configs:
  - job_name: "bss-oss-rust"
    kubernetes_sd_configs:
      - role: pod
    relabel_configs:
      - source_labels: [__meta_kubernetes_pod_label_app_kubernetes_io_name]
        action: keep
        regex: bss-oss-rust
```

## Production Recommendations

1. **Use Production Secrets**: Update `secrets.JWT_SECRET` and `secrets.DATABASE_PASSWORD`
2. **Enable Ingress**: Set `ingress.enabled=true` and configure TLS
3. **Resource Limits**: Adjust based on your workload
4. **Persistence**: Ensure PostgreSQL and Redis persistence is enabled
5. **Backups**: Set up regular database backups
6. **Monitoring**: Integrate with Prometheus/Grafana
7. **Logging**: Set up centralized logging

## Troubleshooting

### Check Pod Status

```bash
kubectl get pods -l app.kubernetes.io/name=bss-oss-rust
```

### View Logs

```bash
kubectl logs -l app.kubernetes.io/name=bss-oss-rust --tail=100
```

### Describe Pod

```bash
kubectl describe pod <pod-name>
```

### Check Events

```bash
kubectl get events --sort-by='.lastTimestamp'
```
