use {
    crate::{cookie::build_session_cookie, util::get_session},
    serde::{de::DeserializeOwned, Serialize},
    tide::{Middleware, Next, Request, Response},
};

pub struct SecureCookieSessionMiddleware<Session> {
    secret_key: Vec<u8>,
    path: String,
    _cookie: std::marker::PhantomData<Session>,
}

impl<S> std::fmt::Debug for SecureCookieSessionMiddleware<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SecureCookieSessionMiddleware")
            .field("path", &self.path)
            .field("secret_key", &"***")
            .finish()
    }
}

impl<S> SecureCookieSessionMiddleware<S> {
    pub fn new(secret_key: Vec<u8>) -> Self {
        SecureCookieSessionMiddleware {
            secret_key,
            path: "/".to_string(),
            _cookie: std::marker::PhantomData,
        }
    }

    pub fn set_path(&mut self, path: &str) -> &mut Self {
        self.path = path.to_string();
        self
    }
}

#[async_trait::async_trait]
impl<State, Session> Middleware<State> for SecureCookieSessionMiddleware<Session>
where
    State: Clone + Send + Sync + 'static,
    Session: Serialize + DeserializeOwned + Send + Sync + 'static,
{
    async fn handle(
        &self,
        mut req: Request<State>,
        next: Next<'_, State>,
    ) -> tide::Result<Response> {
        let session = get_session_from_req::<State, Session>(&req, &self.secret_key);
        if let Some(session) = session {
            req.set_ext(session);
        }
        let mut resp = next.run(req).await;
        if let Some(session) = resp.ext::<Session>() {
            let cookie = build_session_cookie(session, &self.secret_key)?
                .path(self.path.clone())
                .finish();
            resp.insert_cookie(cookie);
        }
        Ok(resp)
    }
}

fn get_session_from_req<S, Session>(req: &Request<S>, secret_key: &[u8]) -> Option<Session>
where
    Session: DeserializeOwned + Send + Sync + 'static,
{
    let cookie = req.cookie("session")?;
    let session = cookie.value();
    get_session(session, secret_key)
}
