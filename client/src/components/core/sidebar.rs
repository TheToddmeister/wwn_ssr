use leptos::{component, IntoView, view};
#[component]
pub fn SideBar() -> impl IntoView {

    view! {
            <h2 style="margin-top: 0;">Content</h2>
            <ul style="list-style-type: none; padding: 0;">
                <li><a href="#" style="display: block; padding: 10px; text-decoration: none; color: #333;">Home</a></li>
                <li><a href="#" style="display: block; padding: 10px; text-decoration: none; color: #333;">About</a></li>
                <li><a href="#" style="display: block; padding: 10px; text-decoration: none; color: #333;">Services</a></li>
                <li><a href="#" style="display: block; padding: 10px; text-decoration: none; color: #333;">Contact</a></li>
            </ul>
    }
}