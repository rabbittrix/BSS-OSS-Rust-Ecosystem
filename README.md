# BSS/OSS Rust Ecosystem

🌍 **Build an open, secure, and high-performance BSS/OSS ecosystem in Rust**, fully compliant with **TM Forum Open APIs**, offering a **modular**, **interoperable**, and **community-driven** foundation for telecom operators worldwide.

## 🎯 Vision

This project aims to revolutionize the BSS/OSS landscape by providing:

- **Interoperability**: Native TM Forum Open API implementations for seamless integration
- **Business Agility**: Flexible Product Catalog Engine for rapid product innovation
- **Safety & Performance**: Rust's memory safety guarantees prevent costly billing errors
- **Modularity**: Composable crates for different BSS/OSS components

## 🧱 Phase 1 Roadmap – Stable Core Foundation ✅

1. ✅ Implement **TMF620 - Product Catalog Management API**
2. ✅ Add **PostgreSQL (via sqlx)** for real data persistence
3. ✅ Integrate **JWT Authentication** using `jsonwebtoken`
4. ✅ Add **OpenAPI/Swagger** auto-generation using `utoipa` and `utoipa-swagger-ui`
5. ✅ License: **MIT + Donation Link**
6. ✅ Implement **TMF622 - Product Ordering Management API**
7. ✅ Implement **TMF637 - Product Inventory Management API**

### Current Version: 0.2.5

**Phase 2 APIs Completed:** 8. ✅ Implement **TMF629 - Customer Management API** 9. ✅ Implement **TMF678 - Customer Bill Management API** 10. ✅ Implement **TMF679 - Customer Usage Management API** 11. ✅ Implement **TMF688 - Appointment Management API**

## 🧭 Phase 2 Roadmap – Customer & User Domain (High Priority) ✅

### Strategic TMF APIs Implemented

These are critical for any commercial BSS:

1. ✅ **TMF629 – Customer Management**

   - Manages customer profiles and their contact information
   - **Why**: Core for CRM, KYC, account management, onboarding

2. ✅ **TMF678 – Customer Bill Management**

   - Retrieves bills and billing structures
   - **Why**: Required for customer portals and self-care

3. ✅ **TMF679 – Customer Usage Management**

   - Handles CDRs, usage records, and consumption
   - **Why**: Essential for 5G, IoT, prepaid, and analytics

4. ✅ **TMF688 – Appointment Management**
   - For scheduling technician visits, installations, etc.
   - **Why**: Critical for field operations and order fulfillment

### Additional Phase 2 Goals

- 🔄 Add comprehensive test coverage for all TMF APIs
- 🔄 Performance optimization and benchmarking
- 🔄 Add rate limiting and request validation
- 🔄 Enhanced error handling and validation
- 🔄 API versioning and backward compatibility

## 🔥 Phase 3 Roadmap – Service Lifecycle (OSS Core) ✅

### Phase 3 TMF APIs Implemented

These APIs manage the technical services, not the commercial products:

1. ✅ **TMF641 – Service Order Management**

   - Equivalent to TMF622 but for network/service-level
   - A customer order triggers one or more service orders
   - **Why**: You cannot complete fulfillment without this

2. ✅ **TMF638 – Service Inventory**

   - Manages provisioned services and service instances
   - **Why**: You need this to track active network-level services

3. ✅ **TMF640 – Service Activation & Configuration**

   - Handles provisioning actions on network elements
   - **Why**: Connects BSS → OSS → Network

4. ✅ **TMF702 – Resource Activation & Configuration**
   - Low-level provisioning of physical/virtual network elements
   - **Why**: Required for automated orchestration

## ⚡ Phase 4 Roadmap – Resource Domain (Network & Infrastructure) ✅

### Phase 4 TMF APIs Implemented

These APIs manage network resources and infrastructure:

1. ✅ **TMF639 – Resource Inventory**

   - Tracks physical and virtual network resources
   - **Why**: Needed for network planning, provisioning, 5G slicing

2. ✅ **TMF645 – Resource Order Management**
   - Like TMF622 & TMF641, but for network resources
   - **Why**: Service activation depends on resource orders

## 💰 Phase 5 Roadmap – Revenue Management (Charging & Billing) ✅

### Phase 5 TMF APIs Implemented

These APIs manage revenue, charging, and billing:

1. ✅ **TMF678 – Billing Management** (Already implemented in Phase 2)

   - Retrieves bills and billing structures
   - **Why**: Required for customer portals and self-care

2. ✅ **TMF635 – Usage Management**

   - Tracks and queries usage (CDRs, event consumption)
   - **Why**: Needed for OCS/OFCS (5G charging systems)

3. ✅ **TMF668 – Party Role Management**
   - Manages parties, organizations, roles, partners
   - **Why**: Required for partner settlements and ecosystem play

### Phase 5 Revenue Management Features ✅

Beyond the TMF APIs, comprehensive revenue management capabilities have been implemented:

1. ✅ **Real-time Charging Integration**

   - Processes usage events in real-time
   - Applies rating rules and calculates charges
   - Automatic tax calculation
   - Updates usage record states

