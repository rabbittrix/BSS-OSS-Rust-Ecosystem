# BSS/OSS Rust Ecosystem

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)

🌍 **A high-performance, memory-safe BSS/OSS ecosystem in Rust**, fully compliant with **TM Forum Open APIs** (TMF). Built for telecom operators who need **interoperability**, **safety**, and **performance**.

## What is BSS/OSS?

**BSS (Business Support Systems)** and **OSS (Operations Support Systems)** are critical software platforms for telecommunications companies. They handle:

- **BSS**: Customer management, billing, product catalogs, orders
- **OSS**: Service provisioning, network inventory, service activation, resource management

This project provides a complete, production-ready implementation of TM Forum's standardized APIs, enabling seamless integration between different telecom systems.

## Features

✅ **17 TM Forum APIs Implemented**:

- **Phase 1 (Product Domain)**: TMF620 (Catalog), TMF622 (Ordering), TMF637 (Inventory)
- **Phase 2 (Customer Domain)**: TMF629 (Customer), TMF678 (Billing), TMF679 (Usage), TMF688 (Appointment)
- **Phase 3 (Service Lifecycle)**: TMF641 (Service Order), TMF638 (Service Inventory), TMF640 (Service Activation), TMF702 (Resource Activation)
- **Phase 4 (Resource Domain)**: TMF639 (Resource Inventory), TMF645 (Resource Order)
- **Phase 5 (Revenue Management)**: TMF635 (Usage), TMF668 (Party Role)
- **Phase 6 (Security, Party & Identity)**: TMF632 (Party), TMF669 (Identity & Credential)

✅ **Production-Ready Features**:

- PostgreSQL database integration via `sqlx`
- JWT authentication for secure API access
- Auto-generated OpenAPI/Swagger documentation
- Async/await support with Actix Web
- Type-safe Rust implementation
- Docker Compose setup included

✅ **Memory Safety**: Rust's compile-time guarantees prevent common bugs that could lead to billing errors or security vulnerabilities.

## Quick Start

### Prerequisites

- Rust 1.70 or later
- PostgreSQL 15+ (or use Docker Compose)
- Docker and Docker Compose (optional)

### Installation

```bash
# Clone the repository
git clone https://github.com/rabbittrix/BSS-OSS-Rust-Ecosystem.git
cd BSS-OSS-Rust-Ecosystem

# Start PostgreSQL and the application
docker-compose up -d

# Or run manually
cargo run --release --bin bss-oss-rust
```

### Usage Example

```rust
// The server automatically starts with all TMF APIs enabled
// Access Swagger UI at: http://localhost:8080/swagger-ui/

// Example: Create a product catalog
curl -X POST http://localhost:8080/tmf-api/productCatalogManagement/v4/catalog \
  -H "Authorization: Bearer <your-token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "5G Services Catalog",
    "description": "Catalog for 5G service offerings",
    "version": "1.0.0",
    "lifecycle_status": "ACTIVE"
  }'
```

## Architecture

This is a **Cargo workspace** with modular crates:

```text
bss-oss-rust/
├── crates/
│   ├── tmf-apis/          # Individual TMF API implementations
│   │   ├── core/          # Shared models and error types
│   │   ├── tmf620_catalog/
│   │   ├── tmf622_ordering/
│   │   └── ... (17 total APIs)
│   ├── server/            # Main application server
│   ├── pcm-engine/        # Product Catalog Engine
│   └── utils/             # Common utilities
└── migrations/            # Database schema migrations
```

## Available APIs

All APIs follow TM Forum Open API standards and are accessible via REST:

| API    | Endpoint                                         | Description                 |
| ------ | ------------------------------------------------ | --------------------------- |
| TMF620 | `/tmf-api/productCatalogManagement/v4`           | Product catalog management  |
| TMF622 | `/tmf-api/productOrderingManagement/v4`          | Product order management    |
| TMF637 | `/tmf-api/productInventoryManagement/v4`         | Product inventory tracking  |
| TMF629 | `/tmf-api/customerManagement/v4`                 | Customer profile management |
| TMF678 | `/tmf-api/customerBillManagement/v4`             | Customer billing            |
| TMF679 | `/tmf-api/customerUsageManagement/v4`            | Usage records (CDRs)        |
| TMF688 | `/tmf-api/appointmentManagement/v4`              | Appointment scheduling      |
| TMF641 | `/tmf-api/serviceOrderingManagement/v4`          | Service order management    |
| TMF638 | `/tmf-api/serviceInventoryManagement/v4`         | Service inventory           |
| TMF640 | `/tmf-api/serviceActivationAndConfiguration/v4`  | Service activation          |
| TMF702 | `/tmf-api/resourceActivationAndConfiguration/v4` | Resource activation         |
| TMF639 | `/tmf-api/resourceInventoryManagement/v4`        | Resource inventory          |
| TMF645 | `/tmf-api/resourceOrderingManagement/v4`         | Resource order management   |
| TMF635 | `/tmf-api/usageManagement/v4`                    | Usage management            |
| TMF668 | `/tmf-api/partyRoleManagement/v4`                | Party role management       |
| TMF632 | `/tmf-api/partyManagement/v4`                    | Party management            |
| TMF669 | `/tmf-api/identityManagement/v4`                 | Identity & credential mgmt  |

## Documentation

- **Swagger UI**: `http://localhost:8080/swagger-ui/` (when server is running)
- **Full Documentation**: See [README.md](https://github.com/rabbittrix/BSS-OSS-Rust-Ecosystem/blob/main/README.md) for complete setup and usage guide
- **TM Forum Standards**: [TM Forum Open APIs](https://www.tmforum.org/open-apis/)

## Why Rust?

- **Memory Safety**: No null pointer dereferences, buffer overflows, or data races
- **Performance**: Comparable to C/C++ with zero-cost abstractions
- **Concurrency**: Built-in async/await for high-throughput APIs
- **Type Safety**: Catch errors at compile-time, not runtime
- **Ecosystem**: Growing ecosystem with excellent tooling (Cargo, Clippy, Rustfmt)

## Contributing

Contributions are welcome! This project aims to be a community-driven implementation of TM Forum standards.

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Submit a pull request

## License

This project is licensed under the MIT License - see the [LICENSE](https://github.com/rabbittrix/BSS-OSS-Rust-Ecosystem/blob/main/LICENSE) file for details.

## Author

**Roberto de Souza** - [rabbittrix@hotmail.com](mailto:rabbittrix@hotmail.com)

## Acknowledgments

- [TM Forum](https://www.tmforum.org/) for the Open API standards
- The Rust community for excellent tools and libraries
- All contributors who help improve this project

---

Built with ❤️ in Rust for the telecom industry
