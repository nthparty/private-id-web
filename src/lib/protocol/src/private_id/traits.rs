


extern crate crypto;

use crate::private_id::ProtocolError;
use crypto::prelude::Bytes;

pub trait PartnerPrivateIdProtocol {
    fn gen_permute_pattern(&self) -> Result<(), ProtocolError>;

    fn permute_hash_to_bytes(&self) -> Result<Bytes, ProtocolError>;
    fn encrypt_permute(&self, company: Bytes) -> (Bytes, Bytes);
    fn encrypt(&self, partner: Bytes) -> Result<Bytes, ProtocolError>;

    fn create_id_map(&self, partner: Bytes, company: Bytes, na_val: Option<&str>);
    fn print_id_map(&self, limit: usize, input_with_headers: bool, use_row_numbers: bool);
    fn stringify_id_map(&self, use_row_numbers: bool) -> String;
    // fn save_id_map(
    //     &self,
    //     path: &str,
    //     input_with_headers: bool,
    //     use_row_numbers: bool,
    // ) -> Result<(), ProtocolError>;
}

pub trait CompanyPrivateIdProtocol {
    fn set_encrypted_company(&self, name: String, data: Bytes) -> Result<(), ProtocolError>;
    fn set_encrypted_partner_keys(&self, u_partner_payload: Bytes) -> Result<(), ProtocolError>;

    fn get_permuted_keys(&self) -> Result<Bytes, ProtocolError>;
    fn get_encrypted_partner_keys(&self) -> Result<Bytes, ProtocolError>;

    fn calculate_set_diff(&self) -> Result<(), ProtocolError>;
    fn get_set_diff_output(&self, name: String) -> Result<Bytes, ProtocolError>;

    fn write_company_to_id_map(&self);
    fn write_partner_to_id_map(
        &self,
        s_prime_partner_payload: Bytes,
        na_val: Option<&str>,
    ) -> Result<(), ProtocolError>;

    fn print_id_map(&self, limit: usize, input_with_headers: bool, use_row_numbers: bool);
    fn stringify_id_map(&self, use_row_numbers: bool) -> String;
    // fn save_id_map(
    //     &self,
    //     path: &str,
    //     input_with_headers: bool,
    //     use_row_numbers: bool,
    // ) -> Result<(), ProtocolError>;
}
