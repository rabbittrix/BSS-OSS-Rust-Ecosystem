# BSS/OSS Rust Ecosystem

🌍 **Build an open, secure, and high-performance BSS/OSS ecosystem in Rust**, fully compliant with **TM Forum Open APIs**, offering a **modular**, **interoperable**, and **community-driven** foundation for telecom operators worldwide.

## 🎯 Vision

This project aims to revolutionize the BSS/OSS landscape by providing:

- **Interoperability**: Native TM Forum Open API implementations for seamless integration
- **Business Agility**: Flexible Product Catalog Engine for rapid product innovation
- **Safety & Performance**: Rust's memory safety guarantees prevent costly billing errors
- **Modularity**: Composable crates for different BSS/OSS components

## 🧱 Phase 1 Roadmap – Stable Core Foundation

1. ✅ Implement **TMF620 - Product Catalog Management API**
2. ✅ Add **PostgreSQL (via sqlx)** for real data persistence
3. ✅ Integrate **JWT Authentication** using `jsonwebtoken`
4. ✅ Add **OpenAPI/Swagger** auto-generation using `utoipa` and `utoipa-swagger-ui`
5. ✅ License: **MIT + Donation Link**

## 🏗️ Architecture

### Workspace Structure

```text
bss-oss-rust/
├── crates/
│   ├── tmf-apis/
│   │   ├── core/              # Shared models and errors
│   │   └── tmf620_catalog/    # TMF620 Product Catalog API
│   ├── pcm-engine/            # Product Catalog Engine
│   ├── utils/                 # Logger, helpers, observability
│   └── server/                # Main application server
└── docs/                      # Documentation
```

### Strategic Focus Areas

#### 1. TM Forum API Implementation Library (TMF APIs)

The most strategic choice for interoperability. Adherence to industry standards is crucial for commercial value.

**Current Implementation:**

- **TMF620** - Product Catalog Management API ✅

**Planned:**

- **TMF622** - Product Ordering Management API
- **TMF637** - Product Inventory Management API

#### 2. Product Catalog Engine (PCM) Framework

The heart of the BSS. An efficient catalog allows Telcos to innovate quickly in service offerings.

**Features:**

- Pricing rules and calculations
- Product eligibility validation
- Bundling and product relationships
- Catalog versioning and lifecycle management

## 🚀 Getting Started

### Prerequisites

- Rust 1.70+ (2021 edition)
- PostgreSQL 12+ database
- Cargo (comes with Rust)
- Docker and Docker Compose (optional, for containerized setup)

### Quick Start with Docker Compose (Recommended)

The easiest way to get started:

```bash
# Start PostgreSQL and application
docker-compose up -d

# View logs
docker-compose logs -f app

# Stop services
docker-compose down
```

The application will be available at:

- **API**: <http://localhost:8080>
- **Swagger UI**: <http://localhost:8080/swagger-ui>

### Manual Setup

1. **Clone the repository:**

```bash
git clone https://github.com/your-org/bss-oss-rust.git
cd bss-oss-rust
```

1. **Set up PostgreSQL database:**

   **Option A: Using Docker (PostgreSQL only)**

   ```bash
   docker run --name bss-oss-postgres \
     -e POSTGRES_USER=bssoss \
     -e POSTGRES_PASSWORD=bssoss123 \
     -e POSTGRES_DB=bssoss \
     -p 5432:5432 \
     -d postgres:15-alpine
   ```

   **Option B: Local PostgreSQL**

   ```bash
   # Create database
   createdb bssoss
   # Or using psql
   psql -U postgres -c "CREATE DATABASE bssoss;"
   ```

1. **Run database migrations:**

```bash
psql -U bssoss -d bssoss -f migrations/001_initial_schema.sql
```

1. **Set up environment variables:**

Create a `.env` file or export variables:

```bash
# .env file
DATABASE_URL="postgresql://bssoss:bssoss123@localhost:5432/bssoss"
JWT_SECRET="your-super-secret-jwt-key-change-in-production"
RUST_LOG="info"
HOST="127.0.0.1"
PORT="8080"
```

Or export in shell:

