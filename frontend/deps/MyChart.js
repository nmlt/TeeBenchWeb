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
            element: document.querySelector('#tbw-commits-upload-form'),
            intro: 'This is the upload form for new operators.'
        },
        {
            element: document.querySelector('#tbw-commits-upload-form-operators'),
            intro: 'You can upload different types of relational operators: JOIN, GROUP BY, PROJECTION, ORDER BY'
        },
        {
            element: document.querySelector('#tbw-commits-commit-list'),
            intro: 'Any uploaded operators appear in this list.'
        },
        {
            element: document.querySelector('.tbw-commits-list-item-code'),
            intro: 'You can view its code,'
        },
        {
            element: document.querySelector('.tbw-commits-list-item-compiler-output'),
            intro: 'the compiler output from when it was compiled,'
        },
        {
            element: document.querySelector('.tbw-commits-list-item-report'),
            intro: 'the report that compares it to its baseline (explained in detail later),'
        },
        {
            element: document.querySelector('.tbw-commits-list-item-diff'),
            intro: 'and the <code>diff</code> between this operator\'s code and the previous.'
        },
        { // Currently this is step 7. If you add more before this or remove any, change the number in the if clause below
            // title: 'Farewell!',
            element: document.querySelector('.tbw-link-Profiling'),
            intro: 'Now we continue with the Profiling Menu, by changing the view in the sidebar.'
        }
    ];
    var profilingSteps = [
        {
            element: document.querySelector('#tbw-profiling-form'),
            intro: 'This form allows you to create custom experiments. In the static version of the TeeBench, only already cached results can be viewed.'
        },
        {
            element: document.querySelector(".foo"),
            intro: 'dummy text'
        }  
    ];
    var intro = introJs();
    var options = {
        steps: commitsSteps.concat(profilingSteps)
    };
    intro.setOptions(options).onbeforechange(function(targetElement) {  
        console.log(this._currentStep);
        console.log(targetElement);
        if (this._currentStep === 7) {
            // This works, but yew doesn't update.
            //window.history.pushState("some obj", "Title", "/profiling");
            targetElement.click();
            this.refresh();
            // this.addSteps(profilingSteps);
            
        } else if (this._currentStep === 8) {
            intro._introItems[8].element = document.querySelector('#tbw-profiling-form');
            intro._introItems[8].position = 'bottom';        }
        
    }).start();
}

