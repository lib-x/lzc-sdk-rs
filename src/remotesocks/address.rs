use std::fmt;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use std::str::FromStr;

use base64::Engine as _;
use base64::engine::general_purpose::STANDARD_NO_PAD;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

use crate::Error;

const IPV4_ADDRESS: u8 = 0x01;
const DOMAIN_ADDRESS: u8 = 0x03;
const IPV6_ADDRESS: u8 = 0x04;
const CUSTOM_SUFFIX: &str = ".custom";

/// An RFC 1928 SOCKS address or `LazyCat` custom-network address.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum SocksAddress {
    /// Numeric IPv4 or IPv6 socket address.
    Ip(SocketAddr),
    /// Domain name and port resolved by the SOCKS server.
    Domain {
        /// Domain name carried on the wire without DNS resolution.
        host: String,
        /// Destination port.
        port: u16,
    },
    /// `LazyCat` custom network encoded into the SOCKS domain field.
    Custom {
        /// Custom network identifier, such as `unix`.
        network: String,
        /// Address interpreted by the custom network implementation.
        address: String,
    },
}

impl SocksAddress {
    /// Construct a domain address without resolving it locally.
    ///
    /// # Errors
    ///
    /// Returns an error for an empty host or a host longer than 255 bytes.
    pub fn domain(host: impl Into<String>, port: u16) -> Result<Self, Error> {
        let host = host.into();
        validate_wire_string(&host)?;
        if host.contains(':')
            || host
                .chars()
                .any(|character| character.is_whitespace() || character.is_control())
        {
            return Err(Error::InvalidSocksAddress);
        }
        Ok(Self::Domain { host, port })
    }

    /// Construct a `LazyCat` custom-network address.
    ///
    /// # Errors
    ///
    /// Returns an error for an empty network or when the encoded custom domain
    /// is longer than 255 bytes.
    pub fn custom(network: impl Into<String>, address: impl Into<String>) -> Result<Self, Error> {
        let network = network.into();
        let address = address.into();
        if network.is_empty() {
            return Err(Error::InvalidSocksAddress);
        }
        let encoded = encode_custom_domain(&network, &address);
        validate_wire_string(&encoded)?;
        Ok(Self::Custom { network, address })
    }

    /// Encode this address using the RFC 1928 wire representation.
    ///
    /// # Errors
    ///
    /// Returns an error if a string address cannot fit in the one-byte length
    /// field.
    pub fn encode(&self) -> Result<Vec<u8>, Error> {
        let mut encoded = Vec::with_capacity(19);
        match self {
            Self::Ip(SocketAddr::V4(address)) => {
                encoded.push(IPV4_ADDRESS);
                encoded.extend_from_slice(&address.ip().octets());
                encoded.extend_from_slice(&address.port().to_be_bytes());
            }
            Self::Ip(SocketAddr::V6(address)) => {
                encoded.push(IPV6_ADDRESS);
                encoded.extend_from_slice(&address.ip().octets());
                encoded.extend_from_slice(&address.port().to_be_bytes());
            }
            Self::Domain { host, port } => {
                encode_domain(&mut encoded, host, *port)?;
            }
            Self::Custom { network, address } => {
                encode_domain(&mut encoded, &encode_custom_domain(network, address), 0)?;
            }
        }
        Ok(encoded)
    }

    /// Decode exactly one RFC 1928 address.
    ///
    /// # Errors
    ///
    /// Returns an error for truncated data, trailing bytes, invalid UTF-8,
    /// malformed custom addresses, or unsupported address types.
    pub fn decode(encoded: &[u8]) -> Result<Self, Error> {
        let (&address_type, remainder) = encoded.split_first().ok_or(Error::InvalidSocksAddress)?;
        match address_type {
            IPV4_ADDRESS => {
                if remainder.len() != 6 {
                    return Err(Error::InvalidSocksAddress);
                }
                let ip = Ipv4Addr::new(remainder[0], remainder[1], remainder[2], remainder[3]);
                let port = u16::from_be_bytes([remainder[4], remainder[5]]);
                Ok(Self::Ip(SocketAddr::new(IpAddr::V4(ip), port)))
            }
            IPV6_ADDRESS => {
                if remainder.len() != 18 {
                    return Err(Error::InvalidSocksAddress);
                }
                let octets: [u8; 16] = remainder[..16]
                    .try_into()
                    .map_err(|_| Error::InvalidSocksAddress)?;
                let port = u16::from_be_bytes([remainder[16], remainder[17]]);
                Ok(Self::Ip(SocketAddr::new(
                    IpAddr::V6(Ipv6Addr::from(octets)),
                    port,
                )))
            }
            DOMAIN_ADDRESS => decode_domain(remainder),
            address_type => Err(Error::UnsupportedSocksAddressType { address_type }),
        }
    }

    /// Network name associated with this address.
    #[must_use]
    pub fn network(&self) -> &str {
        match self {
            Self::Custom { network, .. } => network,
            Self::Ip(_) | Self::Domain { .. } => "socks5",
        }
    }