2. ✅ **Usage Aggregation and Rating**

   - Aggregates usage records by customer, product, and period
   - Supports multiple rate types: Flat, Tiered, Volume, Time-based
   - Configurable rating rules per product offering
   - Tiered rate structures for volume discounts

3. ✅ **Billing Cycle Management**

   - Manages billing cycles (Monthly, Quarterly, Annually, Weekly)
   - Automatically closes cycles and generates bills
   - Aggregates usage and applies rating for bill generation
   - Batch processing of due cycles

4. ✅ **Partner Settlement Workflows**
   - Handles partner revenue sharing
   - Configurable settlement rules with revenue share percentages
   - Calculates settlements for specified periods
   - Supports approval workflow (Pending → Calculated → Approved → Paid)

## 🔒 Phase 6 Roadmap – Security, Party & Identity ✅

### Phase 6 TMF APIs Implemented

These APIs manage security, party management, and identity:

1. ✅ **TMF632 – Party Management**

   - Manages individuals, organizations, account-level attributes
   - **Why**: KYC & business partners depend on it

2. ✅ **TMF669 – Identity & Credential Management**
   - Handles digital identities, credentials, OAuth/JWT integration
   - **Why**: Your system becomes enterprise-ready

### Phase 6 Security Features Implemented

Comprehensive security system for enterprise-grade authentication and authorization:

1. ✅ **OAuth 2.0 / OIDC Integration**

   - OAuth 2.0 authorization server with multiple grant types
   - Authorization code flow with PKCE support
   - Client credentials flow for service-to-service authentication
   - Access token generation, validation, and refresh
   - Token revocation and expiration management
   - OpenID Connect (OIDC) discovery document support

2. ✅ **Multi-Factor Authentication (MFA)**

   - TOTP (Time-based One-Time Password) support with QR code generation
   - SMS-based MFA with challenge codes
   - Email-based MFA with challenge codes
   - Backup codes generation and verification
   - MFA status management and configuration

3. ✅ **Role-Based Access Control (RBAC)**

   - Role creation and management with permissions
   - Permission-based access control (resource:action format)
   - User-role assignments with optional expiration
   - Permission checking methods (has_role, has_permission, has_any_permission, has_all_permissions)
   - Role and permission queries for identities

4. ✅ **Audit Logging for Security Events**
   - Comprehensive security event logging (authentication, authorization, role assignments, etc.)
   - OAuth token events logging (issued, revoked)
   - MFA events logging (enabled, disabled, verified)
   - Security policy violation logging
   - Query capabilities (by identity, event type, date range)
   - Compliance-ready audit trail

## 🧪 Phase 7 Roadmap – Testing & Quality Assurance ✅

### Phase 7 Testing & Quality Assurance Features Implemented

Comprehensive testing infrastructure for quality assurance:

1. ✅ **Comprehensive Unit Test Coverage (>80%)**

   - Unit tests for all security modules (OAuth, MFA, RBAC, Audit)
   - Unit tests for revenue management components
   - Unit tests for service orchestration
   - Test coverage tracking and reporting

2. ✅ **Integration Tests for All TMF APIs**

   - Integration tests for all 17 TMF APIs
   - Database-backed integration tests
   - API endpoint testing utilities

3. ✅ **End-to-End Workflow Tests**

   - Customer onboarding workflow tests
   - Billing cycle workflow tests
   - Service orchestration workflow tests

4. ✅ **Performance Benchmarking and Optimization**

   - API response time benchmarks
   - Concurrent request handling benchmarks
   - Database query performance benchmarks
   - JSON serialization/deserialization benchmarks

5. ✅ **Load Testing and Stress Testing**

   - Load testing utilities with configurable concurrent users
   - Stress testing with gradual user ramp-up
   - Performance metrics collection (RPS, response times, error rates)

6. ✅ **Security Vulnerability Scanning**

   - Automated security audits using `cargo-audit`
   - CI/CD integration for continuous security monitoring
   - Dependency vulnerability tracking

7. ✅ **Code Quality Metrics**
   - Automated clippy linting with strict warnings
   - Code formatting checks with `rustfmt`
   - CI/CD integration for quality gates

## 📡 Phase 8 Roadmap – Open Digital Architecture (MDM, AI, Orchestration) ✅

### Phase 7 TMF APIs Implemented

These APIs manage alarms, network slicing, and orchestration:

1. ✅ **TMF702 – Resource Activation & Configuration** (Already implemented in Phase 3)

   - Low-level provisioning of physical/virtual network elements
   - **Why**: Required for automated orchestration

2. ✅ **TMF642 – Alarm Management**

   - For network alarms, NOC workflows
   - **Why**: Critical for network operations and monitoring

3. ✅ **TMF656 – Slice Management (5G)**
   - Required for new 5G/FTTH products
   - **Why**: Essential for 5G network slicing and service differentiation

## 🏗️ Architecture

### Workspace Structure

