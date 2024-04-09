//! ## Overview
//!
//! This crate provides utility functions for working with OAuth2:
//! - PKCE
//! - URL-safe tokens
//! - URL-safe base64 encoding/decoding
//!
//! ## Installation
//! ```shell
//! cargo add oauth2_utils
//! ```
//!
//! ## Examples
//!
//! To generate a PKCE pair with the corresponding method:
//!
//! ```rust
//! use oauth2_utils::pkce::PKCE;
//!
//! let pkce = PKCE::new();
//! println!("PKCE Code Challenge: {}", pkce.code_challenge);
//! println!("PKCE Code Verifier: {}", pkce.code_verifier);
//! println!("PKCE Code method: {}", pkce.method);
//! ```
//!
//! To generate a code verifier with a custom length:
//!
//! ```rust
//! use oauth2_utils::errors::CodeVerifierError;
//! use oauth2_utils::pkce::gen::{gen_code_challenge, gen_code_verifier};
//!
//!
//! let code_verifier = gen_code_verifier(Some(128))?; if `None` then defaults to 98
//! eprintln!("Code Verifier: {}", code_verifier);
//! let code_challenge = gen_code_challenge(&code_verifier);
//! eprintln!("Code Challenge: {}", code_challenge);

//! ```
//!
//! To generate a URL-safe token for Nonce, State, etc.:
//!
//! ```rust
//! use oauth2_utils::urlsafe::urlsafe_token;
//!

//! println!("URL-safe Token: {}", urlsafe_token(32)) // of length 32;
//! ```
//!
//! For base64 encoding/decoding operations:
//!
//! ```rust
//! use oauth2_utils::errors::B64Error;
//! use oauth2_utils::urlsafe::b64::{urlsafe_b64decode, urlsafe_b64encode};
//!
//!
//! let a: String = String::from("some value");
//! let encoded = urlsafe_b64encode(a);
//! println!("{}", encoded);
//! let decoded = urlsafe_b64decode(encoded)?;
//! println!("{}", decoded);
//! ```

pub mod consts;
pub mod errors;
pub mod pkce;
pub mod urlsafe;
