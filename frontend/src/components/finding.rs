use yew::prelude::*;

use common::data_types::{Finding, FindingStyle};

#[derive(Debug, PartialEq, Properties)]
pub struct FindingCardColumnProps {
    pub finding: Finding,
}

#[function_component]
pub fn FindingCardColumn(FindingCardColumnProps { finding }: &FindingCardColumnProps) -> Html {
    let class_list = match finding.style {
        FindingStyle::Neutral => "background-color: #FFFFFF;",
        FindingStyle::Good => "background-color: #77DD77;",
        FindingStyle::SoSo => "background-color: #FDE26C;",
        FindingStyle::Bad => "background-color: #FF6961;",
    };
    html! {
        <div class="col-xl-2 col-lg-4">
            <div class="card my-4" style={class_list}>
                <div class="card-body">
                    <h5 class="card-text">{finding.title.clone()}</h5>
                    <h5 class="card-title">{finding.message.clone()}</h5>
                </div>
            </div>
        </div>
    }
}
