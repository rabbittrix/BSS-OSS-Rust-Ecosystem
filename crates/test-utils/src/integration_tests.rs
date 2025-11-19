//! Integration test helpers for TMF APIs

use crate::fixtures;
use crate::helpers;
use actix_web::{test, App};
use sqlx::PgPool;
use uuid::Uuid;

/// Test configuration for integration tests
pub struct TestConfig {
    pub pool: PgPool,
    pub base_url: String,
}

/// Setup test application with database
pub async fn setup_test_app(
    pool: PgPool,
) -> App<
    impl actix_web::dev::ServiceFactory<
            actix_web::dev::ServiceRequest,
            Config = (),
            Response = actix_web::dev::ServiceResponse<actix_web::body::BoxBody>,
            Error = actix_web::Error,
            InitError = (),
        > + 'static,
> {
    // This would be set up with actual route configuration
    // For now, return a basic app structure
    App::new().app_data(actix_web::web::Data::new(pool))
}

/// Test helper for TMF629 - Customer Management API
pub mod tmf629_tests {
    use super::*;

    /// Test creating a customer
    pub async fn test_create_customer(
        app: App<
            impl actix_web::dev::ServiceFactory<
                    actix_web::dev::ServiceRequest,
                    Config = (),
                    Response = actix_web::dev::ServiceResponse<actix_web::body::BoxBody>,
                    Error = actix_web::Error,
                    InitError = (),
                > + 'static,
        >,
    ) -> Result<Uuid, Box<dyn std::error::Error>> {
        let customer_json = fixtures::create_test_customer_json();
        let req = helpers::create_test_request(
            "POST",
            "/tmf-api/customerManagement/v4/customer",
            Some(&customer_json.to_string()),
        );

        let service = test::init_service(app).await;
        let resp = test::call_service(&service, req.to_request()).await;
        assert!(resp.status().is_success());

        let body = test::read_body(resp).await;
        let json: serde_json::Value = serde_json::from_slice(&body)?;

        // Extract customer ID from response
        let id_str = json["id"]
            .as_str()
            .ok_or("Missing customer ID in response")?;
        Ok(Uuid::parse_str(id_str)?)
    }

    /// Test getting a customer by ID
    pub async fn test_get_customer(
        app: App<
            impl actix_web::dev::ServiceFactory<
                    actix_web::dev::ServiceRequest,
                    Config = (),
                    Response = actix_web::dev::ServiceResponse<actix_web::body::BoxBody>,
                    Error = actix_web::Error,
                    InitError = (),
                > + 'static,
        >,
        customer_id: Uuid,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let req = helpers::create_test_request(
            "GET",
            &format!("/tmf-api/customerManagement/v4/customer/{}", customer_id),
            None,
        );

        let service = test::init_service(app).await;
        let resp = test::call_service(&service, req.to_request()).await;
        assert!(resp.status().is_success());

        let body = test::read_body(resp).await;
        let json: serde_json::Value = serde_json::from_slice(&body)?;
        assert_eq!(json["id"].as_str().unwrap(), customer_id.to_string());
        Ok(())
    }

    /// Test listing customers
    pub async fn test_list_customers(
        app: App<
            impl actix_web::dev::ServiceFactory<
                    actix_web::dev::ServiceRequest,
                    Config = (),
                    Response = actix_web::dev::ServiceResponse<actix_web::body::BoxBody>,
                    Error = actix_web::Error,
                    InitError = (),
                > + 'static,
        >,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let req =
            helpers::create_test_request("GET", "/tmf-api/customerManagement/v4/customer", None);

        let service = test::init_service(app).await;
        let resp = test::call_service(&service, req.to_request()).await;
        assert!(resp.status().is_success());

        let body = test::read_body(resp).await;
        let json: serde_json::Value = serde_json::from_slice(&body)?;
        assert!(json.is_array() || json["data"].is_array());
        Ok(())
    }
}

/// Test helper for TMF678 - Customer Bill Management API
pub mod tmf678_tests {
    use super::*;