```text
bss-oss-rust/
├── crates/
│   ├── tmf-apis/
│   │   ├── core/              # Shared models and errors
│   │   ├── tmf620_catalog/    # TMF620 Product Catalog API ✅
│   │   ├── tmf622_ordering/   # TMF622 Product Ordering API ✅
│   │   ├── tmf637_inventory/  # TMF637 Product Inventory API ✅
│   │   ├── tmf629_customer/   # TMF629 Customer Management API ✅
│   │   ├── tmf678_billing/    # TMF678 Customer Bill Management API ✅
│   │   ├── tmf679_usage/      # TMF679 Customer Usage Management API ✅
│   │   ├── tmf688_appointment/# TMF688 Appointment Management API ✅
│   │   ├── tmf641_service_order/ # TMF641 Service Order Management API ✅
│   │   ├── tmf638_service_inventory/ # TMF638 Service Inventory Management API ✅
│   │   ├── tmf640_service_activation/ # TMF640 Service Activation & Configuration API ✅
│   │   ├── tmf702_resource_activation/ # TMF702 Resource Activation & Configuration API ✅
│   │   ├── tmf639_resource_inventory/ # TMF639 Resource Inventory Management API ✅
│   │   ├── tmf645_resource_order/ # TMF645 Resource Order Management API ✅
│   │   ├── tmf635_usage/ # TMF635 Usage Management API ✅
│   │   ├── tmf668_party_role/ # TMF668 Party Role Management API ✅
│   │   ├── tmf632_party/ # TMF632 Party Management API ✅
│   │   ├── tmf669_identity/ # TMF669 Identity & Credential Management API ✅
│   │   ├── tmf642_alarm/ # TMF642 Alarm Management API ✅
│   │   └── tmf656_slice/ # TMF656 Slice Management API ✅
│   ├── pcm-engine/            # Product Catalog Engine
│   ├── policy-engine/         # Policy Engine (bundling, eligibility, pricing, SLA)
│   ├── api-gateway/           # API Gateway (auth, middleware, rate limiting, validation)
│   ├── event-bus/             # Event Bus abstraction (publisher/subscriber)
│   ├── order-orchestrator/    # Order Orchestration (decomposition, dependencies)
│   ├── service-orchestrator/  # Service Lifecycle Orchestrator ✅
│   ├── resource-management/  # Resource Management (capacity, reservation, topology)
│   ├── revenue-management/   # Revenue Management System ✅
│   ├── security/             # Security System (OAuth 2.0/OIDC, MFA, RBAC, Audit) ✅
│   ├── test-utils/            # Test utilities and fixtures
│   ├── benchmarks/            # Performance benchmarks
│   ├── utils/                 # Logger, helpers, observability
│   └── server/                # Main application server
├── migrations/                # Database migration scripts
└── docs/                      # Documentation
```

### Strategic Focus Areas

#### 1. TM Forum API Implementation Library (TMF APIs)

The most strategic choice for interoperability. Adherence to industry standards is crucial for commercial value.

**Current Implementation (v0.2.5):**

- **TMF620** - Product Catalog Management API ✅
- **TMF622** - Product Ordering Management API ✅
- **TMF637** - Product Inventory Management API ✅
- **TMF629** - Customer Management API ✅
- **TMF678** - Customer Bill Management API ✅
- **TMF679** - Customer Usage Management API ✅
- **TMF688** - Appointment Management API ✅
- **TMF641** - Service Order Management API ✅
- **TMF638** - Service Inventory Management API ✅
- **TMF640** - Service Activation & Configuration API ✅
- **TMF702** - Resource Activation & Configuration API ✅
- **TMF639** - Resource Inventory Management API ✅
- **TMF645** - Resource Order Management API ✅
- **TMF635** - Usage Management API ✅
- **TMF668** - Party Role Management API ✅
- **TMF632** - Party Management API ✅
- **TMF669** - Identity & Credential Management API ✅
- **TMF642** - Alarm Management API ✅
- **TMF656** - Slice Management API ✅

#### 2. Product Catalog Engine (PCM) Framework

The heart of the BSS. An efficient catalog allows Telcos to innovate quickly in service offerings.

**Features:**

- Pricing rules and calculations
- Product eligibility validation
- Bundling and product relationships
- Catalog versioning and lifecycle management

#### 3. Service Orchestrator ✅

Automates the complete service lifecycle from order to activation to inventory.

**Features:**

- Service orchestration workflows (Service Order → Service Activation → Service Inventory)
- Service dependency management with dependency graph
- Automatic service activation when dependencies are met
- Service lifecycle state tracking
- Background worker for processing pending workflows

#### 4. Revenue Management System ✅

Comprehensive revenue management for charging, billing, and partner settlements.

**Features:**

- Real-time charging integration for usage events
- Usage aggregation and rating engine with multiple rate types
- Billing cycle management with automatic bill generation
- Partner settlement workflows with revenue sharing

#### 5. Security System ✅

Enterprise-grade security system for authentication, authorization, and compliance.

**Features:**

- OAuth 2.0 / OIDC integration with multiple grant types and PKCE support
- Multi-factor authentication (TOTP, SMS, Email, Backup Codes)
- Role-based access control (RBAC) with fine-grained permissions
- Comprehensive audit logging for security events and compliance

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

