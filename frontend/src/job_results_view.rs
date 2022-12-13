use gloo_console::log;
use yew::prelude::*;
use yewdux::prelude::*;

use common::data_types::{
    //     Algorithm,
    //     Dataset,
    //     ExperimentType,
    //     Parameter,
    //     Platform,
    ProfilingConfiguration,
    QueueMessage,
    Job,
};

#[derive(Debug, Clone, Default, PartialEq, Store)]
pub struct FinishedJobState {
    pub jobs: Vec<Job>,
}

#[function_component]
pub fn JobResultsView() -> Html {
    let (finished_job_store, _dispatch) = use_store::<FinishedJobState>();
    let jobs = finished_job_store.jobs.iter().map(|j| {
        match j {
            Job::Running(_) => html! { <span>{"Error!"}</span> },
            Job::Finished { config, result } => {
                html! {
                    <li>
                        <span>{format!("Config: {config:?}")}</span>
                        <span>{format!("Result: {result:?}")}</span>
                    </li>
                }
            }
        }
    });
    html! {
        <ul class="list-group">
            {for jobs}
        </ul>
    }
}