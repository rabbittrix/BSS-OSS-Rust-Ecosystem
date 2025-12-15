//! Diameter Protocol Support
//!
//! Implements Gx, Gy, and Gz interfaces for PCRF/PCF communication
//!
//! ## Interfaces
//!
//! - **Gx**: Policy and Charging Control (PCC) rules between PCRF and PCEF (Policy and Charging Enforcement Function)
//! - **Gy**: Online charging between PCEF and OCS (Online Charging System)
//! - **Gz**: Offline charging between PCEF and CGF (Charging Gateway Function)

use crate::error::PcfError;
use crate::models::{PolicyDecision, PolicyRequest};
use log::{debug, info};
use serde::{Deserialize, Serialize};

/// Diameter application IDs
pub mod application_ids {
    /// Gx interface application ID
    pub const GX_APPLICATION_ID: u32 = 16777238;
    /// Gy interface application ID
    pub const GY_APPLICATION_ID: u32 = 4;
    /// Gz interface application ID
    pub const GZ_APPLICATION_ID: u32 = 4;
}

/// Diameter command codes
pub mod command_codes {
    /// Credit Control Request (CCR) - used in Gy
    pub const CCR: u32 = 272;
    /// Credit Control Answer (CCA) - used in Gy
    pub const CCA: u32 = 272;
    /// Re-Auth Request (RAR) - used in Gx
    pub const RAR: u32 = 258;
    /// Re-Auth Answer (RAA) - used in Gx
    pub const RAA: u32 = 258;
    /// Credit Control Request Initial (CCR-I)
    pub const CCR_INITIAL: u32 = 1;
    /// Credit Control Request Update (CCR-U)
    pub const CCR_UPDATE: u32 = 2;
    /// Credit Control Request Terminate (CCR-T)
    pub const CCR_TERMINATE: u32 = 3;
}

/// Diameter AVP (Attribute-Value Pair) codes
pub mod avp_codes {
    /// Session-Id
    pub const SESSION_ID: u32 = 263;
    /// Auth-Application-Id
    pub const AUTH_APPLICATION_ID: u32 = 258;
    /// CC-Request-Type
    pub const CC_REQUEST_TYPE: u32 = 416;
    /// CC-Request-Number
    pub const CC_REQUEST_NUMBER: u32 = 415;
    /// Subscription-Id
    pub const SUBSCRIPTION_ID: u32 = 443;
    /// Subscription-Id-Type (IMSI, MSISDN, etc.)
    pub const SUBSCRIPTION_ID_TYPE: u32 = 450;
    /// Subscription-Id-Data
    pub const SUBSCRIPTION_ID_DATA: u32 = 444;
    /// QoS-Information
    pub const QOS_INFORMATION: u32 = 1016;
    /// Charging-Rule-Install
    pub const CHARGING_RULE_INSTALL: u32 = 1001;
    /// Charging-Rule-Remove
    pub const CHARGING_RULE_REMOVE: u32 = 1002;
    /// Used-Service-Unit
    pub const USED_SERVICE_UNIT: u32 = 447;
    /// Granted-Service-Unit
    pub const GRANTED_SERVICE_UNIT: u32 = 446;
    /// CC-Total-Octets
    pub const CC_TOTAL_OCTETS: u32 = 421;
    /// Result-Code
    pub const RESULT_CODE: u32 = 268;
    /// Origin-Host
    pub const ORIGIN_HOST: u32 = 264;
    /// Origin-Realm
    pub const ORIGIN_REALM: u32 = 296;
}

/// Diameter message types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiameterMessageType {
    /// Request message
    Request,
    /// Answer message
    Answer,
}

/// Gx interface message (Policy and Charging Control)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GxMessage {
    /// Session ID
    pub session_id: String,
    /// Subscriber ID (IMSI or MSISDN)
    pub subscriber_id: String,
    /// APN (Access Point Name)
    pub apn: String,
    /// Request type
    pub request_type: GxRequestType,
    /// Policy decision (in responses)
    pub policy_decision: Option<PolicyDecision>,
}