1. **Using Docker Compose (Recommended):**

```bash
# Start all services (PostgreSQL + Application)
docker-compose up -d

# View logs
docker-compose logs -f app

# Stop services
docker-compose down

# Start with test database
docker-compose --profile test up -d postgres_test
```

The Docker Compose setup includes:

- **PostgreSQL** database on port `5432` (main database)
- **PostgreSQL Test** database on port `5433` (for running tests, use `--profile test`)
- **Application** server on port `8080` with Swagger UI
- Automatic database migrations on startup
- Health checks for all services

1. **Manual Setup - Run database migrations:**

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

### TMF622 Product Ordering Management API

**Base URL:** `/tmf-api/productOrderingManagement/v4`

#### Product Orders

- **GET** `/productOrder` - List all product orders
- **GET** `/productOrder/{id}` - Get product order by ID (UUID)
- **POST** `/productOrder` - Create a new product order

### TMF637 Product Inventory Management API

**Base URL:** `/tmf-api/productInventoryManagement/v4`

#### Product Inventories

- **GET** `/productInventory` - List all product inventories
- **GET** `/productInventory/{id}` - Get product inventory by ID (UUID)
- **POST** `/productInventory` - Create a new product inventory

### TMF629 Customer Management API

**Base URL:** `/tmf-api/customerManagement/v4`

#### Customers

- **GET** `/customer` - List all customers
- **GET** `/customer/{id}` - Get customer by ID (UUID)
- **POST** `/customer` - Create a new customer

### TMF678 Customer Bill Management API

**Base URL:** `/tmf-api/customerBillManagement/v4`

#### Customer Bills

- **GET** `/customerBill` - List all customer bills
- **GET** `/customerBill/{id}` - Get customer bill by ID (UUID)
- **POST** `/customerBill` - Create a new customer bill

### TMF679 Customer Usage Management API

**Base URL:** `/tmf-api/customerUsageManagement/v4`

#### Customer Usages

- **GET** `/customerUsage` - List all customer usage records
- **GET** `/customerUsage/{id}` - Get customer usage by ID (UUID)
- **POST** `/customerUsage` - Create a new customer usage record

### TMF688 Appointment Management API

**Base URL:** `/tmf-api/appointmentManagement/v4`

#### Appointments

- **GET** `/appointment` - List all appointments
- **GET** `/appointment/{id}` - Get appointment by ID (UUID)
- **POST** `/appointment` - Create a new appointment

### TMF641 Service Order Management API

**Base URL:** `/tmf-api/serviceOrderingManagement/v4`

#### Service Orders

- **GET** `/serviceOrder` - List all service orders
- **GET** `/serviceOrder/{id}` - Get service order by ID (UUID)
- **POST** `/serviceOrder` - Create a new service order

### TMF638 Service Inventory Management API

**Base URL:** `/tmf-api/serviceInventoryManagement/v4`

#### Service Inventories

- **GET** `/serviceInventory` - List all service inventories
- **GET** `/serviceInventory/{id}` - Get service inventory by ID (UUID)
- **POST** `/serviceInventory` - Create a new service inventory

### TMF640 Service Activation & Configuration API

**Base URL:** `/tmf-api/serviceActivationAndConfiguration/v4`

#### Service Activations

- **GET** `/serviceActivation` - List all service activations
- **GET** `/serviceActivation/{id}` - Get service activation by ID (UUID)
- **POST** `/serviceActivation` - Create a new service activation

### TMF702 Resource Activation & Configuration API

**Base URL:** `/tmf-api/resourceActivationAndConfiguration/v4`

#### Resource Activations

- **GET** `/resourceActivation` - List all resource activations
- **GET** `/resourceActivation/{id}` - Get resource activation by ID (UUID)
- **POST** `/resourceActivation` - Create a new resource activation

### TMF642 Alarm Management API

**Base URL:** `/tmf-api/alarmManagement/v4`

#### Alarms

- **GET** `/alarm` - List all alarms
- **GET** `/alarm/{id}` - Get alarm by ID (UUID)
- **POST** `/alarm` - Create a new alarm
- **PATCH** `/alarm/{id}` - Update an alarm (state, acknowledged_time, cleared_time)
- **DELETE** `/alarm/{id}` - Delete an alarm

### TMF656 Slice Management API

**Base URL:** `/tmf-api/sliceManagement/v4`

#### Network Slices

- **GET** `/networkSlice` - List all network slices
- **GET** `/networkSlice/{id}` - Get network slice by ID (UUID)
- **POST** `/networkSlice` - Create a new network slice
- **PATCH** `/networkSlice/{id}` - Update a network slice (state, activation_date, termination_date)
- **DELETE** `/networkSlice/{id}` - Delete a network slice

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

**Create a product order (TMF622):**

