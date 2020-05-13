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
use ring::{
    rand,
    signature::{self, KeyPair}
};

use std::path::Path;
use std::convert::TryInto;
use encode::{Encoder, RawBytesDecode, RawBytesEncode};
use std::io::{Error, ErrorKind};

#[derive(Clone, Debug, PartialEq)]
pub enum SignatureType {
    RSA,
    ED25519
}

#[derive(Clone, Debug, PartialEq)]
pub enum SignatureFormat {
    BYTES,
    DISK
}


/*
@name Verifier for Signature
@description Signing and verifying with RSA (PKCS#1 1.5 padding)
*/


#[derive(Debug)]
enum SignatureError {
   IO(std::io::Error),
   BadPrivateKey,
   OOM,
   BadSignature,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DigitalSignature {
    signature_type: SignatureType,
    format_type: SignatureFormat
}

trait SignVerify {
    fn sign_and_verify(signature_type: SignatureType, format_type: SignatureFormat) -> bool;
}

impl SignVerify for DigitalSignature {
    fn sign_and_verify(signature_type: SignatureType, format_type: SignatureFormat) -> bool {
        let sig: DigitalSignature = DigitalSignature {
            signature_type: signature_type,
            format_type: format_type
        };
        true
    }
}

/*
@name
@description
*/
pub struct Signature {

}


/*
@name Signer
@description
*/
trait Signer {
    /*
    @name sign
    @description
    */
    fn sign(signature: DigitalSignature, content: &'static str) -> Result<String, SignatureError>;

    /*
    @name sign_RSA
    @description
    */
    fn sign_RSA(private_key_path: &std::path::Path, public_key_path: &std::path::Path, content: &'static str) -> Result<String, SignatureError>;

    /*
    @name sign_Ed25519
    @description
    */
    fn sign_Ed25519() -> Result<(), ring::error::Unspecified>;
}

impl Signer for Signature {
    fn sign(signature: DigitalSignature, content: &'static str) -> Result<String, SignatureError>{
        match signature.signature_type {
            RSA => {
                let private_key_path = Path::new("./keys/private.der");
                let public_key_path = Path::new("./keys/public.der");

                Self::sign_RSA(private_key_path, public_key_path, content)
            },
            ED25519 => {
                //Self::sign_Ed25519();
                Ok(String::from("dssdds"))
            }
        }
    }

    fn sign_RSA(private_key_path: &std::path::Path, public_key_path: &std::path::Path, content: &'static str) -> Result<String, SignatureError> {
       // Create an `RsaKeyPair` from the DER-encoded bytes. This example uses
       // a 2048-bit key, but larger keys are also supported.
       let private_key_der: Vec<u8> = read_file(private_key_path)?;

       let key_pair = signature::RsaKeyPair::from_der(&private_key_der).map_err(|_| SignatureError::BadPrivateKey)?;

       // Sign the message, using PKCS#1 v1.5 padding and the
       // SHA256 digest algorithm.
       let content_string: &'static str = content; //"HELLO WORKD";

       // get length of content
       let i: usize = content_string.len();

       //convert str into bytes
       let bytes_string: &[u8] = content_string.as_bytes();
       //.try_into().expect("sign_RSA: could not convert content_string into array slice");

       //create a box
       let bytes_box: Box<&[u8]> = Box::from(bytes_string);

       //leak box for &'static
       let MESSAGE: &'static [u8] = Box::leak(bytes_box);

       let rng = rand::SystemRandom::new();

       //Returns the length in bytes of the key pair's public modulus.
       let mut signature: Vec<u8> = vec![0; key_pair.public_modulus_len()];

       //key_pair.sign(&signature.clone()::RSA_PKCS1_SHA256, &rng, MESSAGE, &mut signature.clone()).map_err(|_| SignatureError::OOM)?;
       key_pair.sign(&signature::RSA_PKCS1_SHA256, &rng, MESSAGE, &mut signature).map_err(|_| SignatureError::OOM)?;

       match Encoder::bytes_to_base64( signature ) {
           Ok(sig) => {
               println!("SUCCCESS: {}", sig);
               Ok( sig )
           },
           Err(error_message) => {
               println!("ERROR: {}", error_message);

               // errors can be created from strings
               let encoder_bytes_to_base64_error = Error::new(ErrorKind::Other, "Encoder::bytes_to_base64( signature ) returned error");
               Err(SignatureError::IO(encoder_bytes_to_base64_error))
           }
       }

       //key_pair.sign(&signature::RSA_PKCS1_SHA256, &rng, MESSAGE, &mut signature).map_err(|_| SignatureError::OOM)?;

       // Verify the signature.
       //let public_key = signature::UnparsedPublicKey::new(&signature::RSA_PKCS1_2048_8192_SHA256, read_file(public_key_path)?);

       //public_key.verify(MESSAGE, &signature).map_err(|_| SignatureError::BadSignature)

    }

    fn sign_Ed25519() -> Result<(), ring::error::Unspecified> {
        let rng = rand::SystemRandom::new();
        let pkcs8_bytes = signature::Ed25519KeyPair::generate_pkcs8(&rng)?;

        // Normally the application would store the PKCS#8 file persistently. Later
        // it would read the PKCS#8 file from persistent storage to use it.

        let key_pair = signature::Ed25519KeyPair::from_pkcs8(pkcs8_bytes.as_ref())?;

        // Sign the message "hello, world".
        const MESSAGE: &[u8] = b"hello, world";
        let sig = key_pair.sign(MESSAGE);

        // Normally an application would extract the bytes of the signature and
        // send them in a protocol message to the peer(s). Here we just get the
        // public key key directly from the key pair.
        let peer_public_key_bytes = key_pair.public_key().as_ref();

        // Verify the signature of the message using the public key. Normally the
        // verifier of the message would parse the inputs to this code out of the
        // protocol message(s) sent by the signer.
        let peer_public_key = signature::UnparsedPublicKey::new(&signature::ED25519, peer_public_key_bytes);
        peer_public_key.verify(MESSAGE, sig.as_ref())?;

        Ok(())
    }
}



/*
@name Verifier
@description
*/
trait Verifier {
    /*
    @name verify
    @description
    */
    fn verify(ds: DigitalSignature, signature: String, content: &'static str) -> Result<String, SignatureError>;

    /*
    @name verify_rsa
    @description
    */
    fn verify_rsa(signature: Vec<u8>, content: &'static str) -> Result<String, SignatureError>;
}

/*
@name Verifier for Signature
@description
*/
impl Verifier for Signature {

    fn verify(ds: DigitalSignature, signature_string: String, content: &'static str) -> Result<String, SignatureError> {

        match ds.signature_type {
            RSA => {

                // TODO: convert base64 to bytes
                let mut signature_bytes: Result<Vec<u8>, String> = Encoder::base64_to_bytes(signature_string);

                match signature_bytes {
                    Ok(sig) => {
                        return Self::verify_rsa(sig, content)
                    },
                    Err(msg) => {
                        let verify_sig_error = Error::new(ErrorKind::Other, "verify: signature_bytes returned error");
                        return Err(SignatureError::IO(verify_sig_error))
                    }
                }

            },
            ED25519 => {
                //Self::sign_Ed25519();
                // Ok(String::from("dssdds"))
                let unsupported_sig_error = Error::new(ErrorKind::Other, "unsupported signature type");
                return Err(SignatureError::IO(unsupported_sig_error))
            }
        }

    }

    fn verify_rsa(signature: Vec<u8>, content: &'static str) -> Result<String, SignatureError> {

        let content_string: &'static str = content;
        // get length of content
        let i: usize = content_string.len();
        //convert str into bytes
        let bytes_string: &[u8] = content_string.as_bytes();
        //create a box
        let bytes_box: Box<&[u8]> = Box::from(bytes_string);
        //leak box for &'static
        let MESSAGE: &'static [u8] = Box::leak(bytes_box);

        //TODO: get public key by bytes
        let public_key_path = Path::new("./keys/public.der");
        //let public_key_bytes: Vec<u8> = read_file(public_key_path)?;

        let public_key_base64: &str = "MIIBCgKCAQEAuJ3hhGpo+nInkqHpgBx3E5eihx0IGNVit5u0UlvLHmnW2PJ3HqFyafr/eYaas7VetW5Ss5kAWZmH3oED7n4xVlXrUeFlSwShSDVMKyT3iK1et1KQIX5cqp2CiFSs+xi5eJOEEWZxoexYbSTg0Rg34gzob+VqHZzRtRN9eTja7ZCE3/m3cMlWfb7yL2jyN521ZL02QG9PZ4EYenDTM2xcWvCZrWtKUFLahCWWQ1H8ZpoWg/y/tenvUy5YnWLrbhbSZqMJQYfnUwr8FRWZsiI2RWRtXlx9X6bxvaOoG/h4IOhV/G52Xas2WxxUbZ5rgRPziA/mvkJxytHkZPeEnpRcawIDAQAB";

        match Encoder::base64_to_bytes( String::from(public_key_base64) ) {
            Ok(public_key_bytes) => {
                //assert_eq!("dsd",base64_encoded)
                //public_key_bytes
                let public_key = signature::UnparsedPublicKey::new(&signature::RSA_PKCS1_2048_8192_SHA256, public_key_bytes);
                match public_key.verify(MESSAGE, &signature).map_err(|_| SignatureError::BadSignature) {
                    Ok(_) => Ok(String::from("Verification ok")),
                    Err(_) => Err(SignatureError::BadSignature)
                }
            },
            Err(error_message) => {
                println!("ERROR: {}", error_message);
                 Err(SignatureError::BadSignature)
            }
        }

    }

}



fn read_file(path: &std::path::Path) -> Result<Vec<u8>, SignatureError> {
    use std::io::Read;
    let mut file = std::fs::File::open(path).map_err(|e| SignatureError::IO(e))?;
    let mut contents: Vec<u8> = Vec::new();
    file.read_to_end(&mut contents).map_err(|e| SignatureError::IO(e))?;
    Ok(contents)
}


#[cfg(test)]
mod tests {
    use ring::{
        rand,
        signature::{self, KeyPair},
    };

    use super::{SignatureType,
                SignatureFormat,
                DigitalSignature,
                Signer,
                Verifier,
                Signature,
                SignatureError,
                RawBytesDecode,
                read_file};

    use encode::{Encoder, RawBytesEncode};
    use std::path::Path;


    #[test] // Signing and verifying with RSA
    fn test_sign_rsa_from_disk(){

        let digital_signature: DigitalSignature = DigitalSignature {
            signature_type: SignatureType::RSA,
            format_type: SignatureFormat::DISK
        };

        let content: &str = "TEST";
        let signature_result: &str = "opEssZ7CaoYvtZJErFPqiB0L+lxwFm1/YT3tLZ+07fCnwWvuRcXtpwmo4esdNs05OItDBK6SZaxVPO+tKG22NC8R64DQj4J6CXpt4XMxtGJSUeY9MyZB6eyW8qYye7zascGv5+Eht4VJ5Zu9TX8Xl2+oyZA+3RYw5QKHvMgHyN0mpPU8PYpBDdVKg5Nglh4WOjqrvJF/EAdyqfeLN0CNJHeFwwjlkDaOz1x9LBOBf8c5HhDulgblSd4tlJ9zRA97SbnxmQtip/XDLweTtCx9vmjFd0tw/JTcfl2V87r+JgxL0r9EgEoFsexs3XkdqKZ2LzypPMvp0XqeoNEJ03g96A==";

        // sign for testing purposes
        let test: Result<String, SignatureError> = Signature::sign(digital_signature, content);

        // assert signatures match
        assert_eq!( test.unwrap(), String::from(signature_result) );

    }

    #[test]
    fn test_verify_rsa_from_disk(){

        let digital_signature: DigitalSignature = DigitalSignature {
            signature_type: SignatureType::RSA,
            format_type: SignatureFormat::DISK
        };

        let content: &str = "TEST";
        let signature_result: &str = "opEssZ7CaoYvtZJErFPqiB0L+lxwFm1/YT3tLZ+07fCnwWvuRcXtpwmo4esdNs05OItDBK6SZaxVPO+tKG22NC8R64DQj4J6CXpt4XMxtGJSUeY9MyZB6eyW8qYye7zascGv5+Eht4VJ5Zu9TX8Xl2+oyZA+3RYw5QKHvMgHyN0mpPU8PYpBDdVKg5Nglh4WOjqrvJF/EAdyqfeLN0CNJHeFwwjlkDaOz1x9LBOBf8c5HhDulgblSd4tlJ9zRA97SbnxmQtip/XDLweTtCx9vmjFd0tw/JTcfl2V87r+JgxL0r9EgEoFsexs3XkdqKZ2LzypPMvp0XqeoNEJ03g96A==";
        let public_key_base64: &str = "MIIBCgKCAQEAuJ3hhGpo+nInkqHpgBx3E5eihx0IGNVit5u0UlvLHmnW2PJ3HqFyafr/eYaas7VetW5Ss5kAWZmH3oED7n4xVlXrUeFlSwShSDVMKyT3iK1et1KQIX5cqp2CiFSs+xi5eJOEEWZxoexYbSTg0Rg34gzob+VqHZzRtRN9eTja7ZCE3/m3cMlWfb7yL2jyN521ZL02QG9PZ4EYenDTM2xcWvCZrWtKUFLahCWWQ1H8ZpoWg/y/tenvUy5YnWLrbhbSZqMJQYfnUwr8FRWZsiI2RWRtXlx9X6bxvaOoG/h4IOhV/G52Xas2WxxUbZ5rgRPziA/mvkJxytHkZPeEnpRcawIDAQAB";


        match Encoder::base64_to_bytes( String::from(public_key_base64) ) {
            Ok(public_key_bytes) => {
                let verification_result: Result<String, SignatureError> = Signature::verify(digital_signature, String::from(signature_result), content);

                match verification_result {
                    Ok(result) => assert_eq!( String::from("Verification ok"), result ),
                    Err(_) => assert!(false)
                }
            },
            Err(error_message) => {
                println!("ERROR: {}", error_message);
                assert!(false)
            }
        }
    }

    #[test]
    fn test_der_bytes(){
        /*
            MIIBCgKCAQEAuJ3hhGpo+nInkqHpgBx3E5eihx0IGNVit5u0UlvLHmnW2PJ3HqFyafr/eYaas7VetW5Ss5kAWZmH3oED7n4xVlXrUeFlSwShSDVMKyT3iK1et1KQIX5cqp2CiFSs+xi5eJOEEWZxoexYbSTg0Rg34gzob+VqHZzRtRN9eTja7ZCE3/m3cMlWfb7yL2jyN521ZL02QG9PZ4EYenDTM2xcWvCZrWtKUFLahCWWQ1H8ZpoWg/y/tenvUy5YnWLrbhbSZqMJQYfnUwr8FRWZsiI2RWRtXlx9X6bxvaOoG/h4IOhV/G52Xas2WxxUbZ5rgRPziA/mvkJxytHkZPeEnpRcawIDAQAB
        */
        // let expected_der_base64: String = String::from("MIIBCgKCAQEAuJ3hhGpo+nInkqHpgBx3E5eihx0IGNVit5u0UlvLHmnW2PJ3HqFyafr/eYaas7VetW5Ss5kAWZmH3oED7n4xVlXrUeFlSwShSDVMKyT3iK1et1KQIX5cqp2CiFSs+xi5eJOEEWZxoexYbSTg0Rg34gzob+VqHZzRtRN9eTja7ZCE3/m3cMlWfb7yL2jyN521ZL02QG9PZ4EYenDTM2xcWvCZrWtKUFLahCWWQ1H8ZpoWg/y/tenvUy5YnWLrbhbSZqMJQYfnUwr8FRWZsiI2RWRtXlx9X6bxvaOoG/h4IOhV/G52Xas2WxxUbZ5rgRPziA/mvkJxytHkZPeEnpRcawIDAQAB");
        // let public_key_path = Path::new("./keys/public.der");
        // let public_key_bytes: Vec<u8> = read_file(public_key_path).unwrap();
        // match Encoder::base64_to_bytes( public_key_bytes ) {
        //     Ok(base64_encoded) => {
        //         assert_eq!(expected_der_base64, base64_encoded)
        //     },
        //     Err(error_message) => {
        //         println!("ERROR: {}", error_message);
        //     }
        // }
    }

}