/// Gx request types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GxRequestType {
    /// Initial request
    Initial,
    /// Update request
    Update,
    /// Terminate request
    Terminate,
}

/// Gy interface message (Online Charging)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GyMessage {
    /// Session ID
    pub session_id: String,
    /// Subscriber ID
    pub subscriber_id: String,
    /// Request type
    pub request_type: GyRequestType,
    /// Used service units (bytes, time, events)
    pub used_service_units: Option<ServiceUnits>,
    /// Requested service units
    pub requested_service_units: Option<ServiceUnits>,
    /// Granted service units (in response)
    pub granted_service_units: Option<ServiceUnits>,
    /// Result code
    pub result_code: Option<u32>,
}

/// Gy request types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GyRequestType {
    /// Initial request
    Initial,
    /// Update request
    Update,
    /// Terminate request
    Terminate,
    /// Event request
    Event,
}

/// Service units (for charging)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceUnits {
    /// Total octets (bytes)
    pub total_octets: Option<u64>,
    /// Input octets (upload)
    pub input_octets: Option<u64>,
    /// Output octets (download)
    pub output_octets: Option<u64>,
    /// Time (seconds)
    pub time: Option<u64>,
    /// Events
    pub events: Option<u32>,
}

/// Gz interface message (Offline Charging)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GzMessage {
    /// Session ID
    pub session_id: String,
    /// Subscriber ID
    pub subscriber_id: String,
    /// Record type
    pub record_type: GzRecordType,
    /// Service units used
    pub service_units: ServiceUnits,
    /// Timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Gz record types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GzRecordType {
    /// Start record
    Start,
    /// Interim record
    Interim,
    /// Stop record
    Stop,
    /// Event record
    Event,
}

/// Diameter protocol handler
pub struct DiameterHandler {
    /// PCF engine reference
    pcf_engine: Option<std::sync::Arc<dyn crate::pcf_engine::PcfEngineTrait>>,
}

impl DiameterHandler {
    /// Create a new Diameter handler
    pub fn new() -> Self {
        Self { pcf_engine: None }
    }

    /// Set PCF engine reference
    pub fn set_pcf_engine(
        &mut self,
        engine: std::sync::Arc<dyn crate::pcf_engine::PcfEngineTrait>,
    ) {
        self.pcf_engine = Some(engine);
    }

    /// Handle Gx request (Policy and Charging Control)
    pub async fn handle_gx_request(&self, message: &GxMessage) -> Result<GxMessage, PcfError> {
        info!(
            "Handling Gx request: session={}, subscriber={}, type={:?}",
            message.session_id, message.subscriber_id, message.request_type
        );

        // Convert Gx message to PolicyRequest
        #[allow(deprecated)]
        let policy_request = PolicyRequest {
            subscriber_id: message.subscriber_id.clone(),
            imsi: message.subscriber_id.clone(), // In production, would extract from Diameter AVPs
            tax_id: Some(
                crate::tax_id::TaxId::from_string(
                    "123.456.789-09",
                    crate::tax_id::TaxIdCountry::BR,
                )
                .unwrap(),
            ),
            cpf: None, // Deprecated, kept for backward compatibility
            network_generation: crate::models::NetworkGeneration::FourG, // Default, would come from AVPs
            apn: message.apn.clone(),
            service_type: "default".to_string(), // Would come from AVPs
            application_id: None,
            location: None,
            time_of_day: None,
        };

        // Evaluate policy using PCF engine
        if let Some(ref engine) = self.pcf_engine {
            let policy_decision = engine.evaluate_policy(&policy_request).await?;

            let response = GxMessage {
                session_id: message.session_id.clone(),
                subscriber_id: message.subscriber_id.clone(),
                apn: message.apn.clone(),
                request_type: message.request_type,
                policy_decision: Some(policy_decision),
            };

            debug!("Gx response generated for session: {}", message.session_id);
            return Ok(response);
        }

        Err(PcfError::ServiceUnavailable(
            "PCF engine not configured".to_string(),
        ))
    }

