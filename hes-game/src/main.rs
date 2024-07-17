use axum::{
    body::Body,
    extract::State,
    http::{Request, Response, StatusCode, Uri},
    response::{Html, IntoResponse, Response as AxumResponse},
};
use hes_game_ui::CalcSurface;
use leptos::LeptosOptions;
use leptos_integration_utils::html_parts_separated;
use tower::ServiceExt;
use tower_http::services::ServeDir;

pub async fn file_or_index_handler(
    uri: Uri,
    State(options): State<LeptosOptions>,
) -> AxumResponse {
    let root = options.site_root.clone();
    let res =
        get_static_file(uri.clone(), &root).await.unwrap();

    if res.status() == StatusCode::OK {
        res.into_response()
    } else {
        let (head, tail) = html_parts_separated(&options, None);

        Html(format!("{head}</head><body>{tail}"))
            .into_response()
    }
}

async fn get_static_file(
    uri: Uri,
    root: &str,
) -> Result<Response<Body>, (StatusCode, String)> {
    let req = Request::builder()
        .uri(uri.clone())
        .body(Body::empty())
        .unwrap();
    // `ServeDir` implements `tower::Service` so we can call it with `tower::ServiceExt::oneshot`
    // This path is relative to the cargo root
    match ServeDir::new(root).oneshot(req).await {
        Ok(res) => Ok(res.into_response()),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {err}"),
        )),
    }
}

#[tokio::main]
async fn main() {
    use axum::{routing::post, Router};
    use leptos::*;

    // Setting get_configuration(None) means we'll be using cargo-leptos's env values
    // For deployment these variables are:
    // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
    // Alternately a file can be specified such as Some("Cargo.toml")
    // The file would need to be included with the executable when moved to deployment
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;

    server_fn::axum::register_explicit::<CalcSurface>();

    let app = Router::new()
        // server function handlers are normally set up by .leptos_routes()
        // here, we're not actually doing server side rendering, so we set up a manual
        // handler for the server fns
        // this should include a get() handler if you have any GetUrl-based server fns
        .route(
            "/compute/*fn_name",
            post(leptos_axum::handle_server_fns),
        )
        .fallback(file_or_index_handler)
        .with_state(leptos_options);

    let listener =
        tokio::net::TcpListener::bind(&addr).await.unwrap();
    logging::log!("listening on http://{}", &addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
