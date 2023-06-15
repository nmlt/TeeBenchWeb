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

export function bs_init_popovers() {
    const popoverTriggerList = document.querySelectorAll('[data-bs-toggle="popover"]');
    const popoverList = [...popoverTriggerList].map(popoverTriggerEl => new bootstrap.Popover(popoverTriggerEl));
}

export function start_intro() {
    var commitsSteps = [
        {
            element: document.querySelector('.foo'),
            title: 'Welcome!',
            intro: 'Welcome to the tour! TeeBench is a framework for fair and efficient benchmarking of ' +
                'relational operators across TEEs. Here, you can develop your own operators, upload them, and benchmark' +
                ' against existing state-of-the-art solutions.'
        },
        {
            element: document.querySelector('#tbw-commits-upload-form'),
            title: 'Operators',
            intro: 'This is the upload form for new operators.' +
                'Select an implementation of a relational operator, ' +
                'give it a title, version, and its primary baseline you want to compare to.'
        },
        {
            element: document.querySelector('#tbw-commits-upload-form-operators'),
            intro: 'You can upload different types of operators: JOIN, GROUP BY, PROJECTION, ORDER BY'
        },
        {
            element: document.querySelector('#tbw-commits-commit-list'),
            intro: 'All uploaded operators appear in this list.'
        },
        {
            element: document.querySelector('.tbw-commits-list-item-code'),
            intro: 'You can view its code...'
        },
        {
            element: document.querySelector('.tbw-commits-list-item-compiler-output'),
            intro: '...the compiler\'s output...'
        },
        {
            element: document.querySelector('.tbw-commits-list-item-report'),
            intro: '..a report that compares it to its baseline...'
        },
        {
            element: document.querySelector('.tbw-commits-list-item-diff'),
            intro: 'and a Github-like <code>diff</code> between your code and the previous version.'
        },
        { // Currently this is step 7. If you add more before this or remove any, change the number in the if clause below
            // title: 'Farewell!',
            element: document.querySelector('.tbw-link-Profiling'),
            intro: 'Let\'s now switch to the Profiling Menu, by changing the view in the sidebar.'
        }
    ];
    var profilingSteps = [
        {
            element: document.querySelector('#tbw-profiling-form'),
            title: 'Profiling',
            intro: 'This form allows you to create custom experiments containing any uploaded algorithms. '
        },
        {
            element: document.querySelector("#tbw-profiling-form-algs"),
            intro: 'Select the algorithms you want to benchmark. ' +
                'Pay attention - it\'s a multi-select!'
        },
        {
            element: document.querySelector("#tbw-profiling-form-experiment"),
            intro: 'Select one of the predefined configuration or play on your own. Be creative!'
        },
        {
            element: document.querySelector("#tbw-profiling-form-measurement"),
            intro: 'Specify what you want to measure and as a function of what parameter.'
        },
        {
            element: document.querySelector("#tbw-profiling-form-values"),
            intro: 'Specify the values of the measured parameter.'
        },
        {
            element: document.querySelector("#tbw-profiling-form-dataset"),
            intro: 'Pick the datasets to run your experiment. ' +
                'Cache-fit is designed to fit in the secure cache (EPC) of Intel SGX. ' +
                'Cache-exceed is larger than EPC and might trigger costly EPC paging. ' +
                'Select it if you want to see if your algorithm can handle EPC paging!'
        },
        {
            element: document.querySelector("#tbw-profiling-form-platform"),
            intro: 'Run it inside a TEE or on a traditional CPU.'
        },
        {
            element: document.querySelector("#tbw-profiling-form-run"),
            intro: 'Hit the button to run your experiment! Remember - this button is disables in the static website.'
        },
        {
            element: document.querySelector("#tbw-profiling-form-results"),
            intro: 'You can explore the results and the findings in this list. Push \"Results\" to check them out.'
        },
        {
            element: document.querySelector(".foo"),
            intro: 'You have reached the end of the tour. \nEnjoy playing with TeeBench!'
        }
    ];
    var intro = introJs();
    var options = {
        steps: commitsSteps.concat(profilingSteps)
    };
    intro.setOptions(options).onbeforechange(function(targetElement) {  
        console.log(this._currentStep);
        console.log(targetElement);
        if (this._currentStep === 8) {
            // This works, but yew doesn't update.
            //window.history.pushState("some obj", "Title", "/profiling");
            targetElement.click();
            this.refresh();
            // this.addSteps(profilingSteps);
            
        } else if (this._currentStep === 9) {
            intro._introItems[this._currentStep].element = document.querySelector('#tbw-profiling-form');
            intro._introItems[this._currentStep].position = 'bottom'
        } else if (this._currentStep === 10) {
            intro._introItems[this._currentStep].element = document.querySelector('#tbw-profiling-form-algs');
            intro._introItems[this._currentStep].position = 'bottom'
        } else if (this._currentStep === 11) {
            intro._introItems[this._currentStep].element = document.querySelector('#tbw-profiling-form-experiment');
            intro._introItems[this._currentStep].position = 'bottom'
        } else if (this._currentStep === 12) {
            intro._introItems[this._currentStep].element = document.querySelector('#tbw-profiling-form-measurement');
            intro._introItems[this._currentStep].position = 'bottom'
        } else if (this._currentStep === 13) {
            intro._introItems[this._currentStep].element = document.querySelector('#tbw-profiling-form-values');
            intro._introItems[this._currentStep].position = 'bottom'
        } else if (this._currentStep === 14) {
            intro._introItems[this._currentStep].element = document.querySelector('#tbw-profiling-form-dataset');
            intro._introItems[this._currentStep].position = 'bottom'
        } else if (this._currentStep === 15) {
            intro._introItems[this._currentStep].element = document.querySelector('#tbw-profiling-form-platform');
            intro._introItems[this._currentStep].position = 'bottom'
        } else if (this._currentStep === 16) {
            intro._introItems[this._currentStep].element = document.querySelector('#tbw-profiling-form-run');
            intro._introItems[this._currentStep].position = 'bottom'
        } else if (this._currentStep === 17) {
            intro._introItems[this._currentStep].element = document.querySelector('#tbw-profiling-form-results');
            intro._introItems[this._currentStep].position = 'bottom'
        }
        
    }).start();
}

