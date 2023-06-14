export class MyChart {
    chart;
    constructor() {
    }

    draw(context, config) {
        let obj_config = JSON.parse(config);
        if (this.chart) {
            console.log("(This should not happen) this.chart wasn't undefined.");
            let data = obj_config.data;
            let options = obj_config.options;
            this.chart.data = data;
            this.chart.options = options;
            this.chart.update();
        } else {
            //console.log("Creating new chart.");
            this.chart = new Chart(
                context,
                obj_config
            );
        }
        return this.chart;
    }

    destroy() {
        if (this.chart) {
            this.chart.destroy();
            //console.log("Destroyed chart.");
        }
    }
}

export function hljs_highlight(code) {
    return hljs.highlight(code, { language: "cpp", ignoreIllegals: true}).value;
}

export function diff2html_html(diffInput) {
    return Diff2Html.html(diffInput, {"drawFileList": false, rawTemplates: {
        "tag-file-renamed": "",
        "generic-file-path": "" // Removes the header bar that has file names and dates.
    }, });
}

export function start_intro() {
    var commitsSteps = [
        {
            element: '#tbw-commits-upload-form',
            intro: 'This is the upload form for new operators.'
        },
        {
            element: '#tbw-commits-upload-form-operators',
            intro: 'You can upload different types of relational operators: JOIN, GROUP BY, PROJECTION, ORDER BY'
        },
        {
            element: '#tbw-commits-commit-list',
            intro: 'Any uploaded operators appear in this list.'
        },
        {
            element: '.tbw-commits-list-item-code',
            intro: 'You can view its code,'
        },
        {
            element: '.tbw-commits-list-item-compiler-output',
            intro: 'the compiler output from when it was compiled,'
        },
        {
            element: '.tbw-commits-list-item-report',
            intro: 'the report that compares it to its baseline (explained in detail later),'
        },
        {
            element: '.tbw-commits-list-item-diff',
            intro: 'and the <code>diff</code> between this operator\'s code and the previous.'
        },
        {
            // title: 'Farewell!',
            element: '.tbw-link-PerfReport',
            backup_element: '.tbw-link-PerfReport',
            intro: 'Now we are going to view the performance report for this commit, either by clicking on its "View Report" button or on "Performance Report" in the side bar.'
        }
    ];
    var profilingSteps = [
        {
            backup_element: '#tbw-profiling-form',
            intro: 'This form allows you to create custom experiments. In the static version of the TeeBench, only already cached results can be viewed.'
        },
        {
            backup_element: '.foo',
            intro: 'dummy text'
        }  
    ];
    var reportSteps = [
        {
            backup_element: '.tbw-report-general-findings',
            intro: 'The report has two parts: charts and analyzer findings. The findings are displayed at the top and give you information at a glance about how your operator is doing compared to the selected baseline'
        },
        {
            backup_element: '.tbw-report-charts-throughput-cache-fit',
            intro: 'First we show you the throughput of your operator for two datasets on SGX and on native. This chart is for a dataset that fits into the SGX memory.'
        },
        {
            backup_element: '.tbw-report-charts-scalability-cache-exceed',
            intro: 'Then we show the scalability by running the operator with different numbers of threads.'
        },
        {
            backup_element: '.tbw-report-charts-epc-paging-latest-alg',
            intro: 'Finally we show the EPC Page misses depending on dataset size.'
        },
        {
            backup_element: '.tbw-link-Profiling',
            intro: 'Now we continue with the Profiling Menu which can recreate all of these experiments and more, by changing the view in the sidebar.'
        }
    ]
    var steps = commitsSteps.concat(reportSteps).concat(profilingSteps);
    var intro = introJs();
    var options = {
        steps: steps
    };
    let step_before_perf_report = steps.findIndex(step => step.element == '.tbw-link-PerfReport');
    let step_before_profiling = steps.findIndex(step => step.backup_element == '.tbw-link-Profiling');
    console.log("step before profiling index:", step_before_profiling);
    // Fixing IntroJs, inspired by this: https://github.com/usablica/intro.js/issues/328#issuecomment-646445137
    intro.setOptions(options).onbeforechange(function(targetElement) {  
        console.log(this._currentStep);
        console.log(targetElement);
        function fix_intro_item(index, id_class) {
            console.log("fix_intro_item: ", index);
            console.log("fix_intro_item: ", intro._introItems[index]);
            let element = intro._introItems[index].backup_element ? intro._introItems[index].backup_element : intro._introItems[index].element;
            let selector = id_class ? id_class : element;
            intro._introItems.element = document.querySelector(selector);
            intro._introItems.position = 'bottom';
            // intro._introItems[indexc] = item;
        }
        console.log(this._introItems);
        if (this._currentStep === step_before_perf_report) {
            // This works, but yew doesn't update.
            //window.history.pushState("some obj", "Title", "/profiling");
            console.log("step before perfreport");
            // Instead targetting the link to the next page and clicking it programmatically:
            targetElement.click();
            fix_intro_item(step_before_perf_report, '.tbw-link-PerfReport');
            this.refresh(); // Actually probably useless.
        } else if (this._currentStep === step_before_perf_report + 1) {
            console.log("first step in perfreport");
            for (var i = step_before_perf_report + 1; i <= step_before_profiling; i++) {
                fix_intro_item(i, null);
            }
            this.refresh();
        } else if (this._currentStep === step_before_profiling) {
            console.log("step before profiling");
            targetElement.click();
            this.refresh();
            fix_intro_item(step_before_profiling, '.tbw-link-Profiling');
        } else if (this._currentStep === step_before_profiling + 1) {
            console.log("first step in profiling", this._currentStep, step_before_profiling + 1);
        }
    }).start();
}

