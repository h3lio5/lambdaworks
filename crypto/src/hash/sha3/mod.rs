use lambdaworks_math::{
    field::{
        element::FieldElement,
        fields::montgomery_backed_prime_fields::{IsModulus, MontgomeryBackendPrimeField},
    },
    traits::ByteConversion,
    unsigned_integer::element::UnsignedInteger,
};
use sha3::{Digest, Sha3_256};

use super::traits::IsCryptoHash;
pub struct Sha3Hasher;

/// Sha3 Hasher used over fields
/// Notice while it's generic over F, it's only generates enough randomness for fields of at most 256 bits
impl Sha3Hasher {
    pub const fn new() -> Self {
        Self
    }

    pub fn expand_message(msg: &[u8], dst: &[u8], len_in_bytes: u64) -> Result<Vec<u8>, String> {
        let b_in_bytes = Sha3_256::output_size() as u64;

        let ell = (len_in_bytes + b_in_bytes - 1) / b_in_bytes;
        if ell > 255 {
            return Err("Abort".to_string());
        }

        let dst_prime: Vec<u8> = [dst, &Self::i2osp(dst.len() as u64, 1)].concat();
        let z_pad = Self::i2osp(0, 64);
        let l_i_b_str = Self::i2osp(len_in_bytes, 2);
        let msg_prime = [
            z_pad,
            msg.to_vec(),
            l_i_b_str,
            Self::i2osp(0, 1),
            dst_prime.clone(),
        ]
        .concat();
        let b_0: Vec<u8> = Sha3_256::digest(msg_prime).to_vec();
        let a = [b_0.clone(), Self::i2osp(1, 1), dst_prime.clone()].concat();
        let b_1 = Sha3_256::digest(a).to_vec();

        let mut b_vals = Vec::<Vec<u8>>::with_capacity(ell as usize * b_in_bytes as usize);
        b_vals.push(b_1);
        for idx in 1..ell {
            let aux = Self::strxor(&b_0, &b_vals[idx as usize - 1]);
            let b_i = [aux, Self::i2osp(idx, 1), dst_prime.clone()].concat();
            b_vals.push(Sha3_256::digest(b_i).to_vec());
        }

        let mut b_vals = b_vals.concat();
        b_vals.truncate(len_in_bytes as usize);

        Ok(b_vals)
    }

    fn i2osp(x: u64, length: u64) -> Vec<u8> {
        let mut x_aux = x;
        let mut digits = Vec::new();
        while x_aux != 0 {
            digits.push((x_aux % 256) as u8);
            x_aux /= 256;
        }
        digits.resize(digits.len() + (length - digits.len() as u64) as usize, 0);
        digits.reverse();
        digits
    }

    fn strxor(a: &[u8], b: &[u8]) -> Vec<u8> {
        a.iter().zip(b).map(|(a, b)| a ^ b).collect()
    }
}
impl<M: IsModulus<UnsignedInteger<N>> + Clone, const N: usize>
    IsCryptoHash<MontgomeryBackendPrimeField<M, N>> for Sha3Hasher
{
    fn hash_one(
        &self,
        input: &FieldElement<MontgomeryBackendPrimeField<M, N>>,
    ) -> FieldElement<MontgomeryBackendPrimeField<M, N>> {
        let mut hasher = Sha3_256::new();
        hasher.update(input.to_bytes_be());
        let mut result_hash = [0_u8; 32];
        result_hash.copy_from_slice(&hasher.finalize());
        FieldElement::from_bytes_le(&result_hash).unwrap()
    }

    fn hash_two(
        &self,
        left: &FieldElement<MontgomeryBackendPrimeField<M, N>>,
        right: &FieldElement<MontgomeryBackendPrimeField<M, N>>,
    ) -> FieldElement<MontgomeryBackendPrimeField<M, N>> {
        let mut hasher = Sha3_256::new();
        hasher.update(left.to_bytes_be());
        hasher.update(right.to_bytes_be());
        let mut result_hash = [0_u8; 32];
        result_hash.copy_from_slice(&hasher.finalize());
        FieldElement::from_bytes_le(&result_hash).unwrap()
    }
}
