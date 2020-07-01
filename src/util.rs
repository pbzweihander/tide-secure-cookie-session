use {
    hmac::{Hmac, Mac, NewMac},
    serde::{de::DeserializeOwned, Serialize},
    sha1::Sha1,
};

pub fn sign(secret_key: &[u8], message: &[u8]) -> Vec<u8> {
    let mut mac = Hmac::<Sha1>::new_varkey(secret_key).unwrap();
    mac.update(message);
    let result = mac.finalize();
    result.into_bytes().as_slice().to_vec()
}

pub fn verify(secret_key: &[u8], message: &[u8], signature: &[u8]) -> bool {
    let expected = sign(secret_key, message);
    expected == signature
}

pub(crate) fn get_session<T: DeserializeOwned>(session: &str, secret_key: &[u8]) -> Option<T> {
    let i = session.find(".")?;
    let (payload, sign) = session.split_at(i);
    if payload.is_empty() || sign.is_empty() {
        return None;
    }
    let (_, sign) = sign.split_at(1);
    if sign.is_empty() {
        return None;
    }
    let sign = base64::decode(sign).ok()?;
    if !verify(secret_key, payload.as_bytes(), &sign) {
        return None;
    }
    let payload = base64::decode(payload).ok()?;
    serde_json::from_slice(&payload).ok()
}

pub(crate) fn make_session<T: Serialize>(
    payload: &T,
    secret_key: &[u8],
) -> Result<String, serde_json::Error> {
    let payload = serde_json::to_vec(payload)?;
    let payload = base64::encode(payload);
    let sign = sign(secret_key, payload.as_bytes());
    let sign = base64::encode(sign);
    Ok(format!("{}.{}", payload, sign))
}
