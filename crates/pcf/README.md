# Policy Control Function (PCF/PCRF)

A high-performance Policy Control Function (PCF), formerly known as PCRF (Policy and Charging Rules Function), for 3G, 4G, 5G, and 6G mobile telecommunications networks.

## Overview

The PCF is the "brain" of policies in mobile telecommunications networks. It makes real-time decisions about:

- **Quality of Service (QoS)**: Bandwidth limits, prioritization, and traffic gating
- **Charging Rules**: Online (prepaid) and offline (postpaid) charging
- **Quota Management**: Data allowance tracking, throttling, and notifications

## Key Features

### 🎯 Policy Control

- Real-time QoS decisions based on subscriber plans and service types
- Bandwidth management (download/upload limits)
- Traffic prioritization (VoLTE, video streaming, gaming, etc.)
- Service gating (blocking/allowlisting)

### 💰 Charging Rules

- Online charging (prepaid) with real-time balance checks
- Offline charging (postpaid) for billing records
- Zero-rating support (unlimited apps/services)
- Service-specific charging rules

### 📊 Quota Management

- Real-time quota tracking
- Automatic throttling when quota is exceeded
- Threshold-based notifications (e.g., 80% usage warning)
- Monthly quota resets

### 📡 Diameter Protocol Support

- **Gx Interface**: Policy and Charging Control (PCC) rules
- **Gy Interface**: Online charging with OCS
- **Gz Interface**: Offline charging with CGF

### 🤖 AI/ML Integration Hooks

- QoS prediction based on historical data
- Network congestion prediction
- Policy optimization using ML
- Anomaly detection for fraud prevention

### 🌐 Network Generation Support

- **3G** (UMTS/HSPA): Up to 42 Mbps
- **4G** (LTE): Up to 1 Gbps
- **5G** (NR): Up to 20 Gbps
- **6G** (Future): Up to 1 Tbps (theoretical)

## Usage

### Basic Example

```rust
use bss_oss_pcf::{PcfEngine, PolicyRequest, NetworkGeneration};
use chrono::Utc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create PCF engine
    let pcf = PcfEngine::new();

    // Create policy request
    let request = PolicyRequest {
        subscriber_id: "1234567890".to_string(),
        imsi: "123456789012345".to_string(),
        network_generation: NetworkGeneration::FiveG,
        apn: "internet".to_string(),
        service_type: "video_streaming".to_string(),
        application_id: Some("youtube.com".to_string()),
        location: None,
        time_of_day: Some(Utc::now()),
    };

    // Evaluate policy
    let decision = pcf.evaluate_policy(&request).await?;

    println!("Access granted: {}", decision.access_granted);
    println!("QoS Priority: {}", decision.qos.priority);
    println!("Max Download: {} Kbps", decision.qos.max_download_bandwidth_kbps);
    println!("Zero-rated: {}", decision.charging_rules.iter().any(|r| r.zero_rating));

    Ok(())
}
```

### Registering a Subscriber

```rust
use bss_oss_pcf::{PcfEngine, SubscriberProfile, NetworkGeneration, Quota};
use chrono::Utc;

let pcf = PcfEngine::new();

let profile = SubscriberProfile {
    subscriber_id: "1234567890".to_string(),
    imsi: "123456789012345".to_string(),
    plan_name: "Premium Unlimited".to_string(),
    plan_type: "postpaid".to_string(),
    quota: Quota {
        total_quota_bytes: 100_000_000_000, // 100 GB
        used_quota_bytes: 0,
        remaining_quota_bytes: 100_000_000_000,
        notification_threshold_percent: 80,
        exceeded: false,
        throttled_bandwidth_kbps: None,
        last_update: Utc::now(),
    },
    active_policies: vec!["premium_qos".to_string()],
    zero_rated_services: vec!["whatsapp.com".to_string()],
    supported_networks: vec![NetworkGeneration::FourG, NetworkGeneration::FiveG],
    last_update: Utc::now(),
};

pcf.register_subscriber(profile);
```

### Updating Quota Usage

```rust
// Update quota after data usage
pcf.update_quota_usage("1234567890", 1_000_000_000).await?; // 1 GB used

// Get updated quota
let profile = pcf.get_subscriber_profile("1234567890").await?;
println!("Quota used: {}%", profile.quota.usage_percent());
```

### Diameter Protocol Usage

```rust
use bss_oss_pcf::diameter::{DiameterHandler, GxMessage, GxRequestType};

let mut handler = DiameterHandler::new();
handler.set_pcf_engine(Arc::new(pcf));

let gx_request = GxMessage {
    session_id: "session-123".to_string(),
    subscriber_id: "1234567890".to_string(),
    apn: "internet".to_string(),
    request_type: GxRequestType::Initial,
    policy_decision: None,
};

let response = handler.handle_gx_request(&gx_request).await?;
```

## Architecture

```text
┌─────────────────────────────────────────────────────────┐
│                    PCF Engine                            │
├─────────────────────────────────────────────────────────┤
│                                                           │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  │
│  │   Policy     │  │  Charging    │  │    Quota     │  │
│  │   Control    │  │   Rules      │  │  Management  │  │
│  └──────────────┘  └──────────────┘  └──────────────┘  │
│                                                           │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  │
│  │   Diameter   │  │      AI      │  │  Subscriber   │  │
│  │   Protocol   │  │  Integration │  │   Profiles    │  │
│  └──────────────┘  └──────────────┘  └──────────────┘  │
│                                                           │
└─────────────────────────────────────────────────────────┘
         │              │              │
         ▼              ▼              ▼
    ┌────────┐    ┌────────┐    ┌────────┐
    │  P-GW   │    │  OCS   │    │  CGF   │
    │  (Gx)   │    │  (Gy)  │    │  (Gz)  │
    └────────┘    └────────┘    └────────┘
```

## Service Types

The PCF supports different QoS profiles for various service types:

- **VoIP/VoLTE**: Highest priority (QCI 1), low latency, guaranteed bit rate
- **Video Streaming**: High priority (QCI 6), guaranteed bit rate for HD video
- **Gaming**: High priority (QCI 3), low latency, guaranteed bit rate
- **File Download**: Best effort (QCI 9), lower priority
- **Web Browsing**: Best effort (QCI 9), standard priority

## Zero-Rating

Zero-rating allows certain services to not count against data quotas:

```rust
use bss_oss_pcf::models::ZeroRatingRule;
use uuid::Uuid;

let rule = ZeroRatingRule {
    rule_id: Uuid::new_v4(),
    service_identifier: "whatsapp.com".to_string(),
    plan_name: None, // Applies to all plans
    active: true,
};

pcf.charging_rules.add_zero_rating_rule(rule);
```

## AI Integration

The PCF provides hooks for AI/ML integration:

- **QoS Prediction**: Predict optimal QoS based on historical patterns
- **Congestion Prediction**: Forecast network congestion
- **Policy Optimization**: Optimize policies using reinforcement learning
- **Anomaly Detection**: Detect fraud and unusual usage patterns

See the `ai` module for more details.

## Performance

- **Latency**: Sub-millisecond policy decisions
- **Throughput**: Handles millions of requests per second
- **Memory**: Efficient caching with DashMap
- **Scalability**: Designed for horizontal scaling

## License

MIT License - see LICENSE file for details.

## Contributing

Contributions are welcome! Please see the main project CONTRIBUTING.md for guidelines.

## Related Crates

- `bss-oss-policy-engine`: General policy engine for BSS/OSS
- `revenue-management`: Charging and billing integration
- `bss-oss-event-bus`: Event-driven architecture support
