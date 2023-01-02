export class MyChart {
    chart;
    constructor() {
    }

    draw(context, config) {
        let obj_config = JSON.parse(config);
        if (this.chart) {
            let data = obj_config.data;
            let options = obj_config.options;
            this.chart.data = data;
            this.chart.options = options;
            this.chart.update();
        } else {
            this.chart = new Chart(
                context,
                obj_config
            );
        }
        return this.chart;
    }
}
