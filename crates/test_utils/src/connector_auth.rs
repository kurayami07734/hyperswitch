use std::{collections::HashMap, env};

use masking::{PeekInterface, Secret};
use router::types::ConnectorAuthType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConnectorAuthentication {
    pub aci: Option<BodyKey>,
    pub adyen: Option<BodyKey>,
    pub adyen_uk: Option<BodyKey>,
    pub airwallex: Option<BodyKey>,
    pub authorizedotnet: Option<BodyKey>,
    pub bambora: Option<BodyKey>,
    pub bitpay: Option<HeaderKey>,
    pub bluesnap: Option<BodyKey>,
    pub cashtocode: Option<BodyKey>,
    pub checkout: Option<SignatureKey>,
    pub coinbase: Option<HeaderKey>,
    pub cryptopay: Option<BodyKey>,
    pub cybersource: Option<SignatureKey>,
    pub dlocal: Option<SignatureKey>,
    #[cfg(feature = "dummy_connector")]
    pub dummyconnector: Option<HeaderKey>,
    pub fiserv: Option<SignatureKey>,
    pub forte: Option<MultiAuthKey>,
    pub globalpay: Option<BodyKey>,
    pub globepay: Option<BodyKey>,
    pub iatapay: Option<SignatureKey>,
    pub mollie: Option<BodyKey>,
    pub multisafepay: Option<HeaderKey>,
    pub nexinets: Option<BodyKey>,
    pub noon: Option<SignatureKey>,
    pub nmi: Option<HeaderKey>,
    pub nuvei: Option<SignatureKey>,
    pub opayo: Option<HeaderKey>,
    pub opennode: Option<HeaderKey>,
    pub payeezy: Option<SignatureKey>,
    pub payme: Option<BodyKey>,
    pub paypal: Option<BodyKey>,
    pub payu: Option<BodyKey>,
    pub powertranz: Option<BodyKey>,
    pub rapyd: Option<BodyKey>,
    pub shift4: Option<HeaderKey>,
    pub stripe: Option<HeaderKey>,
    pub stripe_au: Option<HeaderKey>,
    pub stripe_uk: Option<HeaderKey>,
    pub trustpay: Option<SignatureKey>,
    pub tsys: Option<SignatureKey>,
    pub worldpay: Option<BodyKey>,
    pub worldline: Option<SignatureKey>,
    pub zen: Option<HeaderKey>,
    pub automation_configs: Option<AutomationConfigs>,
}

