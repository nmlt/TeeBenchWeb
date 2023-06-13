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
    introJs().setOptions({
        steps: [{
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
        {
            // title: 'Farewell!',
            // element: document.querySelector('.card__image'),
            intro: 'Select another view in the sidebar to continue the tour there.'
        }]
    }).start();
}

