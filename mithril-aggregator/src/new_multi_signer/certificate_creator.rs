use chrono::{DateTime, Utc};
use mithril_common::{
    crypto_helper::{key_encode_hex, ProtocolMultiSignature, PROTOCOL_VERSION},
    entities::{self, Beacon, PartyId, ProtocolMessage, ProtocolParameters, SignerWithStake},
};

/// TODO: use dedicated error
use crate::ProtocolError;

pub trait CertificateCreator {
    fn create_certificate(
        working: &WorkingCertificate,
        signatures_party_ids: &[PartyId],
        multi_signature: ProtocolMultiSignature,
    ) -> Result<entities::Certificate, ProtocolError>;
}

pub struct MithrilCertificateCreator {}

/// Laius explicant pourquoi ce n'est pas le PendingCertificate ?
pub struct WorkingCertificate {
    /// Current Beacon
    pub beacon: Beacon,

    /// Current Protocol parameters
    pub protocol_parameters: ProtocolParameters,

    /// Next Protocol parameters
    pub next_protocol_parameters: ProtocolParameters,

    /// Current Signers
    pub signers: Vec<SignerWithStake>,

    /// Message that is currently signed
    pub message: ProtocolMessage,

    /// Created aggregate verification key
    pub aggregate_verification_key: String,

    /// Signing start datetime of current message
    pub initiated_at: DateTime<Utc>,

    previous_hash: String,
}

impl CertificateCreator for MithrilCertificateCreator {
    /// Creates a certificate from a multi signature
    fn create_certificate(
        working: &WorkingCertificate,
        signatures_party_ids: &[PartyId],
        multi_signature: ProtocolMultiSignature,
    ) -> Result<entities::Certificate, ProtocolError> {
        let protocol_version = PROTOCOL_VERSION.to_string();
        let initiated_at = format!("{:?}", working.initiated_at);
        let sealed_at = format!("{:?}", Utc::now());
        let signers = working
            .signers
            .iter()
            .filter(|signer| signatures_party_ids.contains(&signer.party_id))
            .cloned()
            .collect::<Vec<_>>();
        let metadata = entities::CertificateMetadata::new(
            protocol_version,
            working.protocol_parameters.clone(),
            initiated_at,
            sealed_at,
            signers,
        );
        let multi_signature = key_encode_hex(&multi_signature).map_err(ProtocolError::Codec)?;
        let genesis_signature = "".to_string();

        Ok(entities::Certificate::new(
            working.previous_hash.clone(),
            working.beacon.clone(),
            metadata,
            working.message.clone(),
            working.aggregate_verification_key.clone(),
            multi_signature,
            genesis_signature,
        ))
    }
}

#[cfg(test)]
mod tests {
    use chrono::{DateTime, Utc};
    use mithril_common::{
        crypto_helper::{key_decode_hex, tests_setup::setup_certificate_chain},
        entities::PartyId,
    };
    use std::str::FromStr;

    use crate::new_multi_signer::certificate_creator::MithrilCertificateCreator;

    use super::{CertificateCreator, WorkingCertificate};

    #[test]
    fn test() {
        let (certificates, _) = setup_certificate_chain(3, 1);
        let expected = &certificates[1];
        let next_cert = &certificates[0];
        let working_certicate = WorkingCertificate {
            beacon: expected.beacon.clone(),
            protocol_parameters: expected.metadata.protocol_parameters.clone(),
            next_protocol_parameters: next_cert.metadata.protocol_parameters.clone(),
            signers: expected.metadata.signers.clone(),
            message: expected.protocol_message.clone(),
            aggregate_verification_key: expected.aggregate_verification_key.clone(),
            initiated_at: DateTime::<Utc>::from_str(&expected.metadata.initiated_at).unwrap(),
            previous_hash: expected.previous_hash.clone(),
        };
        let party_ids: Vec<PartyId> = expected
            .metadata
            .signers
            .iter()
            .map(|s| s.party_id.clone())
            .collect();

        let mut certificate = MithrilCertificateCreator::create_certificate(
            &working_certicate,
            &party_ids,
            key_decode_hex(&expected.multi_signature.clone()).unwrap(),
        )
        .expect("certificate creation should not fail");
        // Note: We can't sync the 'sealed_at' property with the expected cert before hand since it's
        // computed by create_certificate itself, so we need to do that now :
        certificate.metadata.sealed_at = expected.metadata.sealed_at.clone();
        certificate.hash = certificate.compute_hash();

        assert_eq!(expected, &certificate);
    }
}
