# TM Forum Open API Standards

This document outlines the TM Forum Open API standards implemented in this project.

## Overview

The TM Forum Open APIs provide standardized interfaces for BSS/OSS systems, enabling interoperability between different vendors and systems. This project implements a comprehensive set of TMF APIs covering Product Management, Customer Management, Service Lifecycle, Resource Management, Revenue Management, and Security & Identity domains.

## Current Version: 0.2.0

## Implemented APIs

### Phase 1: Product Domain ✅

#### TMF620 - Product Catalog Management API

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

#### TMF622 - Product Ordering Management API

**Status:** ✅ Implemented

Manages customer orders for products and services. Handles the complete order lifecycle from creation to fulfillment.

**Key Features:**

- Product order creation and management
- Order item management
- Order state tracking
- Related party associations

**Endpoints:**

- `GET /tmf-api/productOrderingManagement/v4/productOrder` - List product orders
- `GET /tmf-api/productOrderingManagement/v4/productOrder/{id}` - Get product order by ID
- `POST /tmf-api/productOrderingManagement/v4/productOrder` - Create product order
- `PATCH /tmf-api/productOrderingManagement/v4/productOrder/{id}` - Update product order
- `DELETE /tmf-api/productOrderingManagement/v4/productOrder/{id}` - Delete product order

#### TMF637 - Product Inventory Management API

**Status:** ✅ Implemented

Manages what products and services customers own. Tracks product instances and their lifecycle states.

**Key Features:**

- Product inventory management
- Product instance tracking
- Status management
- Related party associations

**Endpoints:**

- `GET /tmf-api/productInventoryManagement/v4/productInventory` - List product inventories
- `GET /tmf-api/productInventoryManagement/v4/productInventory/{id}` - Get product inventory by ID
- `POST /tmf-api/productInventoryManagement/v4/productInventory` - Create product inventory
- `PATCH /tmf-api/productInventoryManagement/v4/productInventory/{id}` - Update product inventory
- `DELETE /tmf-api/productInventoryManagement/v4/productInventory/{id}` - Delete product inventory

### Phase 2: Customer & User Domain ✅

#### TMF629 - Customer Management API

**Status:** ✅ Implemented

Manages customer profiles and their contact information. Core for CRM, KYC, account management, and onboarding.

**Key Features:**

- Customer profile management
- Contact information handling
- Account references
- Characteristics and attributes

**Endpoints:**

- `GET /tmf-api/customerManagement/v4/customer` - List customers
- `GET /tmf-api/customerManagement/v4/customer/{id}` - Get customer by ID
- `POST /tmf-api/customerManagement/v4/customer` - Create customer
- `PATCH /tmf-api/customerManagement/v4/customer/{id}` - Update customer
- `DELETE /tmf-api/customerManagement/v4/customer/{id}` - Delete customer

#### TMF678 - Customer Bill Management API

**Status:** ✅ Implemented

Retrieves bills and billing structures. Required for customer portals and self-care.

**Key Features:**

- Bill management
- Bill item tracking
- Payment due dates
- Tax calculations

**Endpoints:**

- `GET /tmf-api/customerBillManagement/v4/customerBill` - List customer bills
- `GET /tmf-api/customerBillManagement/v4/customerBill/{id}` - Get customer bill by ID
- `POST /tmf-api/customerBillManagement/v4/customerBill` - Create customer bill
- `PATCH /tmf-api/customerBillManagement/v4/customerBill/{id}` - Update customer bill
- `DELETE /tmf-api/customerBillManagement/v4/customerBill/{id}` - Delete customer bill

#### TMF679 - Customer Usage Management API

**Status:** ✅ Implemented

Handles CDRs, usage records, and consumption. Essential for 5G, IoT, prepaid, and analytics.

**Key Features:**

- Usage record management
- Consumption tracking
- Product offering references
- Date range queries

**Endpoints:**

