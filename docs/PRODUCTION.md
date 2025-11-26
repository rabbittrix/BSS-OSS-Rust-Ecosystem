# Production Readiness Guide

This guide covers production deployment considerations for the BSS/OSS Rust ecosystem.

## Phase 9: Production Readiness Features

### ✅ Implemented Features

1. **Health Checks and Readiness Probes**

   - `/health` - General health check endpoint
   - `/ready` - Readiness probe with database connectivity check
   - `/live` - Liveness probe endpoint
   - Kubernetes-ready health checks

2. **Graceful Shutdown Handling**

   - SIGTERM/SIGINT signal handling
   - 30-second graceful shutdown timeout
   - Proper connection draining

3. **Prometheus Metrics**

   - `/metrics` endpoint for Prometheus scraping
   - HTTP request metrics (count, duration)
   - Active connections gauge
   - Ready for Grafana dashboards

4. **Docker Containerization Improvements**

   - Multi-stage builds for smaller images
   - Non-root user for security
   - Health checks in Dockerfile
   - Optimized layer caching

5. **Kubernetes Deployment**

   - Complete Kubernetes manifests
   - PostgreSQL and Redis deployments
   - Service definitions
   - Persistent volume claims
   - Horizontal Pod Autoscaler (HPA)

6. **Helm Chart**
   - Complete Helm chart for easy deployment
   - Configurable values
   - PostgreSQL and Redis subcharts
   - HPA configuration
   - Production-ready defaults

## Deployment Options

### Option 1: Docker Compose (Development/Testing)

```bash
docker-compose up -d
```

### Option 2: Kubernetes Manifests

```bash
# Apply all manifests
kubectl apply -f k8s/

# Or apply individually
kubectl apply -f k8s/namespace.yaml
kubectl apply -f k8s/configmap.yaml
kubectl apply -f k8s/secret.yaml
kubectl apply -f k8s/postgres-deployment.yaml
kubectl apply -f k8s/redis-deployment.yaml
kubectl apply -f k8s/app-deployment.yaml
```

### Option 3: Helm Chart (Recommended for Production)

```bash
# Install with Helm
helm install bss-oss-rust ./helm/bss-oss-rust \
  --create-namespace \
  --namespace bss-oss \
  --set secrets.JWT_SECRET=your-production-secret \
  --set secrets.DATABASE_PASSWORD=your-db-password
```

## Health Checks

### Endpoints

- **Health**: `GET /health` - Returns service health status
- **Readiness**: `GET /ready` - Checks database connectivity
- **Liveness**: `GET /live` - Simple liveness check
- **Metrics**: `GET /metrics` - Prometheus metrics

### Kubernetes Probes

The deployment includes:

```yaml
livenessProbe:
  httpGet:
    path: /live
    port: 8080
  initialDelaySeconds: 30
  periodSeconds: 10

readinessProbe:
  httpGet:
    path: /ready
    port: 8080
  initialDelaySeconds: 10
  periodSeconds: 5
```

## Monitoring

### Prometheus Integration

The application exposes metrics at `/metrics` in Prometheus format.

**Key Metrics:**

- `bss_oss_http_requests_total` - Total HTTP requests
- `bss_oss_http_request_duration_seconds` - Request duration histogram
- `bss_oss_active_connections` - Active connections gauge

### Grafana Dashboard

Create a Grafana dashboard to visualize:

- Request rate (RPS)
- Request latency (p50, p95, p99)
- Error rates
- Active connections
- Database connection pool status

### Example Prometheus Scrape Config

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

## Observability

### Logging

The application uses structured logging with configurable levels via `RUST_LOG`:

```bash
# Set log level
export RUST_LOG=info
# Or more detailed
export RUST_LOG=debug
```

### Distributed Tracing (Future)

OpenTelemetry support can be added for distributed tracing across microservices.

## Security Considerations

### Secrets Management

**Never commit secrets to version control!**

Use Kubernetes secrets or external secret management:

```bash
# Create secret
kubectl create secret generic bss-oss-secrets \
  --from-literal=JWT_SECRET=your-secret \
  --from-literal=DATABASE_PASSWORD=your-password
```

### Network Policies

Implement network policies to restrict pod-to-pod communication:

```yaml
apiVersion: networking.k8s.io/v1
kind: NetworkPolicy
metadata:
  name: bss-oss-network-policy
spec:
  podSelector:
    matchLabels:
      app: bss-oss-rust
  policyTypes:
    - Ingress
    - Egress
  ingress:
    - from:
        - namespaceSelector:
            matchLabels:
              name: ingress-nginx
      ports:
        - protocol: TCP
          port: 8080
  egress:
    - to:
        - podSelector:
            matchLabels:
              app: postgres
      ports:
        - protocol: TCP
          port: 5432
    - to:
        - podSelector:
            matchLabels:
              app: redis
      ports:
        - protocol: TCP
          port: 6379
```

## Scaling

### Horizontal Pod Autoscaler

The HPA automatically scales based on CPU and memory:

```yaml
minReplicas: 2
maxReplicas: 10
targetCPUUtilizationPercentage: 70
targetMemoryUtilizationPercentage: 80
```

### Manual Scaling

```bash
# Scale deployment
kubectl scale deployment bss-oss-app --replicas=5 -n bss-oss

# Or using Helm
helm upgrade bss-oss-rust ./helm/bss-oss-rust --set replicaCount=5
```

## Database Management

### Backups

Set up regular PostgreSQL backups:

```bash
# Manual backup
kubectl exec -it deployment/postgres -n bss-oss -- \
  pg_dump -U bssoss bssoss > backup.sql

# Restore
kubectl exec -i deployment/postgres -n bss-oss -- \
  psql -U bssoss bssoss < backup.sql
```

### Migrations

Run database migrations on deployment:

```bash
# Apply migrations
kubectl exec -it deployment/bss-oss-app -n bss-oss -- \
  psql $DATABASE_URL -f /app/migrations/001_initial_schema.sql
```

## High Availability

### Pod Disruption Budget

Add a Pod Disruption Budget for high availability:

```yaml
apiVersion: policy/v1
kind: PodDisruptionBudget
metadata:
  name: bss-oss-pdb
spec:
  minAvailable: 1
  selector:
    matchLabels:
      app: bss-oss-rust
```

### Multi-Zone Deployment

Deploy across multiple availability zones:

```yaml
affinity:
  podAntiAffinity:
    preferredDuringSchedulingIgnoredDuringExecution:
      - weight: 100
        podAffinityTerm:
          labelSelector:
            matchExpressions:
              - key: app
                operator: In
                values:
                  - bss-oss-rust
          topologyKey: kubernetes.io/zone
```

## Performance Tuning

### Resource Limits

Adjust based on your workload:

```yaml
resources:
  requests:
    cpu: 500m
    memory: 512Mi
  limits:
    cpu: 2000m
    memory: 2Gi
```

### Database Connection Pool

Configure connection pool size in your application:

```rust
let pool = PgPoolOptions::new()
    .max_connections(20)
    .connect(&database_url)
    .await?;
```

## Disaster Recovery

### Backup Strategy

1. **Database Backups**: Daily automated backups
2. **Configuration**: Version control all configs
3. **Secrets**: Store in secure secret management
4. **Persistent Volumes**: Regular snapshots

### Recovery Procedures

1. Restore database from backup
2. Redeploy application
3. Verify health checks
4. Monitor metrics

## Compliance

### Audit Logging

All operations are logged for compliance:

- Security events (authentication, authorization)
- Data access (CRUD operations)
- Configuration changes
- System events

### Data Retention

Configure audit log retention policies based on compliance requirements.

## Next Steps

### Remaining Phase 9 Tasks

- [ ] TM Forum certification process
- [ ] Comprehensive API documentation enhancements
- [ ] SDK generation for Python, JavaScript, Go
- [ ] OpenTelemetry distributed tracing integration
- [ ] Enhanced monitoring dashboards

### Production Checklist

- [ ] Update all secrets with production values
- [ ] Configure TLS/HTTPS
- [ ] Set up monitoring and alerting
- [ ] Configure log aggregation
- [ ] Set up database backups
- [ ] Implement network policies
- [ ] Configure resource limits
- [ ] Set up CI/CD pipeline
- [ ] Perform security audit
- [ ] Load testing
- [ ] Disaster recovery plan
