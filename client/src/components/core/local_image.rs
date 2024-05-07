use leptonic::components::button::{ButtonVariant, LinkButton};
use leptonic::components::theme::{LeptonicTheme};
use leptos::*;
use leptos::svg::Image;
use leptos_image::*;


#[component]
fn ImageComparison(width: u32, height: u32, blur: bool) -> impl IntoView {
    view! {
        <div
            style:margin-left="auto"
            style:margin-right="auto"
            style:display="flex"
            style:justify-content="space-around"
            style:align-items="center"
            style:gap="1rem"
        >
            <div>
                <div>
                    <h1>{format!("Optimized ({width} x {height}) with blur preview")}</h1>
                </div>
                <Image src="/logo.png" width height quality=85 blur class="test-image"/>
            </div>
        </div>
    }
}