- `GET /tmf-api/customerUsageManagement/v4/customerUsage` - List customer usages
- `GET /tmf-api/customerUsageManagement/v4/customerUsage/{id}` - Get customer usage by ID
- `POST /tmf-api/customerUsageManagement/v4/customerUsage` - Create customer usage
- `PATCH /tmf-api/customerUsageManagement/v4/customerUsage/{id}` - Update customer usage
- `DELETE /tmf-api/customerUsageManagement/v4/customerUsage/{id}` - Delete customer usage

#### TMF688 - Appointment Management API

**Status:** ✅ Implemented

For scheduling technician visits, installations, etc. Critical for field operations and order fulfillment.

**Key Features:**

- Appointment scheduling
- Contact medium management
- Duration tracking
- Status management

**Endpoints:**

- `GET /tmf-api/appointmentManagement/v4/appointment` - List appointments
- `GET /tmf-api/appointmentManagement/v4/appointment/{id}` - Get appointment by ID
- `POST /tmf-api/appointmentManagement/v4/appointment` - Create appointment
- `PATCH /tmf-api/appointmentManagement/v4/appointment/{id}` - Update appointment
- `DELETE /tmf-api/appointmentManagement/v4/appointment/{id}` - Delete appointment

### Phase 3: Service Lifecycle (OSS Core) ✅

#### TMF641 - Service Order Management API

**Status:** ✅ Implemented

Equivalent to TMF622 but for network/service-level. A customer order triggers one or more service orders. You cannot complete fulfillment without this.

**Key Features:**

- Service order management
- Service order item tracking
- Service specification references
- Order state management

**Endpoints:**

- `GET /tmf-api/serviceOrderingManagement/v4/serviceOrder` - List service orders
- `GET /tmf-api/serviceOrderingManagement/v4/serviceOrder/{id}` - Get service order by ID
- `POST /tmf-api/serviceOrderingManagement/v4/serviceOrder` - Create service order
- `PATCH /tmf-api/serviceOrderingManagement/v4/serviceOrder/{id}` - Update service order
- `DELETE /tmf-api/serviceOrderingManagement/v4/serviceOrder/{id}` - Delete service order

#### TMF638 - Service Inventory Management API

**Status:** ✅ Implemented

Manages provisioned services and service instances. You need this to track active network-level services.

**Key Features:**

- Service inventory management
- Service instance tracking
- Service specification references
- Status management

**Endpoints:**

- `GET /tmf-api/serviceInventoryManagement/v4/serviceInventory` - List service inventories
- `GET /tmf-api/serviceInventoryManagement/v4/serviceInventory/{id}` - Get service inventory by ID
- `POST /tmf-api/serviceInventoryManagement/v4/serviceInventory` - Create service inventory
- `PATCH /tmf-api/serviceInventoryManagement/v4/serviceInventory/{id}` - Update service inventory
- `DELETE /tmf-api/serviceInventoryManagement/v4/serviceInventory/{id}` - Delete service inventory

#### TMF640 - Service Activation & Configuration API

**Status:** ✅ Implemented

Handles provisioning actions on network elements. Connects BSS → OSS → Network.

**Key Features:**

- Service activation management
- Configuration handling
- Activation date tracking
- Service reference management

**Endpoints:**

- `GET /tmf-api/serviceActivationAndConfiguration/v4/serviceActivation` - List service activations
- `GET /tmf-api/serviceActivationAndConfiguration/v4/serviceActivation/{id}` - Get service activation by ID
- `POST /tmf-api/serviceActivationAndConfiguration/v4/serviceActivation` - Create service activation
- `PATCH /tmf-api/serviceActivationAndConfiguration/v4/serviceActivation/{id}` - Update service activation
- `DELETE /tmf-api/serviceActivationAndConfiguration/v4/serviceActivation/{id}` - Delete service activation

#### TMF702 - Resource Activation & Configuration API

**Status:** ✅ Implemented

Low-level provisioning of physical/virtual network elements. Required for automated orchestration.

**Key Features:**

