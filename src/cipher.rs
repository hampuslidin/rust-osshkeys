use std::str::FromStr;

use crate::error::{Error as OsshError, ErrorKind, OsshResult};
use self::internal_impl::*;

/// Provide an unified interface to encrypt/decrypt data
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
pub enum Cipher {
    Aes128_Cbc,
    Aes192_Cbc,
    Aes256_Cbc,
    Aes128_Ctr,
    Aes192_Ctr,
    Aes256_Ctr,
    TDes_Cbc,
    Null,
}

impl Cipher {
    pub fn encrypt(self, src: &[u8], key: &[u8], iv: &[u8]) -> OsshResult<Vec<u8>> {
        use Cipher::*;
        match self {
            Aes128_Cbc => aes128cbc_encrypt(src, key, iv),
            Aes192_Cbc => aes192cbc_encrypt(src, key, iv),
            Aes256_Cbc => aes256cbc_encrypt(src, key, iv),
            Aes128_Ctr => aes128ctr_encrypt(src, key, iv),
            Aes192_Ctr => aes192ctr_encrypt(src, key, iv),
            Aes256_Ctr => aes256ctr_encrypt(src, key, iv),
            TDes_Cbc => tdescbc_encrypt(src, key, iv),
            Null => Ok(src.to_vec()),
        }
    }

    pub fn decrypt(self, src: &[u8], key: &[u8], iv: &[u8]) -> OsshResult<Vec<u8>> {
        use Cipher::*;
        match self {
            Aes128_Cbc => aes128cbc_decrypt(src, key, iv),
            Aes192_Cbc => aes192cbc_decrypt(src, key, iv),
            Aes256_Cbc => aes256cbc_decrypt(src, key, iv),
            Aes128_Ctr => aes128ctr_decrypt(src, key, iv),
            Aes192_Ctr => aes192ctr_decrypt(src, key, iv),
            Aes256_Ctr => aes256ctr_decrypt(src, key, iv),
            TDes_Cbc => tdescbc_decrypt(src, key, iv),
            Null => Ok(src.to_vec()),
        }
    }

    pub fn key_len(self) -> usize {
        use Cipher::*;
        match self {
            Aes128_Cbc => 16,
            Aes192_Cbc => 24,
            Aes256_Cbc => 32,
            Aes128_Ctr => 16,
            Aes192_Ctr => 24,
            Aes256_Ctr => 32,
            TDes_Cbc => 24,
            Null => 0,
        }
    }

    pub fn iv_len(self) -> usize {
        use Cipher::*;
        match self {
            Aes128_Cbc => 16,
            Aes192_Cbc => 16,
            Aes256_Cbc => 16,
            Aes128_Ctr => 16,
            Aes192_Ctr => 16,
            Aes256_Ctr => 16,
            TDes_Cbc => 8,
            Null => 0,
        }
    }

    pub fn block_size(self) -> usize {
        use Cipher::*;
        match self {
            Aes128_Cbc => 16,
            Aes192_Cbc => 16,
            Aes256_Cbc => 16,
            Aes128_Ctr => 16,
            Aes192_Ctr => 16,
            Aes256_Ctr => 16,
            TDes_Cbc => 8,
            Null => 8,
        }
    }

    pub fn name(self) -> &'static str {
        use Cipher::*;
        match self {
            Aes128_Cbc => "aes128-cbc",
            Aes192_Cbc => "aes192-cbc",
            Aes256_Cbc => "aes256-cbc",
            Aes128_Ctr => "aes128-ctr",
            Aes192_Ctr => "aes192-ctr",
            Aes256_Ctr => "aes256-ctr",
            TDes_Cbc => "3des-cbc",
            Null => "none",
        }
    }

    pub fn is_null(self) -> bool {
        self == Cipher::Null
    }
}

impl FromStr for Cipher {
    type Err = OsshError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Cipher::*;
        match s {
            "3des-cbc" => Ok(TDes_Cbc),
            "aes128-cbc" => Ok(Aes128_Cbc),
            "aes192-cbc" => Ok(Aes192_Cbc),
            "aes256-cbc" | "rijndael-cbc@lysator.liu.se" => Ok(Aes256_Cbc),
            "aes128-ctr" => Ok(Aes128_Ctr),
            "aes192-ctr" => Ok(Aes192_Ctr),
            "aes256-ctr" => Ok(Aes256_Ctr),
            "none" => Ok(Null),
            _ => Err(ErrorKind::UnsupportCipher.into()),
        }
    }
}

