use yew::prelude::*;

//use crate::data_structures::*;
use crate::navigation::Navigation;

// #[derive(Properties, PartialEq)]
// struct VersionsListProps {
//     versions: Vec<Version>,
// }

// #[function_component(VersionsList)]
// fn versions_list(VersionsListProps { versions }: &VersionsListProps) -> Html {
//     let list_items_html: Html = versions.iter().map(|version| html! {
//         <li class="list-group-item">{format!("Version: {}\n {:?}", version.title, version.report)}</li>
//     }).collect();
//     html! {
//         <ul class="list-group">
//             {list_items_html}
//         </ul>
//     }
// }

#[function_component(Commits)]
pub fn commits() -> Html {
    html! {
        <div>
            <h2>{"Commits"}</h2>
            <Navigation />
            <ul>
                <li>{"1.2 on 12.12.12"}</li>
                <li>{"1.3 on 13.12.12"}</li>
            </ul>
        </div>
    }
}
