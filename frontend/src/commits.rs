use yew::prelude::*;

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
        <div class="container-fluid">
            <div class="row vh-100">
                <div class="col-12 col-sm-3 col-xl-2 px-sm-2 px-0 bg-dark d-flex sticky-top">
                    <Navigation active_nav_item={"Commits"} />
                </div>
                <div class="col d-flex flex-column h-sm-100">
                    <main class="row">
                        <div class="col pt-4 col-lg-8">
                            <h2>{"Commits"}</h2>
                            <ul>
                                <li>{"1.2 on 12.12.12"}</li>
                                <li>{"1.3 on 13.12.12"}</li>
                            </ul>
                        </div>
                    </main>
                </div>
            </div>
        </div>
    }
}