#[cfg(not(feature = "openssl-cipher"))]
mod internal_impl {
    use aes::{Aes128, Aes192, Aes256};
    use aes_ctr::{Aes128Ctr, Aes192Ctr, Aes256Ctr};
    use stream_cipher::{NewStreamCipher, SyncStreamCipher};
    use block_modes::{block_padding::Pkcs7, BlockMode, Cbc};
    use des::TdesEde3;

    use crate::error::OsshResult;

    type Aes128Cbc = Cbc::<Aes128, Pkcs7>;
    type Aes192Cbc = Cbc::<Aes192, Pkcs7>;
    type Aes256Cbc = Cbc::<Aes256, Pkcs7>;
    type TdesCbc = Cbc::<TdesEde3, Pkcs7>;

    pub fn aes128cbc_encrypt(src: &[u8], key: &[u8], iv: &[u8]) -> OsshResult<Vec<u8>> {
        Ok(Aes128Cbc::new_var(key, iv)?.encrypt_vec(src))
    }
    pub fn aes128cbc_decrypt(src: &[u8], key: &[u8], iv: &[u8]) -> OsshResult<Vec<u8>> {
        Ok(Aes128Cbc::new_var(key, iv)?.decrypt_vec(src)?)
    }

    pub fn aes192cbc_encrypt(src: &[u8], key: &[u8], iv: &[u8]) -> OsshResult<Vec<u8>> {
        Ok(Aes192Cbc::new_var(key, iv)?.encrypt_vec(src))
    }
    pub fn aes192cbc_decrypt(src: &[u8], key: &[u8], iv: &[u8]) -> OsshResult<Vec<u8>> {
        Ok(Aes192Cbc::new_var(key, iv)?.decrypt_vec(src)?)
    }

    pub fn aes256cbc_encrypt(src: &[u8], key: &[u8], iv: &[u8]) -> OsshResult<Vec<u8>> {
        Ok(Aes256Cbc::new_var(key, iv)?.encrypt_vec(src))
    }
    pub fn aes256cbc_decrypt(src: &[u8], key: &[u8], iv: &[u8]) -> OsshResult<Vec<u8>> {
        Ok(Aes256Cbc::new_var(key, iv)?.decrypt_vec(src)?)
    }

    pub fn tdescbc_encrypt(src: &[u8], key: &[u8], iv: &[u8]) -> OsshResult<Vec<u8>> {
        Ok(TdesCbc::new_var(key, iv)?.encrypt_vec(src))
    }
    pub fn tdescbc_decrypt(src: &[u8], key: &[u8], iv: &[u8]) -> OsshResult<Vec<u8>> {
        Ok(TdesCbc::new_var(key, iv)?.decrypt_vec(src)?)
    }

    pub fn aes128ctr_encrypt(src: &[u8], key: &[u8], iv: &[u8]) -> OsshResult<Vec<u8>> {
        let mut encrypted = Vec::from(src);
        Aes128Ctr::new_var(key, iv)?.apply_keystream(&mut encrypted);
        Ok(encrypted)
    }
    pub fn aes128ctr_decrypt(src: &[u8], key: &[u8], iv: &[u8]) -> OsshResult<Vec<u8>> {
        let mut decrypted = Vec::from(src);
        Aes128Ctr::new_var(key, iv)?.apply_keystream(&mut decrypted);
        Ok(decrypted)
    }

    pub fn aes192ctr_encrypt(src: &[u8], key: &[u8], iv: &[u8]) -> OsshResult<Vec<u8>> {
        let mut encrypted = Vec::from(src);
        Aes192Ctr::new_var(key, iv)?.apply_keystream(&mut encrypted);
        Ok(encrypted)
    }
    pub fn aes192ctr_decrypt(src: &[u8], key: &[u8], iv: &[u8]) -> OsshResult<Vec<u8>> {
        let mut decrypted = Vec::from(src);
        Aes192Ctr::new_var(key, iv)?.apply_keystream(&mut decrypted);
        Ok(decrypted)
    }

    pub fn aes256ctr_encrypt(src: &[u8], key: &[u8], iv: &[u8]) -> OsshResult<Vec<u8>> {
        let mut encrypted = Vec::from(src);
        Aes256Ctr::new_var(key, iv)?.apply_keystream(&mut encrypted);
        Ok(encrypted)
    }
    pub fn aes256ctr_decrypt(src: &[u8], key: &[u8], iv: &[u8]) -> OsshResult<Vec<u8>> {
        let mut decrypted = Vec::from(src);
        Aes256Ctr::new_var(key, iv)?.apply_keystream(&mut decrypted);
        Ok(decrypted)
    }
}

#[cfg(feature = "openssl-cipher")]
mod internal_impl {
    use openssl::symm::{Cipher, Crypter, Mode};