```bash
# Linux/macOS
export DATABASE_URL="postgresql://bssoss:bssoss123@localhost:5432/bssoss"
export JWT_SECRET="your-super-secret-jwt-key-change-in-production"
export RUST_LOG="info"

# Windows PowerShell
$env:DATABASE_URL="postgresql://bssoss:bssoss123@localhost:5432/bssoss"
$env:JWT_SECRET="your-super-secret-jwt-key-change-in-production"
$env:RUST_LOG="info"
```

1. **Build and run:**

```bash
# Build
cargo build --release

# Run
cargo run --bin bss-oss-rust
```

1. **Access the API:**

- **API Base URL**: <http://localhost:8080>
- **Swagger UI**: <http://localhost:8080/swagger-ui>
- **OpenAPI JSON**: <http://localhost:8080/api-doc/openapi.json>

## 📚 Documentation

- [Contributing Guide](docs/CONTRIBUTING.md)
- [Donation Information](docs/DONATION.md)
- [TM Forum Standards](docs/tmf_standards.md)
- [Roadmap](docs/roadmap.md)

## 🔐 Authentication

The API uses JWT authentication. All endpoints require a valid JWT token.

### Using Authentication

Include the token in the Authorization header:

```http
Authorization: Bearer <your-jwt-token>
```

### Testing with Swagger UI

1. Open <http://localhost:8080/swagger-ui>
2. Click the **"Authorize"** button at the top right
3. Enter your JWT token: `Bearer <your-token>`
4. Click **"Authorize"** to save
5. All API calls will now include the authentication header

### Testing with cURL

```bash
curl -X GET http://localhost:8080/tmf-api/productCatalogManagement/v4/catalog \
  -H "Authorization: Bearer <your-token>" \
  -H "Content-Type: application/json"
```

**Note:** Currently, JWT validation is implemented but token generation is not exposed via API. For testing, you can temporarily disable authentication or use a pre-generated token. In production, implement a proper authentication endpoint.

## 📡 API Endpoints

### TMF620 Product Catalog Management API

**Base URL:** `/tmf-api/productCatalogManagement/v4`

#### Catalogs

- **GET** `/catalog` - List all catalogs
- **GET** `/catalog/{id}` - Get catalog by ID (UUID)
- **POST** `/catalog` - Create a new catalog

#### Product Offerings

- **GET** `/productOffering` - List all product offerings
- **POST** `/productOffering` - Create a new product offering

### Example Requests

**Create a catalog:**

```bash
curl -X POST http://localhost:8080/tmf-api/productCatalogManagement/v4/catalog \
  -H "Authorization: Bearer <your-token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Test Catalog",
    "description": "A test catalog",
    "version": "1.0.0",
    "lifecycle_status": "ACTIVE"
  }'
```

**Create a product offering:**

```bash
curl -X POST http://localhost:8080/tmf-api/productCatalogManagement/v4/productOffering \
  -H "Authorization: Bearer <your-token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "5G Premium Plan",
    "description": "High-speed 5G plan",
    "version": "1.0.0",
    "lifecycle_status": "ACTIVE",
    "is_sellable": true,
    "is_bundle": false
  }'
```

## 📚 API Documentation

### Swagger UI

Interactive API documentation is available at <http://localhost:8080/swagger-ui>

**Features:**

- Complete OpenAPI 3.0 specification
- Interactive endpoint testing
- Request/response examples
- Schema documentation
- Authentication support

### OpenAPI JSON

The raw OpenAPI specification is available at <http://localhost:8080/api-doc/openapi.json>

## 🧪 Testing

### Run Tests

```bash
# Run all tests
cargo test

# Run tests for specific crate
cargo test --package tmf620-catalog

# Run tests with output
cargo test -- --nocapture
```

### Test API Endpoints

**Using Swagger UI (Recommended):**

1. Open <http://localhost:8080/swagger-ui>
2. Click "Try it out" on any endpoint
3. Fill in parameters and request body
4. Click "Execute" to test

