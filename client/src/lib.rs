
use crate::error_template::{AppError, ErrorTemplate};


use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptonic::*;
use leptonic::prelude::*;
use leptonic::components::prelude::Root;
use components::core::*;
use pages::welcome::Welcome;

use leptonic::components::theme::LeptonicTheme;
use crate::pages::frontpage::FrontPage;
use crate::components::core::nationrouting::NationRoutes;

pub mod pages;
pub mod components;
pub mod error_template;
pub mod data;


#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.

    provide_meta_context();
    leptos_image::provide_image_context();

    view! {
        <Meta name="charset" content="UTF-8"/>
        <Meta name="description" content="Leptonic CSR template"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>
        <Meta name="theme-color" content="#e66956"/>

        <Stylesheet id="leptos" href="/pkg/start-axum-workspace.css"/>
        <Stylesheet href="https://fonts.googleapis.com/css?family=Roboto&display=swap"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>
            <Root default_theme=LeptonicTheme::default()>
                <Routes>
                    <Route path="hi" view=HomePage/>
                    <Route path="" view=FrontPage/>
                    <Route path="nations" view=NationRoutes/>
                </Routes>
            </Root>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}
