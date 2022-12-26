export class MyChart {
    chart;
    constructor() {
        // let data = {
        //     labels: [1, 2, 3, 4, 5, 6],
        //     datasets: [{
        //         label: 'Widget data',
        //         backgroundColor: 'rgb(255, 99, 132)',
        //         borderColor: 'rgb(255, 99, 132)',
        //         data: [10, 35, 30, 20, 25, 15],
        //     }]
        // };

        // this.config = {
        //     type: 'line',
        //     data: data,
        //     options: {
        //         responsive: true,
        //         scales: {
        //             y: {
        //                 suggestedMin: 0,
        //                 suggestedMax: 50
        //             }
        //         }
        //     }
        // };
    }

    draw(element_id, config) {
        let obj_config = JSON.parse(config);
        console.log(obj_config);
        this.chart = new Chart(
            document.getElementById(element_id),
            obj_config
        )
    }
}
