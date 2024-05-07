use leptonic::components::app_bar::AppBar;
use leptonic::components::button::{ButtonVariant, LinkButton};
use leptonic::components::theme::{LeptonicTheme};
use leptos::{ChildrenFn, component, IntoView, view};



pub fn LocalHeader() -> impl IntoView{
    view!{
            <HeaderNavbar/>
    }
}

#[component]
pub fn HeaderNavbarButton(name: &'static str, link: &'static str)-> impl IntoView{
    view! {
        <LinkButton href=link variant=ButtonVariant::Flat>
            {name}
        </LinkButton>
    }
}


#[component]
pub fn HeaderNavbar() -> impl IntoView {
    view! {
    <AppBar>
      <nav>
        <ul>
            <HeaderNavbarButton name="Home" link="/"/>
            <HeaderNavbarButton name="About" link="/about"/>
            <HeaderNavbarButton name="Stations" link="/stations"/>
            <HeaderNavbarButton name="Rivers" link="/rivers"/>
        </ul>
      </nav>
    </AppBar>
    }
}
