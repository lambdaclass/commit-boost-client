///////////////////////// COMMON /////////////////////////

/// Path to the main toml config file
pub const CONFIG_ENV: &str = "CB_CONFIG";
pub const CONFIG_DEFAULT: &str = "/cb-config.toml";

/// Path to the chain spec file
pub const CHAIN_SPEC_ENV: &str = "CB_CHAIN_SPEC";

/// Where to receive scrape requests from Prometheus
pub const METRICS_PORT_ENV: &str = "CB_METRICS_PORT";

/// Path to logs directory
pub const LOGS_DIR_ENV: &str = "CB_LOGS_DIR";
pub const LOGS_DIR_DEFAULT: &str = "/var/logs/commit-boost";

///////////////////////// PBS /////////////////////////

pub const PBS_IMAGE_DEFAULT: &str = "ghcr.io/commit-boost/pbs:latest";
pub const PBS_MODULE_NAME: &str = "pbs";

/// Urls the pbs modules should post events to (comma separated)
pub const BUILDER_URLS_ENV: &str = "CB_BUILDER_URLS";

/// Where to receive BuilderAPI calls from beacon node
pub const PBS_ENDPOINT_ENV: &str = "CB_PBS_ENDPOINT";

///////////////////////// SIGNER /////////////////////////

pub const SIGNER_IMAGE_DEFAULT: &str = "ghcr.io/commit-boost/signer:latest";
pub const SIGNER_MODULE_NAME: &str = "signer";

/// Where the signer module should open the server
pub const SIGNER_PORT_ENV: &str = "CB_SIGNER_PORT";

/// Comma separated list module_id=jwt_secret
pub const JWTS_ENV: &str = "CB_JWTS";

/// Path to json file with plaintext keys (testing only)
pub const SIGNER_KEYS_ENV: &str = "CB_SIGNER_LOADER_FILE";
pub const SIGNER_DEFAULT: &str = "/keys.json";
/// Path to `keys` folder
pub const SIGNER_DIR_KEYS_ENV: &str = "CB_SIGNER_LOADER_KEYS_DIR";
pub const SIGNER_DIR_KEYS_DEFAULT: &str = "/keys";
/// Path to `secrets` folder
pub const SIGNER_DIR_SECRETS_ENV: &str = "CB_SIGNER_LOADER_SECRETS_DIR";
pub const SIGNER_DIR_SECRETS_DEFAULT: &str = "/secrets";
/// Format of the directory structure for keys
pub const SIGNER_DIR_FORMAT_ENV: &str = "CB_SIGNER_LOADER_FORMAT";
pub const SIGNER_DIR_FORMAT_DEFAULT: &str = "lighthouse";
/// Path to store proxies
pub const PROXY_DIR_ENV: &str = "CB_PROXY_STORE_DIR";
pub const PROXY_DIR_DEFAULT: &str = "/proxies";

///////////////////////// MODULES /////////////////////////

/// The unique ID of the module
pub const MODULE_ID_ENV: &str = "CB_MODULE_ID";

// Commit modules
/// The JWT secret for the module to communicate with the signer module
pub const MODULE_JWT_ENV: &str = "CB_SIGNER_JWT";
/// Where to send signature request
pub const SIGNER_URL_ENV: &str = "CB_SIGNER_URL";

/// Events modules
/// Where to receive builder events
pub const BUILDER_PORT_ENV: &str = "CB_BUILDER_PORT";
