use hmac::Mac;

use crate::providers::ProviderKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerificationResult {
    Valid,
    Invalid,
    NotSupported,
}

pub fn verify(
    provider: ProviderKind,
    body: &[u8],
    signature_header: &str,
    secret: &[u8],
) -> VerificationResult {
    match provider {
        ProviderKind::GitHub => verify_sha256(body, signature_header, secret, "sha256="),
        ProviderKind::GitLab => {
            if signature_header.starts_with("sha256=") {
                verify_sha256(body, signature_header, secret, "sha256=")
            } else if signature_header.starts_with("sha1=") {
                verify_sha1(body, signature_header, secret, "sha1=")
            } else {
                VerificationResult::Invalid
            }
        }
        ProviderKind::GiteaFamily => {
            verify_sha256(body, signature_header, secret, "sha256=")
        }
        ProviderKind::Bitbucket => verify_raw_sha256(body, signature_header, secret),
        ProviderKind::AzureDevOps => VerificationResult::NotSupported,
    }
}

fn verify_sha256(
    body: &[u8],
    signature_header: &str,
    secret: &[u8],
    prefix: &str,
) -> VerificationResult {
    let provided = match signature_header.strip_prefix(prefix) {
        Some(s) => s,
        None => return VerificationResult::Invalid,
    };

    let mut mac = match hmac::Hmac::<sha2::Sha256>::new_from_slice(secret) {
        Ok(m) => m,
        Err(_) => return VerificationResult::Invalid,
    };
    mac.update(body);
    let expected = hex::encode(mac.finalize().into_bytes());

    if provided == expected {
        VerificationResult::Valid
    } else {
        VerificationResult::Invalid
    }
}

fn verify_sha1(
    body: &[u8],
    signature_header: &str,
    secret: &[u8],
    prefix: &str,
) -> VerificationResult {
    let provided = match signature_header.strip_prefix(prefix) {
        Some(s) => s,
        None => return VerificationResult::Invalid,
    };

    let mut mac = match hmac::Hmac::<sha1::Sha1>::new_from_slice(secret) {
        Ok(m) => m,
        Err(_) => return VerificationResult::Invalid,
    };
    mac.update(body);
    let expected = hex::encode(mac.finalize().into_bytes());

    if provided == expected {
        VerificationResult::Valid
    } else {
        VerificationResult::Invalid
    }
}

fn verify_raw_sha256(
    body: &[u8],
    signature_header: &str,
    secret: &[u8],
) -> VerificationResult {
    let mut mac = match hmac::Hmac::<sha2::Sha256>::new_from_slice(secret) {
        Ok(m) => m,
        Err(_) => return VerificationResult::Invalid,
    };
    mac.update(body);
    let expected = hex::encode(mac.finalize().into_bytes());

    if signature_header == expected {
        VerificationResult::Valid
    } else {
        VerificationResult::Invalid
    }
}
