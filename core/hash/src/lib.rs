/*
Copyright 2018-Present The AfricaOS Authors
This file is part of the AfricaOS library.
The AfricaOS Platform is free software: you can redistribute it and/or modify
it under the terms of the GNU Lesser General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.
The AfricaOS Platform is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
GNU Lesser General Public License for more details.
You should have received a copy of the GNU Lesser General Public License
along with the AfricaOS Platform. If not, see <http://www.gnu.org/licenses/>.
*/

extern crate crypto;

use self::crypto::digest::Digest;
use self::crypto::sha2::Sha256;

pub struct Hasher {}

/*
    @name CalculateSHA256Hash
    @desc calculate the sha256 hash of a given string type
*/
pub trait CalculateSHA256Hash {
    fn calculate_sha256(string: String) -> String;
}

impl CalculateSHA256Hash for Hasher {
    fn calculate_sha256(string_to_hash: String) -> String{
        println!("[HASHING]: {}", string_to_hash);
        let mut hasher = Sha256::new();
        hasher.input_str(string_to_hash.as_str());
        let hex = hasher.result_str();
        hex
    }
}

#[cfg(test)]
mod tests {
    use super::{Hasher, CalculateSHA256Hash};
    #[test]
    fn test_calculateSHA256Hash() {
        let test_string: String = String::from("test");
        let expected_hash: &str = concat!("9f86d081884c7d659a2feaa0c55ad015","a3bf4f1b2b0b822cd15d6c15b0f00a08");
        let actual_hash: String = Hasher::calculate_sha256(test_string);
        assert_eq!(actual_hash,String::from(expected_hash));
    }
}
