use leptos::{IntoView, view};
use crate::components::core::footer::LocalFooter;
use crate::components::core::header::LocalHeader;
use crate::components::core::sidebar::SideBar;

pub fn FrontPage() -> impl IntoView{
    view!{
        <body>
            <header>
                <LocalHeader/>
            </header>
            <SideBar/>
            <footer>
                <LocalFooter/>
            </footer>
        </body>
    }
}