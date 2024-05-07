use leptos::{IntoView, view};
use crate::components::core::footer::LocalFooter;
use crate::components::core::header::LocalHeader;
use crate::components::core::sidebar::SideBar;

pub fn Header() -> impl IntoView{
    view!{
        <LocalHeader/>
        <SideBar/>
        <LocalFooter/>
    }
}