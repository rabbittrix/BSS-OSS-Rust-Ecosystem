//! SDK Generator

use crate::models::{SdkConfig, SdkPlatform};

/// SDK Generator
pub struct SdkGenerator;

impl SdkGenerator {
    /// Generate SDK configuration
    pub fn generate_config(
        platform: SdkPlatform,
        api_base_url: String,
        package_name: String,
        version: String,
    ) -> SdkConfig {
        let features = match platform {
            SdkPlatform::Ios => vec![
                "Swift".to_string(),
                "Combine".to_string(),
                "Async/Await".to_string(),
            ],
            SdkPlatform::Android => vec![
                "Kotlin".to_string(),
                "Coroutines".to_string(),
                "Retrofit".to_string(),
            ],
            SdkPlatform::Flutter => vec![
                "Dart".to_string(),
                "HTTP".to_string(),
                "Provider".to_string(),
            ],
            SdkPlatform::ReactNative => vec![
                "TypeScript".to_string(),
                "Axios".to_string(),
                "React Hooks".to_string(),
            ],
        };

        SdkConfig {
            platform,
            api_base_url,
            package_name,
            version,
            features,
        }
    }

    /// Generate SDK documentation
    pub fn generate_documentation(config: &SdkConfig) -> String {
        format!(
            r#"
# {} SDK v{}

## Installation

{}

## Configuration

```{}
{}
```

## Features

{}

## API Reference

Base URL: {}

## License

MIT
"#,
            config.package_name,
            config.version,
            Self::get_installation_instructions(config.platform),
            Self::get_config_language(config.platform),
            Self::get_config_example(config),
            Self::format_features(&config.features),
            config.api_base_url
        )
    }

    fn get_installation_instructions(platform: SdkPlatform) -> &'static str {
        match platform {
            SdkPlatform::Ios => "Add to your Podfile:\n```ruby\npod 'BSSOSSRust', '~> 0.3.0'\n```",
            SdkPlatform::Android => {
                "Add to your build.gradle:\n```gradle\nimplementation 'com.bssoss:rust:0.3.0'\n```"
            }
            SdkPlatform::Flutter => {
                "Add to your pubspec.yaml:\n```yaml\ndependencies:\n  bss_oss_rust: ^0.3.0\n```"
            }
            SdkPlatform::ReactNative => "```bash\nnpm install @bssoss/rust-sdk\n```",
        }
    }

    fn get_config_language(platform: SdkPlatform) -> &'static str {
        match platform {
            SdkPlatform::Ios => "swift",
            SdkPlatform::Android => "kotlin",
            SdkPlatform::Flutter => "dart",
            SdkPlatform::ReactNative => "typescript",
        }
    }

    fn get_config_example(config: &SdkConfig) -> String {
        match config.platform {
            SdkPlatform::Ios => {
                format!(
                    "let config = ApiConfig(baseURL: URL(string: \"{}\")!)\nlet client = ApiClient(config: config)",
                    config.api_base_url
                )
            }
            SdkPlatform::Android => {
                format!(
                    "val config = ApiConfig(baseUrl = \"{}\")\nval client = ApiClient(config)",
                    config.api_base_url
                )
            }
            SdkPlatform::Flutter => {
                format!(
                    "final config = ApiConfig(baseUrl: '{}');\nfinal client = ApiClient(config);",
                    config.api_base_url
                )
            }
            SdkPlatform::ReactNative => {
                format!(
                    "const config = {{ baseUrl: '{}' }};\nconst client = new ApiClient(config);",
                    config.api_base_url
                )
            }
        }
    }

    fn format_features(features: &[String]) -> String {
        features
            .iter()
            .map(|f| format!("- {}", f))
            .collect::<Vec<_>>()
            .join("\n")
    }
}
