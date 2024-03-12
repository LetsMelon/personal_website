use hmac::{Hmac, Mac};
use sha2::Sha256;

pub fn call(secret: &[u8], signature: &[u8], payload: &[u8]) -> anyhow::Result<()> {
    let mut mac = Hmac::<Sha256>::new_from_slice(secret)?;
    mac.update(payload);

    Ok(mac.verify_slice(signature)?)
}

#[test]
fn test_verify() {
    // ! Test from github
    // TODO add link to github page where this test is from
    let verify_out = call(
        b"It's a Secret to Everybody",
        &hex_literal::hex!("757107ea0eb2509fc211221cce984b8a37570b6d7586c22c46f4379c8b043e17"),
        b"Hello, World!",
    );

    assert!(verify_out.is_ok());
}
