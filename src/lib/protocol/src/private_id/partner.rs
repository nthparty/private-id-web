


extern crate csv;

use crypto::{
    eccipher::{gen_scalar, ECCipher, ECRistrettoParallel},
    prelude::*,
};

use crate::{
    fileio::{load_data, KeyedCSV},
    private_id::traits::PartnerPrivateIdProtocol,
};

use common::{
    files,
    permutations::{gen_permute_pattern, permute, undo_permute},
};

use std::sync::{Arc, RwLock};

use super::{fill_permute, ProtocolError};

pub struct PartnerPrivateId {
    private_keys: (Scalar, Scalar),
    ec_cipher: ECRistrettoParallel,
    plain_data: Arc<RwLock<KeyedCSV>>,
    permutation: Arc<RwLock<Vec<usize>>>,
    id_map: Arc<RwLock<Vec<Vec<String>>>>,
}

impl PartnerPrivateId {
    pub fn new() -> PartnerPrivateId {
        PartnerPrivateId {
            private_keys: (gen_scalar(), gen_scalar()),
            ec_cipher: ECRistrettoParallel::default(),
            plain_data: Arc::new(RwLock::default()),
            permutation: Arc::new(RwLock::default()),
            id_map: Arc::new(RwLock::default()),
        }
    }

    pub fn load_data(&self, path: &str, input_with_headers: bool) -> Result<bool, ProtocolError> {
        Ok(load_data(self.plain_data.clone(), path, input_with_headers))
    }

    pub fn get_size(&self) -> usize {
        self.plain_data.clone().read().unwrap().records.len()
    }
}

impl Default for PartnerPrivateId {
    fn default() -> Self {
        Self::new()
    }
}

impl PartnerPrivateIdProtocol for PartnerPrivateId {
    fn gen_permute_pattern(&self) -> Result<(), ProtocolError> {
        fill_permute(
            self.permutation.clone(),
            (*self.plain_data.clone().read().unwrap()).records.len(),
        );
        Ok(())
    }

    fn permute_hash_to_bytes(&self) -> Result<Bytes, ProtocolError> {
        match self.plain_data.clone().read() {
            Ok(pdata) => {
                let plain_keys = pdata.get_plain_keys();
                let mut u = self
                    .ec_cipher
                    .hash_encrypt_to_bytes(&plain_keys.as_slice(), &self.private_keys.0);

                self.permutation
                    .clone()
                    .read()
                    .map(|pm| {
                        permute(&pm, &mut u);
                        u
                    })
                    .map_err(|err| {
                        error!("error in permute {}", err);
                        ProtocolError::ErrorEncryption("unable to encrypt data".to_string())
                    })
            }

            Err(e) => {
                error!("Unable to encrypt plain_data: {}", e);
                Err(ProtocolError::ErrorEncryption(
                    "unable to encrypt data".to_string(),
                ))
            }
        }
    }

    //TODO: return result
    fn encrypt_permute(&self, company: Bytes) -> (Bytes, Bytes) {
        let mut encrypt_company = self
            .ec_cipher
            .to_points_encrypt(&company, &self.private_keys.0);
        let v_company = self
            .ec_cipher
            .encrypt_to_bytes(&encrypt_company, &self.private_keys.1);
        {
            let rand_permutation = gen_permute_pattern(encrypt_company.len());
            // TODO: BUG why is this undo_permute
            // undo_permute(&rand_permutation, &mut e_company_dsrlz);
            permute(&rand_permutation, &mut encrypt_company);
        }
        (self.ec_cipher.to_bytes(&encrypt_company), v_company)
    }

    fn encrypt(&self, partner: Bytes) -> Result<Bytes, ProtocolError> {
        let ep = self
            .ec_cipher
            .to_points_encrypt(&partner, &self.private_keys.1);
        Ok(self.ec_cipher.to_bytes(&ep))
    }

    fn create_id_map(&self, partner: Bytes, company: Bytes, na_val: Option<&str>) {
        match (
            self.permutation.clone().read(),
            self.plain_data.clone().read(),
            self.id_map.clone().write(),
        ) {
            (Ok(pm), Ok(plain_data), Ok(mut id_map)) => {
                let mut partner_encrypt = self
                    .ec_cipher
                    .to_points_encrypt(&partner, &self.private_keys.1);
                undo_permute(&pm, &mut partner_encrypt);

                for (k, v) in self
                    .ec_cipher
                    .to_bytes(&partner_encrypt)
                    .iter()
                    .zip(plain_data.get_plain_keys().iter())
                {
                    let record = plain_data.get_record_with_keys(k.to_string(), &v);
                    id_map.push(record);
                }

                for k in self
                    .ec_cipher
                    .to_bytes(
                        &self
                            .ec_cipher
                            .to_points_encrypt(&company, &self.private_keys.1),
                    )
                    .iter()
                {
                    let record = plain_data.get_empty_record_with_key(
                        k.to_string(),
                        na_val,
                    );
                    id_map.push(record);
                }

                if !plain_data.headers.is_empty() {
                    id_map.insert(0, plain_data.headers.clone());
                }
            }
            _ => panic!("Cannot make v"),
        }
    }

    fn print_id_map(&self, limit: usize, input_with_headers: bool, use_row_numbers: bool) {
        let _ = self
            .id_map
            .clone()
            .read()
            .map(|data| {
                files::write_vec_to_stdout(&data, limit, input_with_headers, use_row_numbers)
                    .unwrap()
            })
            .map_err(|_| {});
    }

    fn stringify_id_map(&self, use_row_numbers: bool) -> String {
        let mut output = "".to_owned();
        let _ = self
            .id_map
            .clone()
            .read()
            .map(|view_map| {
                let mut slice = view_map.iter().collect::<Vec<_>>();
                if slice.iter().any(|vec| vec.is_empty()) {
                    panic!("Got empty rows to print out");
                }
                slice[0..].sort_by(|a, b| a[0].cmp(&b[0]));

                output.push_str("-----BEGIN FULL VIEW-----");
                output.push_str("\n");
                for (i, line) in slice.iter().enumerate() {
                    let mut record = line.to_vec();
                    if use_row_numbers {
                        record[0] = i.to_string();
                    }
                    output.push_str(&format!("{}", record.join("\t")));
                    output.push_str("\n");
                }
                output.push_str("-----END FULL VIEW-----");
                output.push_str("\n");
            });
        output
    }

    // fn save_id_map(
    //     &self,
    //     path: &str,
    //     input_with_headers: bool,
    //     use_row_numbers: bool,
    // ) -> Result<(), ProtocolError> {
    //     self.id_map
    //         .clone()
    //         .write()
    //         .map(|mut data| {
    //             files::write_vec_to_csv(&mut data, path, input_with_headers, use_row_numbers)
    //                 .unwrap();
    //         })
    //         .map_err(|_| ProtocolError::ErrorIO("Unable to write partner view to file".to_string()))
    // }
}
