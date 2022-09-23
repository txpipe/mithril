use super::super::entities;
use super::types;
use crate::crypto_helper::{
    key_encode_hex, ProtocolInitializer, ProtocolPartyId, ProtocolSigner,
    ProtocolSignerVerificationKey, ProtocolStake,
};

impl From<types::ProtocolParameters> for entities::ProtocolParameters {
    fn from(other: types::ProtocolParameters) -> Self {
        entities::ProtocolParameters::new(other.k, other.m, other.phi_f)
    }
}

impl From<entities::ProtocolParameters> for types::ProtocolParameters {
    fn from(other: entities::ProtocolParameters) -> Self {
        types::ProtocolParameters {
            k: other.k,
            m: other.m,
            phi_f: other.phi_f as f64,
        }
    }
}

impl From<entities::SignerWithStake> for entities::Signer {
    fn from(other: entities::SignerWithStake) -> Self {
        entities::Signer::new(
            other.party_id,
            other.verification_key,
            other.verification_key_signature,
            other.operational_certificate,
        )
    }
}

impl From<&entities::SignerWithStake> for entities::Signer {
    fn from(other: &entities::SignerWithStake) -> Self {
        entities::Signer::new(
            other.party_id.clone(),
            other.verification_key.clone(),
            other.verification_key_signature.clone(),
            other.operational_certificate.clone(),
        )
    }
}

impl From<&entities::SignerWithStake> for (types::ProtocolPartyId, types::ProtocolStake) {
    fn from(other: &entities::SignerWithStake) -> Self {
        (
            other.party_id.clone() as ProtocolPartyId,
            other.stake as ProtocolStake,
        )
    }
}

impl From<(types::ProtocolPartyId, types::ProtocolStake)> for entities::SignerWithStake {
    fn from(other: (types::ProtocolPartyId, types::ProtocolStake)) -> Self {
        entities::SignerWithStake::new(other.0, "".to_string(), None, None, other.1)
    }
}

impl
    From<(
        ProtocolPartyId,
        ProtocolStake,
        ProtocolSignerVerificationKey,
        ProtocolSigner,
        ProtocolInitializer,
    )> for entities::SignerWithStake
{
    fn from(
        other: (
            ProtocolPartyId,
            ProtocolStake,
            ProtocolSignerVerificationKey,
            ProtocolSigner,
            ProtocolInitializer,
        ),
    ) -> Self {
        let (party_id, stake, verification_key, _, _) = other;
        entities::SignerWithStake::new(
            party_id,
            key_encode_hex(verification_key).unwrap(),
            None,
            None,
            stake,
        )
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_protocol_parameters_from_into() {
        let protocol_parameters_expected = types::ProtocolParameters {
            k: 100,
            m: 1000,
            phi_f: 1.0,
        };
        let protocol_initializer_entities_expected = entities::ProtocolParameters::new(
            protocol_parameters_expected.k,
            protocol_parameters_expected.m,
            protocol_parameters_expected.phi_f,
        );

        let protocol_initializer_entities_into: entities::ProtocolParameters =
            protocol_parameters_expected.into();
        assert_eq!(
            protocol_initializer_entities_expected,
            protocol_initializer_entities_into
        );

        let protocol_initializer_from: types::ProtocolParameters =
            protocol_initializer_entities_expected.into();
        assert_eq!(protocol_parameters_expected, protocol_initializer_from);
    }

    #[test]
    fn test_stake_distribution_from_into() {
        let stake_expected = (
            "1".to_string() as types::ProtocolPartyId,
            100 as types::ProtocolStake,
        );
        let signer_with_stake_expected =
            &entities::SignerWithStake::new("1".to_string(), "".to_string(), None, None, 100);

        let signer_with_stake_expected_into: (types::ProtocolPartyId, types::ProtocolStake) =
            signer_with_stake_expected.into();
        assert_eq!(stake_expected, signer_with_stake_expected_into);

        let stake_expected_from: entities::SignerWithStake = stake_expected.into();
        assert_eq!(signer_with_stake_expected, &stake_expected_from);
    }

    #[test]
    fn test_stake_signers_from_into() {
        let signer_expected =
            entities::Signer::new("1".to_string(), "123456".to_string(), None, None);
        let signer_with_stake =
            entities::SignerWithStake::new("1".to_string(), "123456".to_string(), None, None, 100);

        let signer_into: entities::Signer = signer_with_stake.into();
        assert_eq!(signer_expected, signer_into);
    }
}
