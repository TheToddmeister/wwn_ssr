use app::*;
use axum::Router;
use axum::routing::post;
use fileserv::file_and_error_handler;
use leptos::*;
use leptos_axum::{generate_route_list, LeptosRoutes, handle_server_fns};
use leptos_image;
use leptos_image::ImageCacheRoute;
use http;

pub mod fileserv;

#[tokio::main]
async fn main() {
    simple_logger::init_with_level(log::Level::Debug).expect("couldn't initialize logging");

    // Setting get_configuration(None) means we'll be using cargo-leptos's env values
    // For deployment these variables are:
    // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
    // Alternately a file can be specified such as Some("Cargo.toml")
    // The file would need to be included with the executable when moved to deployment
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;

    let routes = generate_route_list(App);

    #[derive(Clone, axum::extract::FromRef)]
    struct AppState {
        leptos_options: leptos::LeptosOptions,
        optimizer: leptos_image::ImageOptimizer,
    }
    let root = leptos_options.site_root.clone();

    let state = AppState {
        leptos_options,
        optimizer: leptos_image::ImageOptimizer::new("/cache/image", root, 1),
    };

    // Build Router.
    let app = Router::new()
        .route("/api/*fn_name", post(handle_server_fns))
        // Add a handler for serving the cached images.
        .image_cache_route(&state)
        // Provide the optimizer to leptos context.
        .leptos_routes_with_context(&state, routes, state.optimizer.provide_context(), App)
        .fallback(file_and_error_handler)
        .with_state(state);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log::info!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
