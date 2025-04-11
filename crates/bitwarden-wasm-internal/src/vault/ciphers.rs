use std::rc::Rc;

use bitwarden_core::Client;
use bitwarden_vault::{
    Cipher, CipherListView, CipherView, DecryptError, EncryptError, Fido2CredentialView,
    VaultClientExt,
};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct ClientCiphers(Rc<Client>);

impl ClientCiphers {
    pub fn new(client: Rc<Client>) -> Self {
        Self(client)
    }
}

#[wasm_bindgen]
impl ClientCiphers {
    /// Encrypt cipher
    ///
    /// # Arguments
    /// - `cipher_view` - The decrypted cipher to encrypt
    ///
    /// # Returns
    /// - `Ok(Cipher)` containing the encrypted cipher
    /// - `Err(EncryptError)` if encryption fails
    pub fn encrypt(&self, cipher_view: CipherView) -> Result<Cipher, EncryptError> {
        self.0.vault().ciphers().encrypt(cipher_view)
    }

    /// Decrypt cipher
    ///
    /// # Arguments
    /// - `cipher` - The encrypted cipher to decrypt
    ///
    /// # Returns
    /// - `Ok(CipherView)` containing the decrypted cipher
    /// - `Err(DecryptError)` if decryption fails
    pub fn decrypt(&self, cipher: Cipher) -> Result<CipherView, DecryptError> {
        self.0.vault().ciphers().decrypt(cipher)
    }

    /// Decrypt list of ciphers
    ///
    /// # Arguments
    /// - `ciphers` - The list of encrypted ciphers to decrypt
    ///
    /// # Returns
    /// - `Ok(Vec<CipherListView>)` containing the decrypted ciphers
    /// - `Err(DecryptError)` if decryption fails
    pub fn decrypt_list(&self, ciphers: Vec<Cipher>) -> Result<Vec<CipherListView>, DecryptError> {
        self.0.vault().ciphers().decrypt_list(ciphers)
    }

    /// Decrypt FIDO2 credentials
    ///
    /// # Arguments
    /// - `cipher_view` - Cipher to encrypt containing the FIDO2 credential
    ///
    /// # Returns
    /// - `Ok(Vec<Fido2CredentialView>)` containing the decrypted FIDO2 credentials
    /// - `Err(DecryptError)` if decryption fails
    pub fn decrypt_fido2_credentials(
        &self,
        cipher_view: CipherView,
    ) -> Result<Vec<Fido2CredentialView>, DecryptError> {
        self.0
            .vault()
            .ciphers()
            .decrypt_fido2_credentials(cipher_view)
    }
}
