use base32::encode;
use base32::Alphabet::RFC4648;
use bytes::{BufMut, BytesMut};
use crc16::*;
use ed25519_dalek::Keypair;
use rand::rngs::OsRng;
use rand::{CryptoRng, Rng};

pub struct AddressGenerator<T = OsRng>
where
    T: Rng + CryptoRng,
{
    rng: T,
}

impl<T> AddressGenerator<T>
where
    T: Rng + CryptoRng,
{
    pub fn new(rng: T) -> AddressGenerator<T> {
        AddressGenerator { rng }
    }

    fn generate_random_key(&mut self) -> (String, String) {
        // Generate ED25519 key pair
        let keypair: Keypair = Keypair::generate(&mut self.rng);

        // ************** Encode the public key ***************** //
        const VERSION_BYTE_ACCOUNT_ID: u8 = 6 << 3;
        let mut bytes_public = vec![VERSION_BYTE_ACCOUNT_ID];
        // Combine the byte version and the ED25519 raw public key bytes array
        &bytes_public.extend_from_slice(&keypair.public.to_bytes());
        // Calculate checksum
        let checksum_public = State::<XMODEM>::calculate(&bytes_public);
        // Create a buffer to combine byte version : ED25519 raw key : checksum
        let mut bytes_buffer_public = BytesMut::with_capacity(1024);
        bytes_buffer_public.put(&bytes_public);
        bytes_buffer_public.put_u16_le(checksum_public);
        // Base 32 encode the public key
        let public_key = encode(RFC4648 { padding: false }, &bytes_buffer_public);

        // ************** Encode the private key ***************** //
        const VERSION_BYTE_SEED: u8 = 18 << 3;
        let mut bytes_private = vec![VERSION_BYTE_SEED];
        // Combine the byte version and the ED25519 raw private key bytes array
        &bytes_private.extend_from_slice(&keypair.secret.to_bytes());
        // Calculate checksum
        let check_sum_private = State::<XMODEM>::calculate(&bytes_private);
        // Create a buffer to combine byte version : ED25519 raw key : checksum
        let mut bytes_buffer_private = BytesMut::with_capacity(1024);
        bytes_buffer_private.put(&bytes_private);
        bytes_buffer_private.put_u16_le(check_sum_private);
        // Base 32 encode the private key
        let private_key = encode(RFC4648 { padding: false }, &bytes_buffer_private);

        (public_key, private_key)
    }
}

impl Default for AddressGenerator<OsRng> {
    fn default() -> Self {
        let rng = OsRng::new().unwrap();

        AddressGenerator { rng }
    }
}

impl<T> Iterator for AddressGenerator<T>
where
    T: Rng + CryptoRng,
{
    type Item = (String, String);

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.generate_random_key())
    }
}