**Using cURL:**
See examples in the [API Endpoints](#-api-endpoints) section above.

## 📦 Workspace Crates

- **`tmf-apis-core`**: Shared models and error types for all TMF APIs
- **`tmf620-catalog`**: TMF620 Product Catalog Management API implementation
- **`pcm-engine`**: Product Catalog Engine framework (pricing, eligibility, bundling)
- **`bss-oss-utils`**: Common utilities, logger, and helpers
- **`bss-oss-server`**: Main application server (binary)

## 🛠️ Development

### Development Workflow

```bash
# Check code for errors without building
cargo check

# Format code
cargo fmt

# Run linter
cargo clippy

# Run tests
cargo test

# Build in release mode
cargo build --release
```

### Database Management

```bash
# Connect to database
psql -U bssoss -d bssoss

# List all tables
psql -U bssoss -d bssoss -c "\dt"

# Query catalogs
psql -U bssoss -d bssoss -c "SELECT * FROM catalogs;"

# Query product offerings
psql -U bssoss -d bssoss -c "SELECT * FROM product_offerings;"
```

## 🐛 Troubleshooting

**Database connection fails:**

- Verify PostgreSQL is running: `pg_isready` or `docker ps | grep postgres`
- Check `DATABASE_URL` environment variable is set correctly
- Verify database exists and user has proper permissions

**Port 8080 already in use:**

- Change port: `PORT=8081 cargo run --bin bss-oss-rust`
- Or stop the process using port 8080

**Swagger UI shows 404:**

- Ensure server is running
- Access via: <http://localhost:8080/swagger-ui>
- Check server logs for errors

**Unauthorized errors:**

- All endpoints require JWT authentication
- Include `Authorization: Bearer <token>` header
- Verify `JWT_SECRET` environment variable is set

## 🔄 CI/CD and Contributing

### CI/CD Pipeline

This project uses GitHub Actions for continuous integration and deployment:

- **Development** (`develop` branch): Automatic testing and deployment, requires 1 approval for PRs
- **QA** (`qa` branch): Integration tests and QA deployment, requires 1 approval for PRs
- **Production** (`main` branch): Full pipeline with security audits, requires 2 approvals (including owner) for PRs

**Workflow Stages:**

- Lint and format checks
- Unit and integration tests (with PostgreSQL service)
- Multi-platform builds (Linux, Windows, macOS)
- Security audits with `cargo-audit`
- Environment-specific deployments

**Manual Workflow Trigger:**

1. Go to **Actions** tab in GitHub
2. Select **CI/CD Pipeline** workflow
3. Click **Run workflow**
4. Choose environment (development/qa/production)

### Contributing

1. **Fork the repository** (required for external contributors)
2. **Create an issue** describing your changes (use the issue templates)
3. **Create a feature branch** from `develop`:

   ```bash
   git checkout develop
   git pull origin develop
   git checkout -b feature/your-feature-name
   ```

4. **Make your changes** and ensure tests pass
5. **Push to your fork**:

   ```bash
   git push origin feature/your-feature-name
   ```

6. **Create a Pull Request** to `develop` branch
7. **Wait for review and approval** (required)

**Important:** All contributions must go through Pull Requests. Direct pushes to protected branches are not allowed.

### Publishing to crates.io

Crates are automatically published when:

- A GitHub Release is created
- Manual workflow is triggered via Actions tab

**Setup Required:**

1. **Get your crates.io API token:**

   - Log in to <https://crates.io>
   - Go to Account Settings → API Tokens
   - Click "New Token"
   - Enter a descriptive name (e.g., "GitHub Actions - BSS/OSS Rust")
   - **Copy the token immediately** (you won't be able to see it again!)

2. **Add it as a GitHub Secret:**
   - Go to your GitHub repository
   - Navigate to **Settings** → **Secrets and variables** → **Actions**
   - Click **New repository secret**
   - Name: `CRATES_IO_TOKEN`
   - Value: Paste your crates.io token
   - Click **Add secret**

**Manual Publishing:**

1. Go to **Actions** → **Publish to crates.io**
2. Click **Run workflow**
3. Enter version (e.g., `0.1.5`)
4. Select crate(s) to publish (or "all")
5. Click **Run workflow**

## 💚 Support the Project

The BSS/OSS Rust ecosystem is community-driven. See [DONATION.md](docs/DONATION.md) for ways to contribute financially.

**Donation Link:** [https://opencollective.com/bss-oss-rust](https://opencollective.com/bss-oss-rust)

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 👤 Author

### Roberto de Souza

- Email: <rabbittrix@hotmail.com>

## 🙏 Acknowledgments

- TM Forum for the Open API standards
- The Rust community for excellent tooling and libraries
- All contributors and supporters
