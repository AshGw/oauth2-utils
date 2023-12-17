use std::borrow::Cow;
use base64::{Engine,DecodeError};
use crate::consts::URLS_B64; 


/// Encodes a given token into a URL-safe Base64 string using the specified Base64 encoding engine.
///
/// # Arguments
///
/// * `token`: A generic type `T` that can be any type convertible to a slice of `u8` bytes. 
///   It represents the token or data that needs to be encoded using URL-safe Base64 encoding.
///
/// # Returns
///
/// A `String` representing the URL-safe Base64 encoding of the input `token`.

pub fn urlsafe_b64encode<T>(token: T) -> String
where
    T: AsRef<[u8]>,
{
    URLS_B64.encode(token)
}


/// The function `urlsafe_b64decode` decodes a URL-safe base64 encoded token and returns the decoded
/// string.
/// 
/// Arguments:
/// 
/// * `token`: The `token` parameter is a generic type `T` that can be any type that can be converted to
/// a slice of `u8` values using the `AsRef<[u8]>` trait. This allows the function to accept various
/// types such as `&[u8]`, `
/// 
/// Returns:
/// 
/// a `Result` type with the following possible outcomes:
/// - If the decoding is successful, it returns an `Ok` variant containing a `Cow<'static, str>` which
/// represents the decoded string.
/// - If there is an error during decoding, it returns an `Err` variant containing a `DecodeError` which
/// represents the specific decoding error.
pub fn urlsafe_b64decode<T>(token: T) -> Result<Cow<'static, str>, DecodeError>
where
    T: AsRef<[u8]>,
{

    let decoded: Vec<u8> = URLS_B64.decode(token)?;
    Ok(Cow::Owned(String::from_utf8_lossy(&decoded).to_string()))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_urlsafe_b64encode() {
        // simple string
        let input_string: &str = "Hello, World!";
        let expected_output: &str = "SGVsbG8sIFdvcmxkIQ";
        let encoded_result: String = urlsafe_b64encode(input_string);
        assert_eq!(encoded_result, expected_output);

        // empty str
        let empty_input: &str = "";
        let empty_expected_output: &str = "";
        let empty_encoded_result: String = urlsafe_b64encode(empty_input);
        assert_eq!(empty_encoded_result, empty_expected_output);

        //  binary data
        let binary_data: Vec<u8> = vec![0x01, 0x02, 0x03, 0x04];
        let binary_expected_output: &str = "AQIDBA";
        let binary_encoded_result: String = urlsafe_b64encode(binary_data);
        assert_eq!(binary_encoded_result, binary_expected_output);
    }


    #[test]
    fn test_urlsafe_b64decode() {
        // Simple string
        let encoded_string: &str = "aGV5eQ";
        let expected_output: &str = "heyy";
        let decoded_result: Result<Cow<'static, str>, DecodeError> = urlsafe_b64decode(encoded_string);
        assert_eq!(decoded_result.unwrap(),expected_output.to_string());

        // empty str
        let empty_encoded: &str = "";
        let empty_expected_output: &str = "";
        let empty_decoded_result: Result<Cow<'static, str>, DecodeError> = urlsafe_b64decode(empty_encoded);
        assert_eq!(empty_decoded_result.unwrap(),empty_expected_output.to_string());

        //  invalid encoding
        let invalid_encoded: &str = "InvalidBase64";
        let invalid_decoded_result: Result<Cow<'static, str>, DecodeError> = urlsafe_b64decode(invalid_encoded);
        assert!(invalid_decoded_result.is_err());
    }
}
