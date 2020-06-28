use {crate::util::make_session, serde::Serialize, tide::http::cookies::CookieBuilder};

pub fn build_session_cookie<Session>(
    session: &Session,
    secret_key: &[u8],
) -> Result<CookieBuilder<'static>, serde_json::Error>
where
    Session: Serialize,
{
    let session = make_session(session, secret_key)?;
    let builder = CookieBuilder::new("session", session).path("/");
    Ok(builder)
}