    /// Destination port, or zero for custom-network addresses.
    #[must_use]
    pub const fn port(&self) -> u16 {
        match self {
            Self::Ip(address) => address.port(),
            Self::Domain { port, .. } => *port,
            Self::Custom { .. } => 0,
        }
    }

    pub(crate) async fn read_from(reader: &mut (impl AsyncRead + Unpin)) -> Result<Self, Error> {
        let address_type = reader.read_u8().await?;
        let mut encoded = vec![address_type];
        match address_type {
            IPV4_ADDRESS => {
                let mut remainder = [0_u8; 6];
                reader.read_exact(&mut remainder).await?;
                encoded.extend_from_slice(&remainder);
            }
            IPV6_ADDRESS => {
                let mut remainder = [0_u8; 18];
                reader.read_exact(&mut remainder).await?;
                encoded.extend_from_slice(&remainder);
            }
            DOMAIN_ADDRESS => {
                let length = reader.read_u8().await?;
                encoded.push(length);
                let mut remainder = vec![0_u8; usize::from(length) + 2];
                reader.read_exact(&mut remainder).await?;
                encoded.extend_from_slice(&remainder);
            }
            address_type => return Err(Error::UnsupportedSocksAddressType { address_type }),
        }
        Self::decode(&encoded)
    }

    pub(crate) async fn write_to(
        &self,
        writer: &mut (impl AsyncWrite + Unpin),
    ) -> Result<(), Error> {
        writer.write_all(&self.encode()?).await?;
        Ok(())
    }
}

impl From<SocketAddr> for SocksAddress {
    fn from(address: SocketAddr) -> Self {
        Self::Ip(address)
    }
}

impl FromStr for SocksAddress {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if let Ok(address) = value.parse::<SocketAddr>() {
            return Ok(Self::Ip(address));
        }
        let (host, port) = value.rsplit_once(':').ok_or(Error::InvalidSocksAddress)?;
        if host.is_empty() || host.starts_with('[') || host.ends_with(']') {
            return Err(Error::InvalidSocksAddress);
        }
        let port = port
            .parse::<u16>()
            .map_err(|_| Error::InvalidSocksAddress)?;
        Self::domain(host, port)
    }
}

impl fmt::Display for SocksAddress {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ip(address) => address.fmt(formatter),
            Self::Domain { host, port } => write!(formatter, "{host}:{port}"),
            Self::Custom { address, .. } => address.fmt(formatter),
        }
    }
}

fn validate_wire_string(value: &str) -> Result<(), Error> {
    if value.is_empty() {
        return Err(Error::InvalidSocksAddress);
    }
    if value.len() > u8::MAX.into() {
        return Err(Error::SocksAddressTooLong);
    }
    Ok(())
}

fn encode_domain(encoded: &mut Vec<u8>, domain: &str, port: u16) -> Result<(), Error> {
    validate_wire_string(domain)?;
    encoded.push(DOMAIN_ADDRESS);
    encoded.push(u8::try_from(domain.len()).map_err(|_| Error::SocksAddressTooLong)?);
    encoded.extend_from_slice(domain.as_bytes());
    encoded.extend_from_slice(&port.to_be_bytes());
    Ok(())
}

fn decode_domain(remainder: &[u8]) -> Result<SocksAddress, Error> {
    let (&length, remainder) = remainder.split_first().ok_or(Error::InvalidSocksAddress)?;
    let length = usize::from(length);
    if length == 0 || remainder.len() != length + 2 {
        return Err(Error::InvalidSocksAddress);
    }
    let host = std::str::from_utf8(&remainder[..length]).map_err(|_| Error::InvalidSocksAddress)?;
    let port = u16::from_be_bytes([remainder[length], remainder[length + 1]]);
    if port == 0 && host.ends_with(CUSTOM_SUFFIX) {
        let (network, address) = decode_custom_domain(host)?;
        SocksAddress::custom(network, address)
    } else {
        SocksAddress::domain(host, port)
    }
}

fn encode_custom_domain(network: &str, address: &str) -> String {
    format!(
        "{}.{}{}",
        STANDARD_NO_PAD.encode(network),
        STANDARD_NO_PAD.encode(address),
        CUSTOM_SUFFIX
    )
}

fn decode_custom_domain(domain: &str) -> Result<(String, String), Error> {
    let encoded = domain
        .strip_suffix(CUSTOM_SUFFIX)
        .ok_or(Error::InvalidSocksAddress)?;
    let mut fields = encoded.split('.');
    let network = fields.next().ok_or(Error::InvalidSocksAddress)?;
    let address = fields.next().ok_or(Error::InvalidSocksAddress)?;
    if network.is_empty() || fields.next().is_some() {
        return Err(Error::InvalidSocksAddress);
    }
    let network = STANDARD_NO_PAD
        .decode(network)
        .map_err(|_| Error::InvalidSocksAddress)?;
    let address = STANDARD_NO_PAD
        .decode(address)
        .map_err(|_| Error::InvalidSocksAddress)?;
    let network = String::from_utf8(network).map_err(|_| Error::InvalidSocksAddress)?;
    let address = String::from_utf8(address).map_err(|_| Error::InvalidSocksAddress)?;
    Ok((network, address))
}
