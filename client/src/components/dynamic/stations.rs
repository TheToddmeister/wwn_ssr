use leptos::*;
use wwn_definitions::station::Station;
use leptonic::components::prelude::*;
use leptonic::components::table::*;
pub fn StationList(stations: Vec<Station>)-> impl IntoView{
    

    view!{
        <TableContainer>
            <Table bordered=true hoverable=true>
                <TableHeader>
                    <TableRow>
                        <TableHeaderCell min_width=true>"#"</TableHeaderCell>
                        <TableHeaderCell>"Name"</TableHeaderCell>
                        <TableHeaderCell>"Appearance"</TableHeaderCell>
                        <TableHeaderCell>"Num. eyes"</TableHeaderCell>
                    </TableRow>
                </TableHeader>
                <TableBody>
                    <For
                        each=move || stations.clone()
                        key=move |station| station.id.clone()
                        children=move |station| view! {
                            <TableRow>
                                <TableCell>{station.source_id}</TableCell>
                                <TableCell>{station.status}</TableCell>
                                <TableCell>{station.river_name}</TableCell>
                                <TableCell>{station.location.coordinates}</TableCell>
                            </TableRow>
                        }
                    />
                </TableBody>
            </Table>
        </TableContainer>
    }
}