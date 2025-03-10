use crate::prelude::*;
use chrono::NaiveDate;
use plotters::prelude::*;
use plotters::style::full_palette::{
    BLUE_400, CYAN_400, DEEPORANGE_500, GREEN_400, LIGHTBLUE_400, RED_400, YELLOW_400,
};
use serde::{Deserialize, Serialize};
use std::env;
use schemars::JsonSchema;

#[derive(Debug)]
enum GraphType {
    Line,
    Bar,
    Pie,
    Stock,
}

impl GraphType {
    fn from_str(s: &str) -> Option<GraphType> {
        match s {
            "line" => Some(GraphType::Line),
            "bar" => Some(GraphType::Bar),
            "pie" => Some(GraphType::Pie),
            "stock" => Some(GraphType::Stock),
            _ => None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct GraphData {
    x_values: Vec<f32>,
    y_values: Vec<f32>,
    caption: String,
    x_label: String,
    y_label: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct HistogramData {
    x_values: Vec<u32>,
    y_values: Vec<f32>,
    caption: String,
    x_label: String,
    y_label: String,
}
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct PieChartData {
    values: Vec<f64>,
    labels: Vec<String>,
    caption: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct StockChartData {
    dates: Vec<String>,
    open: Vec<f32>,
    high: Vec<f32>,
    low: Vec<f32>,
    close: Vec<f32>,
    caption: String,
}

pub struct Chart {
    pub chart_caption: String,
    pub chart_type: String,
    pub chart_file: String,
}

pub fn create_graph(
    graph_type: String,
    graph_data: Option<GraphData>,
    histogram_data: Option<HistogramData>,
    pie_chart_data: Option<PieChartData>,
    stock_chart_data: Option<StockChartData>,
) -> Result<Chart> {
    let graph_type = GraphType::from_str(&graph_type).unwrap();
    match graph_type {
        GraphType::Line => {
            let graph_data = graph_data.unwrap();
            let chart = create_line_graph(graph_data).expect("Unable to create line graph");
            Ok(chart)
        }
        GraphType::Bar => {
            let histogram_data = histogram_data.unwrap();
            let chart = create_histogram(histogram_data).expect("Unable to create histogram");
            Ok(chart)
        }
        GraphType::Pie => {
            let pie_chart_data = pie_chart_data.unwrap();
            let chart = create_pie_chart(pie_chart_data).expect("Unable to create pie");
            Ok(chart)
        }
        GraphType::Stock => {
            let stock_chart_data = stock_chart_data.unwrap();
            let chart = create_stock_chart(stock_chart_data).expect("Unable to create stock chart");
            Ok(chart)
        }
    }
}

fn create_line_graph(graph_data: GraphData) -> Result<Chart> {
    let temp_dir = env::temp_dir();
    let output_dir = temp_dir.join(format!("{}.png", graph_data.caption));
    let out_file_name: &str = output_dir.to_str().unwrap();
    let root = BitMapBackend::new(out_file_name, (900, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let x_min = graph_data
        .x_values
        .iter()
        .cloned()
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let x_max = graph_data
        .x_values
        .iter()
        .cloned()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap()
        + 1.0;
    let y_min = graph_data
        .y_values
        .iter()
        .cloned()
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let y_max = graph_data
        .y_values
        .iter()
        .cloned()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();

    let mut chart = ChartBuilder::on(&root)
        .caption(graph_data.caption.as_str(), ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(x_min..x_max, y_min..y_max)?;

    chart
        .configure_mesh()
        .x_labels(10)
        .y_labels(10)
        .disable_mesh()
        .x_label_formatter(&|x| format!("{:.1}", x))
        .y_label_formatter(&|y| format!("{:.1}", y))
        .x_desc(graph_data.x_label.as_str())
        .y_desc(graph_data.y_label.as_str())
        .draw()?;

    chart.draw_series(LineSeries::new(
        graph_data
            .x_values
            .iter()
            .zip(graph_data.y_values.iter())
            .map(|(x, y)| (*x, *y)),
        &BLACK,
    ))?;
    chart.draw_series(
        graph_data
            .x_values
            .iter()
            .zip(graph_data.y_values.iter())
            .map(|(x, y)| Circle::new((*x, *y), 5, BLACK.mix(0.5).filled())),
    )?;
    root.present().expect("Unable to save result");
    println!("Graph created at: {}", out_file_name);
    Ok(Chart {
        chart_caption: graph_data.caption.clone(),
        chart_type: "line".to_string(),
        chart_file: out_file_name.to_string(),
    })
}

fn create_histogram(histogram_data: HistogramData) -> Result<Chart> {
    let temp_dir = env::temp_dir();
    let output_dir = temp_dir.join(format!("{}.png", histogram_data.caption));
    let out_file_name: &str = output_dir.to_str().unwrap();
    let root = BitMapBackend::new(out_file_name, (900, 600)).into_drawing_area();
    root.fill(&WHITE)?;
    let x_values: Vec<u32> = histogram_data.x_values.to_vec();
    let x_min = x_values.iter().cloned().min().unwrap();
    let x_max = x_values.iter().cloned().max().unwrap() + 1;
    let y_min = 0;
    let y_max = histogram_data
        .y_values
        .iter()
        .cloned()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let mut chart = ChartBuilder::on(&root)
        .caption(
            histogram_data.caption.as_str(),
            ("sans-serif", 50).into_font(),
        )
        .margin(5)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(x_min..x_max, y_min as f32..y_max)?;

    chart
        .configure_mesh()
        .x_labels(10)
        .y_labels(10)
        .disable_mesh()
        .x_desc(histogram_data.x_label.as_str())
        .y_desc(histogram_data.y_label.as_str())
        .draw()?;

    chart.draw_series(
        Histogram::vertical(&chart)
            .style(RED.mix(0.5).filled())
            .data(
                x_values
                    .iter()
                    .zip(histogram_data.y_values.iter())
                    .map(|(x, y)| (*x, *y)),
            ),
    )?;
    root.present().expect("Unable to save result");
    println!("Bar chart created at: {}", out_file_name);
    Ok(Chart {
        chart_caption: histogram_data.caption.clone(),
        chart_type: "bar".to_string(),
        chart_file: out_file_name.to_string(),
    })
}

fn create_pie_chart(pie_chart_data: PieChartData) -> Result<Chart> {
    let temp_dir = env::temp_dir();
    let output_dir = temp_dir.join(format!("{}.png", pie_chart_data.caption));
    let out_file_name: &str = output_dir.to_str().unwrap();
    let root = BitMapBackend::new(out_file_name, (900, 600)).into_drawing_area();
    root.fill(&WHITE)?;
    let dims = root.dim_in_pixel();
    let center = (dims.0 as i32 / 2, dims.1 as i32 / 2);
    let radius = 200.0;
    let values = pie_chart_data.values;
    let colors = vec![
        RED_400,
        BLUE_400,
        GREEN_400,
        YELLOW_400,
        CYAN_400,
        MAGENTA,
        LIGHTBLUE_400,
    ];
    let labels = pie_chart_data.labels;
    let mut pie = Pie::new(&center, &radius, &values, &colors, &labels);
    pie.start_angle(0.0);
    pie.label_style(("sans-serif", 30).into_font().color(&(DEEPORANGE_500)));
    pie.percentages(
        ("sans-serif", radius * 0.08)
            .into_font()
            .color(&BLACK.mix(0.5)),
    );
    root.draw(&pie)?;
    root.present().expect("Unable to save result");
    println!("Pie chart created at: {}", out_file_name);
    Ok(Chart {
        chart_caption: pie_chart_data.caption.clone(),
        chart_type: "pie".to_string(),
        chart_file: out_file_name.to_string(),
    })
}

fn parse_time(date_str: &str) -> NaiveDate {
    NaiveDate::parse_from_str(date_str, "%Y-%m-%d").unwrap()
}

fn create_stock_chart(stock_data: StockChartData) -> Result<Chart> {
    let temp_dir = env::temp_dir();
    let output_dir = temp_dir.join(format!("{}.png", stock_data.caption));
    let out_file_name: &str = output_dir.to_str().unwrap();
    let root = BitMapBackend::new(out_file_name, (900, 600)).into_drawing_area();
    root.fill(&WHITE)?;
    let y_min = stock_data
        .low
        .iter()
        .cloned()
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap()
        - 10f32;
    let y_max = stock_data
        .high
        .iter()
        .cloned()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap()
        + 10f32;
    let x_min = parse_time(&stock_data.dates[0]) - chrono::Duration::days(1);
    let x_max =
        parse_time(&stock_data.dates[stock_data.dates.len() - 1]) + chrono::Duration::days(1);
    let mut chart = ChartBuilder::on(&root)
        .caption(stock_data.caption.as_str(), ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(x_min..x_max, y_min..y_max)?;

    chart.configure_mesh().light_line_style(WHITE).draw()?;
    chart.draw_series(stock_data.dates.iter().enumerate().map(|(i, date)| {
        CandleStick::new(
            parse_time(date),
            stock_data.open[i],
            stock_data.high[i],
            stock_data.low[i],
            stock_data.close[i],
            GREEN_400.filled(),
            RED_400,
            15,
        )
    }))?;
    root.present().expect("Unable to save result");
    println!("Stock chart created at: {}", out_file_name);
    Ok(Chart {
        chart_caption: stock_data.caption.clone(),
        chart_type: "stock".to_string(),
        chart_file: out_file_name.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_create_line_graph() {
        let graph_data = GraphData {
            x_values: vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0],
            y_values: vec![1.0, 4.0, 9.0, 16.0, 25.0, 5.3, 102.6],
            caption: "line_graph".to_string(),
            x_label: "x-values".to_string(),
            y_label: "y-values".to_string(),
        };
        assert!(create_graph("line".to_string(), Some(graph_data), None, None, None).is_ok());
    }

    #[test]
    fn test_create_bar_graph() {
        let histogram_data = HistogramData {
            x_values: vec![1, 2, 3, 4, 5, 6, 7],
            y_values: vec![1.0, 4.0, 9.0, 16.0, 25.0, 50.0, 100.0],
            caption: "bar_graph".to_string(),
            x_label: "x-values".to_string(),
            y_label: "y-values".to_string(),
        };
        assert!(create_graph("bar".to_string(), None, Some(histogram_data), None, None).is_ok());
    }

    #[test]
    fn test_create_pie_graph() {
        let pie_chart_data = PieChartData {
            values: vec![67.0, 33.0],
            labels: vec!["A".to_string(), "B".to_string()],
            caption: "pie_graph".to_string(),
        };
        assert!(create_graph("pie".to_string(), None, None, Some(pie_chart_data), None).is_ok());
    }

    #[test]
    fn test_create_stock_chart() {
        let stock_data = StockChartData {
            dates: vec![
                "2024-02-10".to_string(),
                "2024-02-11".to_string(),
                "2024-02-12".to_string(),
                "2024-02-13".to_string(),
                "2024-02-14".to_string(),
                "2024-02-15".to_string(),
                "2024-02-16".to_string(),
                "2024-02-17".to_string(),
                "2024-02-18".to_string(),
                "2024-02-19".to_string(),
                "2024-02-20".to_string(),
            ],
            open: vec![
                120.0, 125.0, 122.0, 118.0, 121.0, 127.0, 124.0, 126.0, 122.5, 120.0, 121.5,
            ],
            high: vec![
                125.0, 128.0, 124.5, 119.0, 123.0, 130.0, 126.5, 127.0, 124.0, 122.0, 124.0,
            ],
            low: vec![
                118.0, 123.0, 120.0, 116.5, 119.5, 125.5, 122.0, 123.5, 120.5, 118.5, 119.0,
            ],
            close: vec![
                123.0, 126.5, 121.0, 117.5, 122.0, 128.0, 125.0, 124.0, 121.0, 119.5, 122.0,
            ],
            caption: "stock_graph".to_string(),
        };
        assert!(create_graph("stock".to_string(), None, None, None, Some(stock_data)).is_ok());
    }
}
