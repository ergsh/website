use anyhow::Context;
use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use tower_http::services::ServeDir;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // build our application with a single route
    let assets_path = std::env::current_dir().unwrap();
    let port = 3000;
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));

    let app = Router::new().route("/", get(index_handler)).nest_service(
        "/assets",
        ServeDir::new(format!("{}/assets", assets_path.to_str().unwrap())),
    );

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app)
        .await
        .context("Error while starting server")?;

    Ok(())
}

async fn index_handler() -> impl IntoResponse {
    let template = IndexTemplate {};
    HtmlTemplate(template)
}

struct HtmlTemplate<T>(T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {}", err),
            )
                .into_response(),
        }
    }
}