```bash
curl -X POST http://localhost:8080/tmf-api/productOrderingManagement/v4/productOrder \
  -H "Authorization: Bearer <your-token>" \
  -H "Content-Type: application/json" \
  -d '{
    "external_id": "ORDER-001",
    "order_date": "2025-01-15T10:00:00Z",
    "order_state": "ACKNOWLEDGED",
    "order_items": [
      {
        "action": "ADD",
        "product_offering": {
          "id": "product-offering-uuid",
          "name": "5G Premium Plan"
        },
        "quantity": 1
      }
    ]
  }'
```

**Create a product inventory (TMF637):**

```bash
curl -X POST http://localhost:8080/tmf-api/productInventoryManagement/v4/productInventory \
  -H "Authorization: Bearer <your-token>" \
  -H "Content-Type: application/json" \
  -d '{
    "external_id": "INV-001",
    "product_offering": {
      "id": "product-offering-uuid",
      "name": "5G Premium Plan"
    },
    "state": "ACTIVE",
    "quantity": 1
  }'
```

**Create a customer (TMF629):**

```bash
curl -X POST http://localhost:8080/tmf-api/customerManagement/v4/customer \
  -H "Authorization: Bearer <your-token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "John Doe",
    "description": "Premium customer",
    "version": "1.0.0",
    "status": "ACTIVE",
    "contact_medium": [
      {
        "medium_type": "email",
        "preferred": true,
        "value": "john.doe@example.com",
        "contact_type": "primary"
      }
    ]
  }'
```

**Create a customer bill (TMF678):**

```bash
curl -X POST http://localhost:8080/tmf-api/customerBillManagement/v4/customerBill \
  -H "Authorization: Bearer <your-token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Bill-2025-01",
    "description": "Monthly bill for January 2025",
    "version": "1.0.0",
    "bill_date": "2025-01-31T00:00:00Z",
    "due_date": "2025-02-15T00:00:00Z",
    "total_amount": {
      "value": 99.99,
      "unit": "USD"
    },
    "tax_included": true,
    "bill_item": [
      {
        "description": "5G Premium Plan - Monthly",
        "amount": {
          "value": 99.99,
          "unit": "USD"
        },
        "quantity": 1
      }
    ]
  }'
```

**Create a customer usage record (TMF679):**

```bash
curl -X POST http://localhost:8080/tmf-api/customerUsageManagement/v4/customerUsage \
  -H "Authorization: Bearer <your-token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Usage-2025-01-15",
    "description": "Data usage for January 15, 2025",
    "version": "1.0.0",
    "usage_date": "2025-01-15T12:00:00Z",
    "start_date": "2025-01-15T00:00:00Z",
    "end_date": "2025-01-15T23:59:59Z",
    "usage_type": "DATA",
    "amount": 1024.5,
    "unit": "MB"
  }'
```

**Create an appointment (TMF688):**

```bash
curl -X POST http://localhost:8080/tmf-api/appointmentManagement/v4/appointment \
  -H "Authorization: Bearer <your-token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Installation Appointment",
    "description": "5G router installation",
    "version": "1.0.0",
    "appointment_date": "2025-02-01T10:00:00Z",
    "duration": 120,
    "appointment_type": "INSTALLATION",
    "related_party": [
      {
        "name": "John Doe",
        "role": "CUSTOMER"
      }
    ],
    "contact_medium": [
      {
        "medium_type": "address",
        "value": "123 Main St, City, State 12345"
      }
    ]
  }'
```

**Create a service order (TMF641):**

```bash
curl -X POST http://localhost:8080/tmf-api/serviceOrderingManagement/v4/serviceOrder \
  -H "Authorization: Bearer <your-token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Service Order-001",
    "description": "5G service provisioning order",
    "version": "1.0.0",
    "priority": "HIGH",
    "external_id": "PROD-ORDER-001",
    "order_item": [
      {
        "action": "ADD",
        "service_specification_id": "service-spec-uuid",
        "quantity": 1
      }
    ]
  }'
```

**Create a service inventory (TMF638):**

```bash
curl -X POST http://localhost:8080/tmf-api/serviceInventoryManagement/v4/serviceInventory \
  -H "Authorization: Bearer <your-token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Service-Instance-001",
    "description": "Active 5G service instance",
    "version": "1.0.0",
    "service_specification_id": "service-spec-uuid",
    "service_id": "service-uuid"
  }'
```

**Create a service activation (TMF640):**

```bash
curl -X POST http://localhost:8080/tmf-api/serviceActivationAndConfiguration/v4/serviceActivation \
  -H "Authorization: Bearer <your-token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Service Activation-001",
    "description": "Activate 5G service on network",
    "version": "1.0.0",
    "service_id": "service-uuid",
    "service_order_id": "service-order-uuid",
    "configuration": [
      {
        "name": "bandwidth",
        "value": "1000",
        "description": "Bandwidth in Mbps"
      }
    ]
  }'
```

**Create a resource activation (TMF702):**

```bash
curl -X POST http://localhost:8080/tmf-api/resourceActivationAndConfiguration/v4/resourceActivation \
  -H "Authorization: Bearer <your-token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Resource Activation-001",
    "description": "Activate network resource",
    "version": "1.0.0",
    "resource_id": "resource-uuid",
    "service_activation_id": "service-activation-uuid",
    "configuration": [
      {
        "name": "port",
        "value": "eth0",
        "description": "Network port"
      }
    ]
  }'
```