impl Default for ConnectorAuthentication {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(dead_code)]
impl ConnectorAuthentication {
    #[allow(clippy::expect_used)]
    pub fn new() -> Self {
        // Do `export CONNECTOR_AUTH_FILE_PATH="/hyperswitch/crates/router/tests/connectors/sample_auth.toml"`
        // before running tests in shell
        let path = env::var("CONNECTOR_AUTH_FILE_PATH")
            .expect("Connector authentication file path not set");
        toml::from_str(
            &std::fs::read_to_string(path).expect("connector authentication config file not found"),
        )
        .expect("Failed to read connector authentication config file")
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct ConnectorAuthenticationMap(HashMap<String, ConnectorAuthType>);

impl Default for ConnectorAuthenticationMap {
    fn default() -> Self {
        Self::new()
    }
}

// This is a temporary solution to avoid rust compiler from complaining about unused function
#[allow(dead_code)]
impl ConnectorAuthenticationMap {
    pub fn inner(&self) -> &HashMap<String, ConnectorAuthType> {
        &self.0
    }

    #[allow(clippy::expect_used)]
    pub fn new() -> Self {
        // Do `export CONNECTOR_AUTH_FILE_PATH="/hyperswitch/crates/router/tests/connectors/sample_auth.toml"`
        // before running tests in shell
        let path = env::var("CONNECTOR_AUTH_FILE_PATH")
            .expect("connector authentication file path not set");

        // Read the file contents to a JsonString
        let contents =
            &std::fs::read_to_string(path).expect("Failed to read connector authentication file");

        // Deserialize the JsonString to a HashMap
        let auth_config: HashMap<String, toml::Value> =
            toml::from_str(contents).expect("Failed to deserialize TOML file");

        // auth_config contains the data in below given format:
        // {
        //  "connector_name": Table(
        //      {
        //          "api_key": String(
        //                 "API_Key",
        //          ),
        //          "api_secret": String(
        //              "Secret key",
        //          ),
        //          "key1": String(
        //                  "key1",
        //          ),
        //          "key2": String(
        //              "key2",
        //          ),
        //      },
        //  ),
        // "connector_name": Table(
        //  ...
        // }

        // auth_map refines and extracts required information
        let auth_map = auth_config
            .into_iter()
            .map(|(connector_name, config)| {
                let auth_type = match config {
                    toml::Value::Table(table) => {
                        match (
                            table.get("api_key"),
                            table.get("key1"),
                            table.get("api_secret"),
                            table.get("key2"),
                        ) {
                            (Some(api_key), None, None, None) => ConnectorAuthType::HeaderKey {
                                api_key: api_key.as_str().unwrap_or_default().to_string(),
                            },
                            (Some(api_key), Some(key1), None, None) => ConnectorAuthType::BodyKey {
                                api_key: api_key.as_str().unwrap_or_default().to_string(),
                                key1: key1.as_str().unwrap_or_default().to_string(),
                            },
                            (Some(api_key), Some(key1), Some(api_secret), None) => {
                                ConnectorAuthType::SignatureKey {
                                    api_key: api_key.as_str().unwrap_or_default().to_string(),
                                    key1: key1.as_str().unwrap_or_default().to_string(),
                                    api_secret: api_secret.as_str().unwrap_or_default().to_string(),
                                }
                            }
                            (Some(api_key), Some(key1), Some(api_secret), Some(key2)) => {
                                ConnectorAuthType::MultiAuthKey {
                                    api_key: api_key.as_str().unwrap_or_default().to_string(),
                                    key1: key1.as_str().unwrap_or_default().to_string(),
                                    api_secret: api_secret.as_str().unwrap_or_default().to_string(),
                                    key2: key2.as_str().unwrap_or_default().to_string(),
                                }
                            }
                            _ => ConnectorAuthType::NoKey,
                        }
                    }
                    _ => ConnectorAuthType::NoKey,
                };
                (connector_name, auth_type)
            })
            .collect();

        Self(auth_map)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HeaderKey {
    pub api_key: Secret<String>,
}

impl From<HeaderKey> for ConnectorAuthType {
    fn from(key: HeaderKey) -> Self {
        Self::HeaderKey {
            api_key: key.api_key.peek().to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BodyKey {
    pub api_key: Secret<String>,
    pub key1: Secret<String>,
}

impl From<BodyKey> for ConnectorAuthType {
    fn from(key: BodyKey) -> Self {
        Self::BodyKey {
            api_key: key.api_key.peek().to_string(),
            key1: key.key1.peek().to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SignatureKey {
    pub api_key: Secret<String>,
    pub key1: Secret<String>,
    pub api_secret: Secret<String>,
}

impl From<SignatureKey> for ConnectorAuthType {
    fn from(key: SignatureKey) -> Self {
        Self::SignatureKey {
            api_key: key.api_key.peek().to_string(),
            key1: key.key1.peek().to_string(),
            api_secret: key.api_secret.peek().to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MultiAuthKey {
    pub api_key: Secret<String>,
    pub key1: Secret<String>,
    pub api_secret: Secret<String>,
    pub key2: Secret<String>,
}

impl From<MultiAuthKey> for ConnectorAuthType {
    fn from(key: MultiAuthKey) -> Self {
        Self::MultiAuthKey {
            api_key: key.api_key.peek().to_string(),
            key1: key.key1.peek().to_string(),
            api_secret: key.api_secret.peek().to_string(),
            key2: key.key2.peek().to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AutomationConfigs {
    pub hs_base_url: Option<String>,
    pub hs_api_key: Option<String>,
    pub hs_test_browser: Option<String>,
    pub chrome_profile_path: Option<String>,
    pub firefox_profile_path: Option<String>,
    pub pypl_email: Option<String>,
    pub pypl_pass: Option<String>,
    pub gmail_email: Option<String>,
    pub gmail_pass: Option<String>,
    pub configs_url: Option<String>,
    pub stripe_pub_key: Option<String>,
    pub testcases_path: Option<String>,
    pub bluesnap_gateway_merchant_id: Option<String>,
    pub globalpay_gateway_merchant_id: Option<String>,
    pub run_minimum_steps: Option<bool>,
    pub airwallex_merchant_name: Option<String>,
}