- Resource activation management
- Resource configuration
- Activation and completion date tracking
- Service activation references

**Endpoints:**

- `GET /tmf-api/resourceActivationAndConfiguration/v4/resourceActivation` - List resource activations
- `GET /tmf-api/resourceActivationAndConfiguration/v4/resourceActivation/{id}` - Get resource activation by ID
- `POST /tmf-api/resourceActivationAndConfiguration/v4/resourceActivation` - Create resource activation
- `PATCH /tmf-api/resourceActivationAndConfiguration/v4/resourceActivation/{id}` - Update resource activation
- `DELETE /tmf-api/resourceActivationAndConfiguration/v4/resourceActivation/{id}` - Delete resource activation

### Phase 4: Resource Domain (Network & Infrastructure) ✅

#### TMF639 - Resource Inventory Management API

**Status:** ✅ Implemented

Tracks physical and virtual network resources. Needed for network planning, provisioning, 5G slicing.

**Key Features:**

- Resource inventory management
- Resource specification references
- Resource references
- Status tracking

**Endpoints:**

- `GET /tmf-api/resourceInventoryManagement/v4/resourceInventory` - List resource inventories
- `GET /tmf-api/resourceInventoryManagement/v4/resourceInventory/{id}` - Get resource inventory by ID
- `POST /tmf-api/resourceInventoryManagement/v4/resourceInventory` - Create resource inventory
- `PATCH /tmf-api/resourceInventoryManagement/v4/resourceInventory/{id}` - Update resource inventory
- `DELETE /tmf-api/resourceInventoryManagement/v4/resourceInventory/{id}` - Delete resource inventory

#### TMF645 - Resource Order Management API

**Status:** ✅ Implemented

Like TMF622 & TMF641, but for network resources. Service activation depends on resource orders.

**Key Features:**

- Resource order management
- Resource order item tracking
- Resource specification references
- Order state management

**Endpoints:**

- `GET /tmf-api/resourceOrderManagement/v4/resourceOrder` - List resource orders
- `GET /tmf-api/resourceOrderManagement/v4/resourceOrder/{id}` - Get resource order by ID
- `POST /tmf-api/resourceOrderManagement/v4/resourceOrder` - Create resource order
- `PATCH /tmf-api/resourceOrderManagement/v4/resourceOrder/{id}` - Update resource order
- `DELETE /tmf-api/resourceOrderManagement/v4/resourceOrder/{id}` - Delete resource order

### Phase 5: Revenue Management (Charging & Billing) ✅

#### TMF635 - Usage Management API

**Status:** ✅ Implemented

Tracks and queries usage (CDRs, event consumption). Needed for OCS/OFCS (5G charging systems).

**Key Features:**

- Usage record management
- Product offering references
- Rating references
- Usage state tracking

**Endpoints:**

- `GET /tmf-api/usageManagement/v4/usage` - List usages
- `GET /tmf-api/usageManagement/v4/usage/{id}` - Get usage by ID
- `POST /tmf-api/usageManagement/v4/usage` - Create usage
- `PATCH /tmf-api/usageManagement/v4/usage/{id}` - Update usage
- `DELETE /tmf-api/usageManagement/v4/usage/{id}` - Delete usage

#### TMF668 - Party Role Management API

**Status:** ✅ Implemented

Manages parties, organizations, roles, partners. Required for partner settlements and ecosystem play.

**Key Features:**

- Party role management
- Contact medium handling
- Role state tracking
- Related party associations

**Endpoints:**

- `GET /tmf-api/partyRoleManagement/v4/partyRole` - List party roles
- `GET /tmf-api/partyRoleManagement/v4/partyRole/{id}` - Get party role by ID
- `POST /tmf-api/partyRoleManagement/v4/partyRole` - Create party role
- `PATCH /tmf-api/partyRoleManagement/v4/partyRole/{id}` - Update party role
- `DELETE /tmf-api/partyRoleManagement/v4/partyRole/{id}` - Delete party role