**Create a resource inventory (TMF639):**

```bash
curl -X POST http://localhost:8080/tmf-api/resourceInventoryManagement/v4/resourceInventory \
  -H "Authorization: Bearer <your-token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Network-Resource-001",
    "description": "5G base station resource",
    "version": "1.0.0",
    "resource_type": "PHYSICAL",
    "resource_specification_id": "resource-spec-uuid"
  }'
```

**Create a resource order (TMF645):**

```bash
curl -X POST http://localhost:8080/tmf-api/resourceOrderingManagement/v4/resourceOrder \
  -H "Authorization: Bearer <your-token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Resource Order-001",
    "description": "Network resource provisioning order",
    "version": "1.0.0",
    "priority": "HIGH",
    "external_id": "SERVICE-ORDER-001",
    "order_item": [
      {
        "action": "ADD",
        "resource_specification_id": "resource-spec-uuid",
        "quantity": 1
      }
    ]
  }'
```

**Create a usage record (TMF635):**

```bash
curl -X POST http://localhost:8080/tmf-api/usageManagement/v4/usage \
  -H "Authorization: Bearer <your-token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Usage-2025-01-15",
    "description": "Data usage record",
    "version": "1.0.0",
    "usage_type": "DATA",
    "usage_date": "2025-01-15T12:00:00Z",
    "start_date": "2025-01-15T00:00:00Z",
    "end_date": "2025-01-15T23:59:59Z",
    "amount": 1024.5,
    "unit": "MB"
  }'
```

**Create a party role (TMF668):**

```bash
curl -X POST http://localhost:8080/tmf-api/partyRoleManagement/v4/partyRole \
  -H "Authorization: Bearer <your-token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Partner-ABC",
    "description": "Reseller partner",
    "version": "1.0.0",
    "role": "RESELLER",
    "party_type": "ORGANIZATION",
    "engagement_date": "2025-01-01T00:00:00Z",
    "contact_medium": [
      {
        "medium_type": "email",
        "value": "partner@example.com",
        "preferred": true
      }
    ]
  }'
```

**Create a party (TMF632):**

```bash
curl -X POST http://localhost:8080/tmf-api/partyManagement/v4/party \
  -H "Authorization: Bearer <your-token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "John Doe",
    "description": "Individual customer",
    "version": "1.0.0",
    "party_type": "INDIVIDUAL",
    "registration_date": "2025-01-01T00:00:00Z",
    "contact_medium": [
      {
        "medium_type": "email",
        "value": "john.doe@example.com",
        "preferred": true
      }
    ],
    "characteristic": [
      {
        "name": "taxId",
        "value": "123-45-6789",
        "value_type": "string"
      }
    ]
  }'
```

**Create an identity (TMF669):**

```bash
curl -X POST http://localhost:8080/tmf-api/identityManagement/v4/identity \
  -H "Authorization: Bearer <your-token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "user-identity-001",
    "description": "User identity for API access",
    "version": "1.0.0",
    "identity_type": "USER",
    "party_id": "party-uuid",
    "oauth_client_id": "client-123",
    "jwt_issuer": "https://api.example.com",
    "expiration_date": "2026-01-01T00:00:00Z",
    "credential": [
      {
        "credential_type": "JWT",
        "expiration_date": "2026-01-01T00:00:00Z"
      }
    ]
  }'
```

**Create an alarm (TMF642):**

```bash
curl -X POST http://localhost:8080/tmf-api/alarmManagement/v4/alarm \
  -H "Authorization: Bearer <your-token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Network Interface Down",
    "description": "Network interface eth0 is down",
    "version": "1.0.0",
    "severity": "CRITICAL",
    "alarm_type": "COMMUNICATIONS_ALARM",
    "source_resource_id": "resource-uuid",
    "raised_time": "2025-01-19T10:00:00Z",
    "alarm_details": "Interface eth0 on router-01 is down"
  }'
```

**Create a network slice (TMF656):**

```bash
curl -X POST http://localhost:8080/tmf-api/sliceManagement/v4/networkSlice \
  -H "Authorization: Bearer <your-token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "5G-eMBB-Slice-001",
    "description": "Enhanced Mobile Broadband slice for high-speed data",
    "version": "1.0.0",
    "slice_type": "ENHANCED_MOBILE_BROADBAND",
    "sla_parameters": {
      "max_latency_ms": 10,
      "min_throughput_mbps": 1000,
      "max_devices": 10000,
      "coverage_area": "Metropolitan Area"
    },
    "activation_date": "2025-01-20T00:00:00Z"
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

### Test Infrastructure

The project includes comprehensive testing infrastructure:

- **Unit Tests**: Located in each crate's `tests/` directory
- **Integration Tests**: Test TMF API endpoints with database
- **End-to-End Tests**: Test complete workflows
- **Performance Benchmarks**: Using Criterion for benchmarking
- **Load Testing**: Utilities for load and stress testing
- **Coverage Reporting**: Using cargo-tarpaulin for coverage analysis

### Run Tests

#### Using Docker Compose (Recommended)

```bash
# Start test database
docker-compose --profile test up -d postgres_test

