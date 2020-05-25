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

extern crate base64;

use std::str;
use base64::{encode, decode};

pub struct Encoder {}
pub trait RawBytesEncode {
    /*
    @name encode_rawbytes
    @description
    */
    fn encode_rawbytes(string_to_encode: String) -> Vec<u8>;

    /*
    @name bytes_to_base64
    @description
    */
    fn bytes_to_base64(bytes: Vec<u8>) -> Result<String, String>;
}

impl RawBytesEncode for Encoder {
    fn encode_rawbytes(mut string_to_encode: String) -> Vec<u8> {
        string_to_encode.as_bytes().to_vec()
    }

    fn bytes_to_base64(bytes: Vec<u8>) -> Result<String, String> {
        //let result = base64::encode(b"hello world");
        let result = base64::encode(&bytes);
        Ok(result)
    }
}

pub trait RawBytesDecode {
    /*
    @name decode_rawbytes
    @description convert from Vec<u8> to string
    */
    fn decode_rawbytes(bytes: Vec<u8>) -> Result<String, String>;

    /*
    @name base64_to_bytes
    @description
    */
    fn base64_to_bytes(encoded_string: String) -> Result<Vec<u8>, String>;
}

impl RawBytesDecode for Encoder {
    fn decode_rawbytes(bytes: Vec<u8>) -> Result<String, String> {
        let converted_to_string: Result<String, std::string::FromUtf8Error> = String::from_utf8(bytes);
        match converted_to_string {
            Ok(result) => {
                Ok(result)
            },
            Err(_) => {
                Err(String::from("Could not convert raw bytes to string"))
            }
        }
    }

    fn base64_to_bytes(encoded_string: String) -> Result<Vec<u8>, String>{
        //let decoded_bytes: Vec<u8> = Encoder::encode_rawbytes(bytes_as_string);
        let decoded_bytes: Result<Vec<u8>, base64::DecodeError> = decode(&encoded_string);
        match decoded_bytes {
            Ok(decoded) => {
                Ok(decoded)
            },
            Err(_) => {
                Err( String::from("Could not convert base64 to bytes") )
            }
        }
    }
}

pub trait Base64Encode {
    /*
    @name encode_base64
    @desc encode a string to its base64 representation
    */
    fn encode_base64(bytes_as_string: String) -> Result<String,String>;
}

impl Base64Encode for Encoder {
    fn encode_base64(bytes_as_string: String) -> Result<String,String> {
        let decoded_bytes: Vec<u8> = Encoder::encode_rawbytes(bytes_as_string);
        let encoded: String = encode(&decoded_bytes);
        Ok(encoded)
    }
}

pub trait Base64Decode {
    /*
    @name decode_base64
    @desc convert a base64 encoded string back to its origin
    */
    fn decode_base64(encoded: String) -> Result<String, String>;
}

impl Base64Decode for Encoder {
    fn decode_base64(encoded: String) -> Result<String, String> {
        println!("decode_base64(), base64 encoded: {}", encoded);
        let decoded_result: Result<Vec<u8>, base64::DecodeError> = decode(&encoded);
        match decoded_result {
            Ok(decoded) => {
                let decode_from_utf8: Result<&str, std::str::Utf8Error> = str::from_utf8( &decoded );
                match decode_from_utf8 {
                    Ok(result) => {
                        let decoded_result_w_quotes: String = String::from(result);
                        //TODO: replace only the first and last quotes, not all, since its A json object
                        let decoded_result_wo_quotes: &str = decoded_result_w_quotes.as_str().trim_matches('\"');
                        let final_decoded_result_after_trim: String = String::from(decoded_result_wo_quotes);
                        println!("decoded_result: {}", final_decoded_result_after_trim);
                        Ok(final_decoded_result_after_trim)
                    },
                    Err(error) => {
                        Err(String::from("Base64encode, encode(), Could not UTF8 Decode string"))
                    }
                }
            },
            Err(e) => {
                Err(String::from("Base64encode, encode(), decoded_result is ERR"))
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::{Encoder,
                Base64Encode,
                Base64Decode,
                RawBytesEncode,
                RawBytesDecode};

    #[test]
    fn test_encode_base64_string(){
        let test_string_bytes = String::from("example");
        let expected_result = String::from("ZXhhbXBsZQ==");
        let encoded_result = Encoder::encode_base64(test_string_bytes);
        if encoded_result.is_ok() {
            assert_eq!(expected_result, encoded_result.unwrap());
        }
    }

    #[test]
    fn test_decode_base64_string(){
        let test_string_bytes = String::from("ZXhhbXBsZQ==");
        let expected_result = String::from("example");
        let encoded_result = Encoder::decode_base64(test_string_bytes);
        if encoded_result.is_ok() {
            assert_eq!(expected_result, encoded_result.unwrap());
        }
    }

    #[test]
    fn test_decode_base64_to_bytes(){
        let test_string_bytes = String::from("ZXhhbXBsZQ==");
        let expected_byte_array_result: Vec<u8> = vec![0x65u8, 0x78u8, 0x61u8, 0x6Du8, 0x70u8, 0x6Cu8, 0x65u8];
        let encoded_result = Encoder::base64_to_bytes(test_string_bytes);
        match encoded_result {
            Ok(result) => {
                assert_eq!(result, expected_byte_array_result);
            },
            Err(message) => {
                assert!(false);
            }
        }
    }

    #[test]
    fn test_string_to_rawbytes(){
        let starting_string: String = String::from("example");
        let expected_byte_array: Vec<u8> = vec![0x65u8, 0x78u8, 0x61u8, 0x6Du8, 0x70u8, 0x6Cu8, 0x65u8];
        let actual_result: Vec<u8> = Encoder::encode_rawbytes(starting_string);
        assert_eq!(actual_result, expected_byte_array);
    }

    #[test]
    fn test_bytes_to_string(){
        let starting_bytes: Vec<u8> = vec![0x65u8, 0x78u8, 0x61u8, 0x6Du8, 0x70u8, 0x6Cu8, 0x65u8];
        let expected_string: String = String::from("example");
        let actual_result: Result<String, String> = Encoder::decode_rawbytes(starting_bytes);
        assert_eq!(actual_result, Ok(expected_string));
    }

    #[test]
    fn test_json_decode_with_quotes(){
        let starting_string: String = String::from("{\"test\":\"test\"}");
        let encoded_result: Result<String,String> = Encoder::encode_base64(starting_string.clone());
        if encoded_result.is_ok() {
            let decoded_result: Result<String, String> = Encoder::decode_base64(encoded_result.unwrap());
            assert!(decoded_result.clone().is_ok());
            assert_eq!(starting_string, decoded_result.unwrap());
        }
    }
}