    /// Test creating a bill
    pub async fn test_create_bill(
        app: App<
            impl actix_web::dev::ServiceFactory<
                    actix_web::dev::ServiceRequest,
                    Config = (),
                    Response = actix_web::dev::ServiceResponse<actix_web::body::BoxBody>,
                    Error = actix_web::Error,
                    InitError = (),
                > + 'static,
        >,
        customer_id: Uuid,
    ) -> Result<Uuid, Box<dyn std::error::Error>> {
        let bill_json = fixtures::create_test_bill_json(customer_id);
        let req = helpers::create_test_request(
            "POST",
            "/tmf-api/customerBillManagement/v4/bill",
            Some(&bill_json.to_string()),
        );

        let service = test::init_service(app).await;
        let resp = test::call_service(&service, req.to_request()).await;
        assert!(resp.status().is_success());

        let body = test::read_body(resp).await;
        let json: serde_json::Value = serde_json::from_slice(&body)?;

        let id_str = json["id"].as_str().ok_or("Missing bill ID in response")?;
        Ok(Uuid::parse_str(id_str)?)
    }

    /// Test getting bills for a customer
    pub async fn test_get_customer_bills(
        app: App<
            impl actix_web::dev::ServiceFactory<
                    actix_web::dev::ServiceRequest,
                    Config = (),
                    Response = actix_web::dev::ServiceResponse<actix_web::body::BoxBody>,
                    Error = actix_web::Error,
                    InitError = (),
                > + 'static,
        >,
        customer_id: Uuid,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let req = helpers::create_test_request(
            "GET",
            &format!(
                "/tmf-api/customerBillManagement/v4/bill?relatedParty.id={}",
                customer_id
            ),
            None,
        );

        let service = test::init_service(app).await;
        let resp = test::call_service(&service, req.to_request()).await;
        assert!(resp.status().is_success());
        Ok(())
    }
}

/// Test helper for TMF679 - Customer Usage Management API
pub mod tmf679_tests {
    use super::*;

    /// Test creating a usage record
    pub async fn test_create_usage_record(
        app: App<
            impl actix_web::dev::ServiceFactory<
                    actix_web::dev::ServiceRequest,
                    Config = (),
                    Response = actix_web::dev::ServiceResponse<actix_web::body::BoxBody>,
                    Error = actix_web::Error,
                    InitError = (),
                > + 'static,
        >,
    ) -> Result<Uuid, Box<dyn std::error::Error>> {
        let usage_json = fixtures::create_test_usage_record_json();
        let req = helpers::create_test_request(
            "POST",
            "/tmf-api/customerUsageManagement/v4/usage",
            Some(&usage_json.to_string()),
        );

        let service = test::init_service(app).await;
        let resp = test::call_service(&service, req.to_request()).await;
        assert!(resp.status().is_success());

        let body = test::read_body(resp).await;
        let json: serde_json::Value = serde_json::from_slice(&body)?;

        let id_str = json["id"]
            .as_str()
            .ok_or("Missing usage record ID in response")?;
        Ok(Uuid::parse_str(id_str)?)
    }
}

/// Test helper for TMF688 - Appointment Management API
pub mod tmf688_tests {
    use super::*;

    /// Test creating an appointment
    pub async fn test_create_appointment(
        app: App<
            impl actix_web::dev::ServiceFactory<
                    actix_web::dev::ServiceRequest,
                    Config = (),
                    Response = actix_web::dev::ServiceResponse<actix_web::body::BoxBody>,
                    Error = actix_web::Error,
                    InitError = (),
                > + 'static,
        >,
    ) -> Result<Uuid, Box<dyn std::error::Error>> {
        use chrono::Utc;
        use serde_json::json;

        let appointment_json = json!({
            "name": "Test Appointment",
            "description": "Test appointment for unit tests",
            "status": "CONFIRMED",
            "appointmentDate": Utc::now().to_rfc3339(),
            "relatedParty": [
                {
                    "id": Uuid::new_v4().to_string(),
                    "role": "customer"
                }
            ]
        });

        let req = helpers::create_test_request(
            "POST",
            "/tmf-api/appointmentManagement/v4/appointment",
            Some(&appointment_json.to_string()),
        );

        let service = test::init_service(app).await;
        let resp = test::call_service(&service, req.to_request()).await;
        assert!(resp.status().is_success());

        let body = test::read_body(resp).await;
        let json: serde_json::Value = serde_json::from_slice(&body)?;

        let id_str = json["id"]
            .as_str()
            .ok_or("Missing appointment ID in response")?;
        Ok(Uuid::parse_str(id_str)?)
    }
}
