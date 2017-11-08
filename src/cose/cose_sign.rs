/// We don't need COSE signing at the moment. But we need to generate test files.
/// This module implements basic COSE signing.
use cose::nss;
use cose::decoder::*;
use cose::CoseError;
use cose::util::{build_cose_signature, build_protected_header, build_protected_sig_header,
                 get_sig_struct_bytes};

pub fn sign(
    payload: &[u8],
    alg: CoseSignatureType,
    ee_cert: &[u8],
    cert_chain: &[&[u8]],
    pkcs8: &[u8],
) -> Result<Vec<u8>, CoseError> {
    let nss_alg = match alg {
        CoseSignatureType::ES256 => nss::SignatureAlgorithm::ES256,
        CoseSignatureType::PS256 => nss::SignatureAlgorithm::PS256,
    };

    // Build the signature structure containing the protected headers and the
    // payload to generate the payload that is actually signed.
    let protected_sig_header_serialized = build_protected_sig_header(ee_cert, &alg);
    let protected_header_serialized = build_protected_header(cert_chain);
    let payload = get_sig_struct_bytes(
        protected_header_serialized,
        protected_sig_header_serialized,
        payload,
    );

    let signature = match nss::sign(&nss_alg, &pkcs8, &payload) {
        Err(_) => return Err(CoseError::SigningFailed),
        Ok(signature) => signature,
    };
    let cose_signature = build_cose_signature(cert_chain, ee_cert, &signature, &alg);
    Ok(cose_signature)
}