    /// Handle Gy request (Online Charging)
    pub async fn handle_gy_request(&self, message: &GyMessage) -> Result<GyMessage, PcfError> {
        info!(
            "Handling Gy request: session={}, subscriber={}, type={:?}",
            message.session_id, message.subscriber_id, message.request_type
        );

        // In production, this would:
        // 1. Check subscriber balance (prepaid)
        // 2. Reserve quota
        // 3. Return granted service units

        let granted_units = ServiceUnits {
            total_octets: Some(100_000_000), // 100 MB granted
            input_octets: Some(50_000_000),
            output_octets: Some(50_000_000),
            time: Some(3600), // 1 hour
            events: None,
        };

        let response = GyMessage {
            session_id: message.session_id.clone(),
            subscriber_id: message.subscriber_id.clone(),
            request_type: message.request_type,
            used_service_units: message.used_service_units.clone(),
            requested_service_units: message.requested_service_units.clone(),
            granted_service_units: Some(granted_units),
            result_code: Some(2001), // DIAMETER_SUCCESS
        };

        debug!("Gy response generated for session: {}", message.session_id);
        Ok(response)
    }

    /// Handle Gz message (Offline Charging)
    pub async fn handle_gz_message(&self, message: &GzMessage) -> Result<(), PcfError> {
        info!(
            "Handling Gz message: session={}, subscriber={}, type={:?}",
            message.session_id, message.subscriber_id, message.record_type
        );

        // In production, this would:
        // 1. Store CDR (Call Detail Record)
        // 2. Forward to billing system
        // 3. Update usage statistics

        debug!("Gz message processed for session: {}", message.session_id);
        Ok(())
    }
}

impl Default for DiameterHandler {
    fn default() -> Self {
        Self::new()
    }
}

/// Diameter result codes
pub mod result_codes {
    /// Success
    pub const DIAMETER_SUCCESS: u32 = 2001;
    /// Command unsupported
    pub const DIAMETER_COMMAND_UNSUPPORTED: u32 = 3001;
    /// Unable to deliver
    pub const DIAMETER_UNABLE_TO_DELIVER: u32 = 3002;
    /// Realms in transaction do not match
    pub const DIAMETER_REALM_NOT_SERVED: u32 = 3003;
    /// Too busy
    pub const DIAMETER_TOO_BUSY: u32 = 3004;
    /// Loop detected
    pub const DIAMETER_LOOP_DETECTED: u32 = 3005;
    /// Redirect indication
    pub const DIAMETER_REDIRECT_INDICATION: u32 = 3006;
    /// Application unsupported
    pub const DIAMETER_APPLICATION_UNSUPPORTED: u32 = 3007;
    /// Invalid HDR bits
    pub const DIAMETER_INVALID_HDR_BITS: u32 = 3008;
    /// Invalid AVP length
    pub const DIAMETER_INVALID_AVP_LENGTH: u32 = 3009;
    /// Invalid message length
    pub const DIAMETER_INVALID_MESSAGE_LENGTH: u32 = 3010;
    /// Invalid AVP bit combo
    pub const DIAMETER_INVALID_AVP_BIT_COMBO: u32 = 3011;
    /// Invalid AVP value
    pub const DIAMETER_INVALID_AVP_VALUE: u32 = 3012;
    /// Missing AVP
    pub const DIAMETER_MISSING_AVP: u32 = 3013;
    /// Resource exhaustion
    pub const DIAMETER_RESOURCES_EXCEEDED: u32 = 5004;
    /// Authentication rejected
    pub const DIAMETER_AUTHENTICATION_REJECTED: u32 = 4001;
    /// Out of space
    pub const DIAMETER_OUT_OF_SPACE: u32 = 5002;
    /// Selection of destination failed
    pub const DIAMETER_ELECTION_LOST: u32 = 4003;
    /// User unknown
    pub const DIAMETER_USER_UNKNOWN: u32 = 5001;
    /// Rating failed
    pub const DIAMETER_RATING_FAILED: u32 = 5031;
    /// Credit limit reached
    pub const DIAMETER_CREDIT_LIMIT_REACHED: u32 = 4012;
}
