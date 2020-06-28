use {
    crate::util::get_session,
    futures::future::BoxFuture,
    serde::de::DeserializeOwned,
    tide::{Middleware, Next, Request, Response},
};

pub struct SecureCookieSessionMiddleware<Session> {
    secret_key: Vec<u8>,
    _cookie: std::marker::PhantomData<Session>,
}

impl<S> std::fmt::Debug for SecureCookieSessionMiddleware<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SecureCookieMiddleware")
            .field("secret_key", &"***")
            .finish()
    }
}

impl<S> SecureCookieSessionMiddleware<S> {
    pub fn new(secret_key: Vec<u8>) -> Self {
        SecureCookieSessionMiddleware {
            secret_key,
            _cookie: std::marker::PhantomData,
        }
    }
}

impl<State, Session> Middleware<State> for SecureCookieSessionMiddleware<Session>
where
    State: Send + Sync + 'static,
    Session: DeserializeOwned + Send + Sync + 'static,
{
    fn handle<'a>(
        &'a self,
        mut req: Request<State>,
        next: Next<'a, State>,
    ) -> BoxFuture<'a, tide::Result<Response>> {
        Box::pin(async move {
            let session = get_session_from_req::<State, Session>(&req, &self.secret_key);
            if let Some(session) = session {
                req.set_ext(session);
            }
            next.run(req).await
        })
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