### Phase 6: Security, Party & Identity ✅

#### TMF632 - Party Management API

**Status:** ✅ Implemented

Manages individuals, organizations, account-level attributes. KYC & business partners depend on it.

**Key Features:**

- Party management
- Party type handling (individual/organization)
- Contact medium management
- Account references
- Characteristics and attributes

**Endpoints:**

- `GET /tmf-api/partyManagement/v4/party` - List parties
- `GET /tmf-api/partyManagement/v4/party/{id}` - Get party by ID
- `POST /tmf-api/partyManagement/v4/party` - Create party
- `PATCH /tmf-api/partyManagement/v4/party/{id}` - Update party
- `DELETE /tmf-api/partyManagement/v4/party/{id}` - Delete party

#### TMF669 - Identity & Credential Management API

**Status:** ✅ Implemented

Handles digital identities, credentials, OAuth/JWT integration. Your system becomes enterprise-ready.

**Key Features:**

- Identity management
- Credential management
- Credential type handling
- Party references
- Identity state tracking

**Endpoints:**

- `GET /tmf-api/identityAndCredentialManagement/v4/identity` - List identities
- `GET /tmf-api/identityAndCredentialManagement/v4/identity/{id}` - Get identity by ID
- `POST /tmf-api/identityAndCredentialManagement/v4/identity` - Create identity
- `PATCH /tmf-api/identityAndCredentialManagement/v4/identity/{id}` - Update identity
- `DELETE /tmf-api/identityAndCredentialManagement/v4/identity/{id}` - Delete identity

## Implementation Summary

### Total Implemented APIs: 17

- **Phase 1 (Product Domain):** 3 APIs (TMF620, TMF622, TMF637)
- **Phase 2 (Customer & User Domain):** 4 APIs (TMF629, TMF678, TMF679, TMF688)
- **Phase 3 (Service Lifecycle):** 4 APIs (TMF641, TMF638, TMF640, TMF702)
- **Phase 4 (Resource Domain):** 2 APIs (TMF639, TMF645)
- **Phase 5 (Revenue Management):** 2 APIs (TMF635, TMF668)
- **Phase 6 (Security, Party & Identity):** 2 APIs (TMF632, TMF669)

## Compliance

This project aims for full compliance with TM Forum Open API specifications. All implementations follow the official TMF API guidelines and patterns, including:

- Standard HTTP methods (GET, POST, PATCH, DELETE)
- Consistent error handling
- JWT-based authentication
- OpenAPI/Swagger documentation
- PostgreSQL data persistence
- Related party associations
- State management
- Lifecycle tracking

## Resources

- [TM Forum Open APIs](https://www.tmforum.org/open-apis/)
- [TMF620 Specification](https://www.tmforum.org/open-apis/)
- [TMF622 Specification](https://www.tmforum.org/open-apis/)
- [TMF637 Specification](https://www.tmforum.org/open-apis/)
- [TMF629 Specification](https://www.tmforum.org/open-apis/)
- [TMF678 Specification](https://www.tmforum.org/open-apis/)
- [TMF679 Specification](https://www.tmforum.org/open-apis/)
- [TMF688 Specification](https://www.tmforum.org/open-apis/)
- [TMF641 Specification](https://www.tmforum.org/open-apis/)
- [TMF638 Specification](https://www.tmforum.org/open-apis/)
- [TMF640 Specification](https://www.tmforum.org/open-apis/)
- [TMF702 Specification](https://www.tmforum.org/open-apis/)
- [TMF639 Specification](https://www.tmforum.org/open-apis/)
- [TMF645 Specification](https://www.tmforum.org/open-apis/)
- [TMF635 Specification](https://www.tmforum.org/open-apis/)
- [TMF668 Specification](https://www.tmforum.org/open-apis/)
- [TMF632 Specification](https://www.tmforum.org/open-apis/)
- [TMF669 Specification](https://www.tmforum.org/open-apis/)
