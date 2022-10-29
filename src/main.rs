use yew::prelude::*;

#[derive(Debug, Clone, PartialEq)]
struct Report {
    performance_gain: i32,
}

#[derive(Debug, Clone, PartialEq)]
struct Version {
    title: String,
    code: String,
    report: Report,
}

#[derive(Properties, PartialEq)]
struct VersionsListProps {
    versions: Vec<Version>,
}

#[function_component(VersionsList)]
fn versions_list(VersionsListProps { versions }: &VersionsListProps) -> Html {
    let list_items_html: Html = versions.iter().map(|version| html! {
        <li class="list-group-item">{format!("Version: {}\n {:?}", version.title, version.report)}</li>
    }).collect();
    html! {
        <ul class="list-group">
            {list_items_html}
        </ul>
    }
}

#[function_component(App)]
fn app() -> Html {
    let versions = vec![Version {
        title: "1.0".to_string(),
        code: "auto a = 0;".to_string(),
        report: Report {
            performance_gain: -1,
        },
    }];
    html! {
        <>
        <h1>{ "TeeBenchWeb" }</h1>
        <VersionsList versions={versions} />
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
