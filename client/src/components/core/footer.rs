use leptos::{ IntoView, view};
use stylance::import_style;
use crate::components::core::header::HeaderNavbarButton;
use crate::components::images::local::LogoImage;

pub fn LocalFooter() -> impl IntoView{
    view!{
        <footer>
            <div>
                <FooterLinks/>
            </div>
        </footer>
    }
}

pub fn FooterLinks() -> impl IntoView{
    view! {
      <nav>
        <div>
        <ul>
            <HeaderNavbarButton name="Home" link="/"/>
            <HeaderNavbarButton name="About" link="/about"/>
            <LogoImage/>
            <HeaderNavbarButton name="References" link="/references"/>
            <HeaderNavbarButton name="Contact" link="/contact"/>
        </ul>
        </div>
      </nav>

    }
}

