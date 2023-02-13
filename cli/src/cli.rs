//! Command-line interface flags and parameters.

use std::path::PathBuf;

use clap::Parser;
use http::uri;

/// Accepted arguments by the command-line application.
#[derive(Parser)]
#[command(author, version, about)]
pub struct App {
    // Flags.
    #[arg(long, short = 'c', default_value_t = 1)]
    pub ehter_chain_id: u64,
    #[arg(long, short = 'a', value_parser = validate_ether_address)]
    pub ether_contract_address: String,
    /// Ethereum endpoint. Format http(s)?://<host>:<port>
    #[arg(long, short = 'e')]
    pub ether_endpoint: Endpoint,
    /// Ethereum private key of the CID's owner. Format 0x.... or without it
    #[arg(long, short = 'p', value_parser = validate_ether_private_key)]
    pub ether_owner_priv_key: String,

    #[arg(long, short = 'i')]
    /// IPFS endpoint. Format http(s)?://<host>:<port>
    pub ipfs_endpoint: Endpoint,

    // Positional arguments.
    pub filepath: PathBuf,
    /// The path to set for the uploaded file
    #[arg(value_parser = validate_remote_path)]
    pub remote_path: Option<String>,
}

/// Validates if a passed Ethereum address is of a valid format.
fn validate_ether_address(addr: &str) -> Result<String, String> {
    use ethers::types::Address;

    match addr.parse::<Address>() {
        Ok(_) => Ok(String::from(addr)),
        Err(err) => Err(format!("invalid format for Ethereum address. {}", err)),
    }
}

/// Validates if a passed Ethereum private key is of a valid format.
fn validate_ether_private_key(key: &str) -> Result<String, String> {
    use ethers::signers::LocalWallet;

    let key = if key.starts_with("0x") {
        &key[2..]
    } else {
        key
    };

    match key.parse::<LocalWallet>() {
        Ok(_) => Ok(String::from(key)),
        Err(err) => Err(format!("invalid format for Ethereum private key. {}", err)),
    }
}

fn validate_remote_path(p: &str) -> Result<String, String> {
    if p.starts_with("/") {
        Ok(String::from(p))
    } else {
        Err(String::from("invalid remote path, it MUST start with '/'"))
    }
}

/// Contains the parts of an endpoint only formed by scheme, host, and port.
/// It's used by Clap to parse the passed argument and validate all and only these parts.
#[derive(Clone)]
pub struct Endpoint {
    pub scheme: uri::Scheme,
    pub host: String,
    pub port: u16,
}

impl std::fmt::Display for Endpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}://{}:{}", self.scheme, self.host, self.port)
    }
}

impl std::str::FromStr for Endpoint {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let u = s
            .parse::<uri::Uri>()
            .map_err(|err| format!("Invalid URI. {}", err))?;

        let scheme = if let Some(scheme) = u.scheme() {
            if *scheme != uri::Scheme::HTTP && *scheme != uri::Scheme::HTTPS {
                return Err(String::from(
                    "Invalid endpoint, only HTTP and HTTPS schemes are accepted",
                ));
            }

            scheme
        } else {
            return Err(String::from("Invalid endpoint, no scheme provided"));
        };

        let host = if let Some(host) = u.host() {
            host
        } else {
            return Err(String::from("Invalid endpoint, no host provided"));
        };

        let port = if let Some(port) = u.port_u16() {
            port
        } else {
            return Err(String::from(
                "Invalid endpoint, no port provided or isn't a valid unsigned 16 bits number",
            ));
        };

        Ok(Endpoint {
            scheme: scheme.clone(),
            host: String::from(host),
            port,
        })
    }
}

#[cfg(test)]
mod test {
    // TODO: Write tests for the validate functions, Endpoint::from_str, and Endpoint::fmt.
}
