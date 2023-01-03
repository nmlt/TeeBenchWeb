use yew::prelude::*;

use crate::{chartjs::Chart, modal::Modal, navigation::Navigation};
use common::data_types::Report;

#[function_component]
pub fn PerfReport() -> Html {
    html! {
        <div class="container-fluid">
            <div class="row vh-100">
                <div class="col-12 col-sm-3 col-xl-2 px-sm-2 px-0 bg-dark d-flex sticky-top">
                    <Navigation active_nav_item={"PerfReport"} />
                </div>
                <div class="col d-flex flex-column h-sm-100">
                    <main class="row">
                        <div class="col pt-4">
                            <div class="row">
                                // Top row
                            </div>
                            <div class="row">
                                // Graph cards
                                <div class="col-lg-6">
                                    <div class="card">
                                        <div class="card-body">
                                            <Chart report={Report::default()} />
                                        </div>
                                    </div>
                                </div>
                                <div class="col-lg-6">
                                    <div class="card">
                                        <div class="card-body">
                                            <h5 class="card-title">{"Throughput cache-fit"}</h5>
                                            <p class="card-text">{"With supporting text below as a natural lead-in to additional content."}</p>
                                            <a href="#" class="btn btn-primary">{"Go somewhere"}</a>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </main>
                </div>
            </div>
            <Modal />
        </div>
    }
}