    use crate::error::OsshResult;

    fn openssl_encrypt(cipher: Cipher, src: &[u8], key: &[u8], iv: &[u8]) -> OsshResult<Vec<u8>> {
        let mut crypt = Crypter::new(cipher, Mode::Encrypt, key, Some(iv))?;
        let mut buf = vec![0; src.len() + cipher.block_size()];
        let mut n = crypt.update(src, &mut buf)?;
        n += crypt.finalize(&mut buf[n..])?;
        buf.truncate(n);
        Ok(buf)
    }

    fn openssl_decrypt(cipher: Cipher, src: &[u8], key: &[u8], iv: &[u8]) -> OsshResult<Vec<u8>> {
        let mut crypt = Crypter::new(cipher, Mode::Decrypt, key, Some(iv))?;
        let mut buf = vec![0; src.len() + cipher.block_size()];
        let mut n = crypt.update(src, &mut buf)?;
        n += crypt.finalize(&mut buf[n..])?;
        buf.truncate(n);
        Ok(buf)
    }

    pub fn aes128cbc_encrypt(src: &[u8], key: &[u8], iv: &[u8]) -> OsshResult<Vec<u8>> {
        Ok(openssl_encrypt(Cipher::aes_128_cbc(), src, key, iv)?)
    }
    pub fn aes128cbc_decrypt(src: &[u8], key: &[u8], iv: &[u8]) -> OsshResult<Vec<u8>> {
        Ok(openssl_decrypt(Cipher::aes_128_cbc(), src, key, iv)?)
    }

    pub fn aes192cbc_encrypt(src: &[u8], key: &[u8], iv: &[u8]) -> OsshResult<Vec<u8>> {
        Ok(openssl_encrypt(Cipher::aes_192_cbc(), src, key, iv)?)
    }
    pub fn aes192cbc_decrypt(src: &[u8], key: &[u8], iv: &[u8]) -> OsshResult<Vec<u8>> {
        Ok(openssl_decrypt(Cipher::aes_192_cbc(), src, key, iv)?)
    }

    pub fn aes256cbc_encrypt(src: &[u8], key: &[u8], iv: &[u8]) -> OsshResult<Vec<u8>> {
        Ok(openssl_encrypt(Cipher::aes_256_cbc(), src, key, iv)?)
    }
    pub fn aes256cbc_decrypt(src: &[u8], key: &[u8], iv: &[u8]) -> OsshResult<Vec<u8>> {
        Ok(openssl_decrypt(Cipher::aes_256_cbc(), src, key, iv)?)
    }

    pub fn tdescbc_encrypt(src: &[u8], key: &[u8], iv: &[u8]) -> OsshResult<Vec<u8>> {
        Ok(openssl_encrypt(Cipher::des_ede3_cbc(), src, key, iv)?)
    }
    pub fn tdescbc_decrypt(src: &[u8], key: &[u8], iv: &[u8]) -> OsshResult<Vec<u8>> {
        Ok(openssl_decrypt(Cipher::des_ede3_cbc(), src, key, iv)?)
    }

    pub fn aes128ctr_encrypt(src: &[u8], key: &[u8], iv: &[u8]) -> OsshResult<Vec<u8>> {
        Ok(openssl_encrypt(Cipher::aes_128_ctr(), src, key, iv)?)
    }
    pub fn aes128ctr_decrypt(src: &[u8], key: &[u8], iv: &[u8]) -> OsshResult<Vec<u8>> {
        Ok(openssl_decrypt(Cipher::aes_128_ctr(), src, key, iv)?)
    }

    pub fn aes192ctr_encrypt(src: &[u8], key: &[u8], iv: &[u8]) -> OsshResult<Vec<u8>> {
        Ok(openssl_encrypt(Cipher::aes_192_ctr(), src, key, iv)?)
    }
    pub fn aes192ctr_decrypt(src: &[u8], key: &[u8], iv: &[u8]) -> OsshResult<Vec<u8>> {
        Ok(openssl_decrypt(Cipher::aes_192_ctr(), src, key, iv)?)
    }

    pub fn aes256ctr_encrypt(src: &[u8], key: &[u8], iv: &[u8]) -> OsshResult<Vec<u8>> {
        Ok(openssl_encrypt(Cipher::aes_256_ctr(), src, key, iv)?)
    }
    pub fn aes256ctr_decrypt(src: &[u8], key: &[u8], iv: &[u8]) -> OsshResult<Vec<u8>> {
        Ok(openssl_decrypt(Cipher::aes_256_ctr(), src, key, iv)?)
    }
}