# Set test database URL
export TEST_DATABASE_URL="postgresql://bssoss:bssoss123@localhost:5433/bssoss_test"

# Run all tests
cargo test --all-targets

# Stop test database when done
docker-compose --profile test down postgres_test
```

#### Local Testing

```bash
# Run all tests
cargo test --all-targets

# Run tests for specific crate
cargo test --package security

# Run tests with output
cargo test --all-targets -- --nocapture

# Run integration tests
cargo test --test integration_tmf_apis

# Run end-to-end tests
cargo test --test e2e_workflows

# Run all tests using script
./scripts/run-all-tests.sh
```

### Test Coverage

```bash
# Install cargo-tarpaulin
cargo install cargo-tarpaulin

# Generate coverage report
cargo tarpaulin --out Xml --out Html --output-dir coverage

# View report
open coverage/tarpaulin-report.html
```

**Coverage Target:** >80% code coverage

### Performance Benchmarks

```bash
# Run benchmarks
cargo bench --all-targets

# View benchmark results
open target/criterion/*/report/index.html
```

### Load Testing

```bash
# Using test utilities (Rust)
cargo test --package test-utils --features load-testing

# Using scripts (requires ab or wrk)
./scripts/load-test.sh
```

### Security Auditing

```bash
# Install cargo-audit
cargo install cargo-audit

# Run security audit
cargo audit

# Or use script
./scripts/security-audit.sh
```

### Code Quality

```bash
# Check formatting
cargo fmt --all -- --check

# Format code
cargo fmt --all

# Run clippy
cargo clippy --all-targets --all-features -- -D warnings
```

### Test API Endpoints

#### Using Swagger UI (Recommended)

Swagger UI provides an interactive interface to test all TMF APIs:

1. **Start the application:**

   ```bash
   # Using Docker Compose
   docker-compose up -d

   # Or locally
   cargo run --bin bss-oss-rust
   ```

2. **Access Swagger UI:**

   - Open <http://localhost:8080/swagger-ui> in your browser
   - All 17 TMF APIs are available and documented

3. **Test Endpoints:**

   - Click on any API endpoint to expand it
   - Click "Try it out" to enable testing
   - Fill in required parameters and request body (JSON)
   - Click "Execute" to send the request
   - View the response, status code, and headers

4. **Authentication (if required):**

   - Click the "Authorize" button at the top right
   - Enter your JWT token: `Bearer <your-token>`
   - Click "Authorize" to save
   - All subsequent requests will include the authentication header

5. **Test All APIs:**

   - **TMF620**: Product Catalog Management (catalogs, product offerings)
   - **TMF622**: Product Ordering (create, list, get orders)
   - **TMF637**: Product Inventory (inventory management)
   - **TMF629**: Customer Management (customers, contact info)
   - **TMF678**: Customer Bill Management (bills, billing structures)
   - **TMF679**: Customer Usage Management (usage records, CDRs)
   - **TMF688**: Appointment Management (scheduling, technician visits)
   - **TMF641**: Service Order Management (service-level orders)
   - **TMF638**: Service Inventory (provisioned services)
   - **TMF640**: Service Activation (service provisioning)
   - **TMF702**: Resource Activation (network element provisioning)
   - **TMF639**: Resource Inventory (network resources)
   - **TMF645**: Resource Order Management (resource orders)
   - **TMF635**: Usage Management (usage tracking, queries)
   - **TMF668**: Party Role Management (parties, organizations, roles)
   - **TMF632**: Party Management (individuals, organizations)
   - **TMF669**: Identity & Credential Management (OAuth, JWT)
   - **TMF642**: Alarm Management (network alarms, NOC workflows)
   - **TMF656**: Slice Management (5G network slicing)

6. **View OpenAPI Specification:**
   - Access the raw OpenAPI JSON at <http://localhost:8080/api-doc/openapi.json>
   - This can be imported into Postman, Insomnia, or other API clients

#### Using cURL

See examples in the [API Endpoints](#-api-endpoints) section above.

#### Running Tests via Swagger

You can use Swagger UI to manually test all endpoints and verify:

- Request/response formats
- Error handling
- Authentication flows
- Data validation
- API documentation accuracy

**For detailed testing documentation, see [docs/TESTING.md](docs/TESTING.md)**

## 📦 Workspace Crates

### TMF API Crates

- **`tmf-apis-core`**: Shared models and error types for all TMF APIs
- **`tmf620-catalog`**: TMF620 Product Catalog Management API implementation
- **`tmf622-ordering`**: TMF622 Product Ordering Management API implementation
- **`tmf637-inventory`**: TMF637 Product Inventory Management API implementation
- **`tmf629-customer`**: TMF629 Customer Management API implementation
- **`tmf678-billing`**: TMF678 Customer Bill Management API implementation
- **`tmf679-usage`**: TMF679 Customer Usage Management API implementation
- **`tmf688-appointment`**: TMF688 Appointment Management API implementation
- **`tmf641-service-order`**: TMF641 Service Order Management API implementation
- **`tmf638-service-inventory`**: TMF638 Service Inventory Management API implementation
- **`tmf640-service-activation`**: TMF640 Service Activation & Configuration API implementation
- **`tmf702-resource-activation`**: TMF702 Resource Activation & Configuration API implementation
- **`tmf639-resource-inventory`**: TMF639 Resource Inventory Management API implementation
- **`tmf645-resource-order`**: TMF645 Resource Order Management API implementation
- **`tmf635-usage`**: TMF635 Usage Management API implementation
- **`tmf668-party-role`**: TMF668 Party Role Management API implementation
- **`tmf632-party`**: TMF632 Party Management API implementation
- **`tmf669-identity`**: TMF669 Identity & Credential Management API implementation
- **`tmf642-alarm`**: TMF642 Alarm Management API implementation
- **`tmf656-slice`**: TMF656 Slice Management API implementation

### Core Engine Crates

- **`pcm-engine`**: Product Catalog Engine framework (pricing, eligibility, bundling)
- **`policy-engine`**: Policy Engine (bundling, eligibility, pricing, network, SLA policies)
- **`order-orchestrator`**: Order orchestration (decomposition, dependencies, state management)
- **`service-orchestrator`**: Service lifecycle orchestrator (workflows, dependencies, activation automation) ✅
- **`resource-management`**: Resource management (capacity, reservation, network topology)
- **`revenue-management`**: Revenue management system (charging, rating, billing cycles, settlements) ✅
- **`security`**: Security system (OAuth 2.0/OIDC, MFA, RBAC, audit logging) ✅

### Infrastructure Crates

- **`api-gateway`**: API Gateway (authentication, middleware, rate limiting, validation, versioning)
- **`event-bus`**: Event Bus abstraction (publisher/subscriber for event-driven architecture)
- **`utils`**: Common utilities, logger, and observability helpers
- **`test-utils`**: Test utilities, fixtures, and integration test helpers
- **`benchmarks`**: Performance benchmarking utilities
- **`server`**: Main application server (binary)

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

# Query product orders (TMF622)
psql -U bssoss -d bssoss -c "SELECT * FROM product_orders;"

# Query product inventories (TMF637)
psql -U bssoss -d bssoss -c "SELECT * FROM product_inventories;"

# Query customers (TMF629)
psql -U bssoss -d bssoss -c "SELECT * FROM customers;"

# Query customer bills (TMF678)
psql -U bssoss -d bssoss -c "SELECT * FROM customer_bills;"

# Query customer usages (TMF679)
psql -U bssoss -d bssoss -c "SELECT * FROM customer_usages;"

# Query appointments (TMF688)
psql -U bssoss -d bssoss -c "SELECT * FROM appointments;"

# Query service orders (TMF641)
psql -U bssoss -d bssoss -c "SELECT * FROM service_orders;"

# Query service inventories (TMF638)
psql -U bssoss -d bssoss -c "SELECT * FROM service_inventories;"

# Query service activations (TMF640)
psql -U bssoss -d bssoss -c "SELECT * FROM service_activations;"

# Query resource activations (TMF702)
psql -U bssoss -d bssoss -c "SELECT * FROM resource_activations;"

# Query resource inventories (TMF639)
psql -U bssoss -d bssoss -c "SELECT * FROM resource_inventories;"

# Query resource orders (TMF645)
psql -U bssoss -d bssoss -c "SELECT * FROM resource_orders;"

# Query usage records (TMF635)
psql -U bssoss -d bssoss -c "SELECT * FROM usages;"

# Query party roles (TMF668)
psql -U bssoss -d bssoss -c "SELECT * FROM party_roles;"

# Query parties (TMF632)
psql -U bssoss -d bssoss -c "SELECT * FROM parties;"

# Query identities (TMF669)
psql -U bssoss -d bssoss -c "SELECT * FROM identities;"

# Query alarms (TMF642)
psql -U bssoss -d bssoss -c "SELECT * FROM alarms;"

# Query network slices (TMF656)
psql -U bssoss -d bssoss -c "SELECT * FROM network_slices;"

# Query revenue management tables
psql -U bssoss -d bssoss -c "SELECT * FROM charging_results;"
psql -U bssoss -d bssoss -c "SELECT * FROM rating_rules;"
psql -U bssoss -d bssoss -c "SELECT * FROM billing_cycles;"
psql -U bssoss -d bssoss -c "SELECT * FROM partner_settlements;"
psql -U bssoss -d bssoss -c "SELECT * FROM settlement_rules;"

# Query security tables
psql -U bssoss -d bssoss -c "SELECT * FROM oauth_clients;"
psql -U bssoss -d bssoss -c "SELECT * FROM access_tokens;"
psql -U bssoss -d bssoss -c "SELECT * FROM mfa_configs;"
psql -U bssoss -d bssoss -c "SELECT * FROM roles;"
psql -U bssoss -d bssoss -c "SELECT * FROM user_roles;"
psql -U bssoss -d bssoss -c "SELECT * FROM audit_logs;"
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
