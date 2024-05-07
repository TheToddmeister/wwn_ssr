use leptos::html::div;
use leptos::*;
use crate::components::images::local::RoutingImages;
use leptonic::prelude::*;
use leptonic::components::grid::*;
use leptonic::components::skeleton::*;
#[component]
pub fn NationRoute(app_route: &'static str, image_route: &'static str, label: &'static str) -> impl IntoView{
    view! {
        <div>
            <a href=app_route>
                <RoutingImages filename=image_route />
            </a>
            <h2>{label}</h2>
        </div>
    }
}

#[component]
pub fn NationRoutes() -> impl IntoView{
    view!{
        <Grid gap=Size::Em(0.6)>
            <Row>
                <Col md=3 sm=4 xs=6>
                    <Skeleton animated=false>
                        <NationRoute app_route="/norway" image_route="/nations/norway.jpg" label="Norway"/>
                    </Skeleton>
                </Col>
                <Col md=3 sm=4 xs=6>
                    <Skeleton animated=false>
                        <NationRoute app_route="/uk" image_route="/nations/uk.jpg" label="Uk"/>
                    </Skeleton>
                </Col>
            </Row>
        </Grid>
    }
}