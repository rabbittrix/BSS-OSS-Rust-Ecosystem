# TM Forum Open API Standards

This document outlines the TM Forum Open API standards implemented in this project.

## Overview

The TM Forum Open APIs provide standardized interfaces for BSS/OSS systems, enabling interoperability between different vendors and systems.

## Implemented APIs

### TMF620 - Product Catalog Management API

**Status:** ✅ Implemented

The Product Catalog Management API provides capabilities for managing product catalogs, product offerings, and product specifications.

**Key Features:**

- Catalog management (CRUD operations)
- Product offering management
- Product specification references
- Lifecycle status management

**Endpoints:**

- `GET /tmf-api/productCatalogManagement/v4/catalog` - List catalogs
- `GET /tmf-api/productCatalogManagement/v4/catalog/{id}` - Get catalog by ID
- `POST /tmf-api/productCatalogManagement/v4/catalog` - Create catalog
- `GET /tmf-api/productCatalogManagement/v4/productOffering` - List product offerings
- `POST /tmf-api/productCatalogManagement/v4/productOffering` - Create product offering

## Planned APIs

### TMF622 - Product Ordering Management API

**Status:** 🔄 Planned

Manages customer orders for products and services.

### TMF637 - Product Inventory Management API

**Status:** 🔄 Planned

Manages what products and services customers own.

## Compliance

This project aims for full compliance with TM Forum Open API specifications. All implementations follow the official TMF API guidelines and patterns.

## Resources

- [TM Forum Open APIs](https://www.tmforum.org/open-apis/)
- [TMF620 Specification](https://www.tmforum.org/open-apis/)
- [TMF622 Specification](https://www.tmforum.org/open-apis/)
- [TMF637 Specification](https://www.tmforum.org/open-apis/)
