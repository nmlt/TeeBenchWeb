use yew::prelude::*;

use crate::{modal::Modal, navigation::Navigation};

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
                            <h2>{"Performance Report"}</h2>
                            <div class="row">
                                // Top row
                                <div class="col-sm-2">
                                    <div class="card my-4">
                                        <div class="card-body">
                                            <h5 class="card-text">{"Performance Gain"}</h5>
                                            <h5 class="card-title">{"+ 3.6 %"}</h5>
                                        </div>
                                    </div>
                                </div>
                                <div class="col-sm-2">
                                    <div class="card my-4">
                                        <div class="card-body">
                                            <h5 class="card-text">{"Phase 1: Sort"}</h5>
                                            <h5 class="card-title">{"180/191"}</h5>
                                        </div>
                                    </div>
                                </div>
                                <div class="col-sm-2">
                                    <div class="card my-4">
                                        <div class="card-body">
                                            <h5 class="card-text">{"Phase 2: Merge"}</h5>
                                            <h5 class="card-title">{"11/191"}</h5>
                                        </div>
                                    </div>
                                </div>
                                <div class="col-sm-2">
                                    <div class="card my-4">
                                        <div class="card-body">
                                            <h5 class="card-text">{"EPC Paging"}</h5>
                                            <h5 class="card-title">{"- 1.4 %"}</h5>
                                        </div>
                                    </div>
                                </div>
                                <div class="col-sm-2">
                                    <div class="card my-4">
                                        <div class="card-body">
                                            <h5 class="card-text">{"Overall Best Version"}</h5>
                                            <h5 class="card-title">{"v2.2"}</h5>
                                        </div>
                                    </div>
                                </div>
                            </div>
                            <div class="row">
                                // Graph cards
                                <div class="col-lg-6">
                                    <div class="card my-4">
                                        <div class="card-body ratio ratio-16x9">
                                            <iframe src="assets/throughput.html" />
                                        </div>
                                    </div>
                                    <div class="card my-4">
                                        <div class="card-body ratio ratio-16x9">
                                            <iframe src="assets/scalabilityCF.html" />
                                        </div>
                                    </div>
                                    <div class="card my-4">
                                        <div class="card-body ratio ratio-16x9">
                                            <iframe src="assets/epc.html" />
                                        </div>
                                    </div>
                                </div>
                                <div class="col-lg-6">
                                    <div class="card my-4">
                                        <div class="card-body ratio ratio-16x9">
                                            <iframe src="assets/throughput.html" />
                                        </div>
                                    </div>
                                    <div class="card my-4">
                                        <div class="card-body ratio ratio-16x9">
                                            <iframe src="assets/scalability.html" />
                                        </div>
                                    </div>
                                    <div class="card my-4">
                                        <div class="card-body ratio ratio-16x9">
                                            <iframe src="assets/epc2.html" />
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
