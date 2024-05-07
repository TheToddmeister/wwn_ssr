use leptos::{component, IntoView, view};

use leptos::prelude::*;
use leptos_image::Image;
use leptonic::prelude::*;
use leptonic::*;

#[component]
pub fn LogoImage()->impl IntoView{
    view! {
        <div class="circular_image">
            <Image
                src="/logo.png"
                blur=false
                width=750
                height=500
                quality=85
            />
        </div>
    }
}

#[component]
pub fn RoutingImages(filename: &'static str) ->impl IntoView{
    view! {
        <Image
            src=filename
            blur=false
            width=750
            height=500
            quality=85
        />
    }
}