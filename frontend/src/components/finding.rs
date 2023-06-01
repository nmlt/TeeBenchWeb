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
        // <div class="col-md-auto">
        <div class="col-xl-4 col-lg-4 col-6">
            <div class="card my-2 mx-0" style={class_list}>
            // <div class="card my-4" style={class_list}>
                <div class="card-body">
                    <h5 class="card-text text-center">{finding.title.clone()}</h5>
                    <h5 class="card-title text-center">{finding.message.clone()}</h5>
                </div>
            </div>
        </div>
    }
}
