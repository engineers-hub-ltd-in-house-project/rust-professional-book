window.onload = function () {
    const ctx = document.getElementById('memoryChart').getContext('2d');
    const chart = new Chart(ctx, {
        type: 'line',
        data: {
            datasets: [{
                label: 'Memory Usage (KB)',
                borderColor: 'rgb(75, 192, 192)',
                backgroundColor: 'rgba(75, 192, 192, 0.2)',
                tension: 0.1,
                data: [],
                fill: true,
            }]
        },
        options: {
            scales: {
                x: {
                    type: 'time',
                    time: {
                        unit: 'second',
                        displayFormats: {
                            second: 'h:mm:ss a'
                        }
                    },
                    title: {
                        display: true,
                        text: 'Time'
                    }
                },
                y: {
                    beginAtZero: true,
                    title: {
                        display: true,
                        text: 'Memory (KB)'
                    }
                }
            },
            animation: {
                duration: 200 // 少しだけアニメーションさせる
            }
        }
    });

    const ws = new WebSocket(`ws://${window.location.host}/ws`);

    ws.onmessage = function (event) {
        try {
            const dataPoint = JSON.parse(event.data);
            const chartData = chart.data.datasets[0].data;

            chartData.push({
                x: dataPoint.timestamp * 1000, // Chart.jsはミリ秒を期待
                y: dataPoint.memory_kb
            });

            // グラフが無限に大きくならないように、古いデータを削除 (120点 = 2分)
            if (chartData.length > 120) {
                chartData.shift();
            }

            chart.update('quiet'); // 'quiet' は再描画時のアニメーションを防ぐ
        } catch (e) {
            console.error("Failed to parse or update chart:", e);
        }
    };

    ws.onopen = () => console.log("Connected to profiler server.");
    ws.onclose = () => console.log("Disconnected from profiler server.");
    ws.onerror = (error) => console.error("WebSocket Error:", error);
};
