use leptonic::prelude::*;
use leptos::*;
use leptos::html::Button;
use crate::components::core::footer::LocalFooter;
use crate::components::core::header::LocalHeader;
use crate::components::core::sidebar::SideBar;

#[component]
pub fn Welcome() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <LocalHeader/>
        <SideBar/>
        <LocalFooter/>
    }
}
