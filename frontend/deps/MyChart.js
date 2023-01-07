export class MyChart {
    chart;
    constructor() {
    }

    draw(context, config) {
        let obj_config = JSON.parse(config);
        if (this.chart) {
            console.log("this.chart wasn't undefined.");
            let data = obj_config.data;
            let options = obj_config.options;
            this.chart.data = data;
            this.chart.options = options;
            this.chart.update();
        } else {
            console.log("Creating new chart.");
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
            console.log("Destroyed chart.");
        }
    }
}
