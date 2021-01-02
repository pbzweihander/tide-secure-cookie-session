use {
    serde::{Deserialize, Serialize},
    tide_secure_cookie_session::SecureCookieSessionMiddleware,
};

static SECRET_KEY: &str = "very secure secret key";

#[derive(Debug, Serialize, Deserialize)]
struct MySession {
    name: String,
    count: usize,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let middleware =
        SecureCookieSessionMiddleware::<MySession>::new(SECRET_KEY.as_bytes().to_vec());

    let mut app = tide::new();
    app.with(middleware);
    app.at("/hello").get(hello);
    app.at("/login/:name").get(login);
    app.listen("0.0.0.0:8080").await?;

    Ok(())
}

async fn hello(req: tide::Request<()>) -> tide::Result {
    let session = req.ext::<MySession>();
    if let Some(session) = session {
        let resp = tide::Response::new(tide::StatusCode::Ok);

        let new_session = MySession {
            name: session.name.clone(),
            count: session.count + 1,
        };
        // FIXME
        // use Response::insert_ext
        // https://github.com/http-rs/tide/commit/7f946a9c9bee84c430dda62ebdf736b287fa0797
        let mut resp: tide::http::Response = resp.into();
        resp.ext_mut().insert(new_session);
        let mut resp: tide::Response = resp.into();

        let body = format!(
            "Hello! {}. You visited {} times.",
            session.name, session.count
        );
        resp.set_body(body);

        Ok(resp)
    } else {
        Ok("No session found! Visit /login/<name> first."
            .to_string()
            .into())
    }
}

async fn login(req: tide::Request<()>) -> tide::Result {
    let name = req.param("name").unwrap();
    let session = MySession { name : name.to_string(), count: 0 };
    let resp: tide::Response = tide::Redirect::new("/hello").into();
    // FIXME
    // use Response::insert_ext
    // https://github.com/http-rs/tide/commit/7f946a9c9bee84c430dda62ebdf736b287fa0797
    let mut resp: tide::http::Response = resp.into();
    resp.ext_mut().insert(session);
    let resp = resp.into();
    Ok(resp)
}
