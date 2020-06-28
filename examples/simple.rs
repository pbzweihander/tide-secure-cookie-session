use {
    serde::{Deserialize, Serialize},
    tide_secure_cookie_session::{build_session_cookie, SecureCookieSessionMiddleware},
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
    app.middleware(middleware);
    app.at("/hello").get(hello);
    app.at("/login/:name").get(login);
    app.listen("0.0.0.0:8080").await?;

    Ok(())
}

async fn hello(req: tide::Request<()>) -> tide::Result {
    let session = req.ext::<MySession>();
    if let Some(session) = session {
        let new_session = MySession {
            name: session.name.clone(),
            count: session.count + 1,
        };
        let cookie = build_session_cookie(&new_session, SECRET_KEY.as_bytes())?.finish();
        let body = format!(
            "Hello! {}. You visited {} times.",
            session.name, session.count
        );
        let mut resp = tide::Response::new(tide::StatusCode::Ok);
        resp.set_body(body);
        resp.insert_cookie(cookie);
        Ok(resp)
    } else {
        Ok("No session found! Visit /login/<name> first."
            .to_string()
            .into())
    }
}

async fn login(req: tide::Request<()>) -> tide::Result {
    let name = req.param("name").unwrap();
    let session = MySession { name, count: 0 };
    let cookie = build_session_cookie(&session, SECRET_KEY.as_bytes())?.finish();

    let mut resp: tide::Response = tide::Redirect::new("/hello").into();
    resp.insert_cookie(cookie);
    Ok(resp)
}
