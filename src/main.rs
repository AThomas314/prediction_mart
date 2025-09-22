use dioxus::prelude::*;
use image::ImageEncoder;
use plotters::prelude::*;
use rand::Rng;
use std::io::Cursor;

#[derive(Debug, Props, PartialEq, Clone)]
struct Data {
    data: Vec<f32>,
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut data_signal = use_signal(|| update_data(Vec::new()));

    rsx!(
        div{line_chart {
                data: Data { data: data_signal.read().clone() }

        }
        div {  
      button {
                onclick: move |_| {
                    let current_data = data_signal.read().clone();
                    data_signal.set(update_data(current_data));
                },
                id: "update",
                "Update!"
            }
        }
}
    )
}

fn update_data(data: Vec<f32>) -> Vec<f32> {
    let mut rng = rand::rng();
    let blank: Vec<f32> = Vec::new();
    let mut data = data.clone();
    if data == blank {
        for _i in 0..50 {
            data.push(rng.random_range(0.0..=1.0));
        }
    } else {
        data.remove(0);
        data.push(rng.random_range(0.0..=1.0));
    }
    data
}

#[component]
fn line_chart(data: Data) -> Element {
    let chart_resource = use_resource(move || {
        // Clone the vector *before* the async block.
        // The async block will capture and own this clone.
        let chart_data_clone = data.data.clone();

        async move {
            // Use the clone, which is now owned by the async block.
            generate_chart(chart_data_clone)
        }
    });

    let resource_value = chart_resource.read();

    match resource_value.as_ref() {
        Some(data_url) => rsx! {
            h1 { "Chart Generated!" }
            img {
                src: "{data_url}",
                alt: "A generated plot"
            }
        },
        // The resource is still loading
        None => rsx! {
            h1 { "Loading chart..." }
        },
    }
}
fn generate_chart(data: Vec<f32>) -> String {
    let width = 600;
    let height = 400;
    let mut buffer = vec![0; (width * height * 3) as usize];
    {
        let root = BitMapBackend::with_buffer(&mut buffer, (width, height)).into_drawing_area();
        root.fill(&WHITE).unwrap();

        let mut chart = ChartBuilder::on(&root)
            .margin(10)
            .build_cartesian_2d(-0.5f32..49.5f32, 0f32..1.1f32)
            .unwrap();

        chart.configure_mesh().draw().unwrap();

        chart
            .draw_series(LineSeries::new(
                (0..=49).map(|x| (x as f32, *data.get(x).unwrap_or(&0.0) as f32)),
                &RED,
            ))
            .unwrap();
    }

    let mut png_buffer = Vec::new();
    let encoder = image::codecs::png::PngEncoder::new(Cursor::new(&mut png_buffer));
    encoder
        .write_image(&buffer, width, height, image::ExtendedColorType::Rgb8)
        .unwrap();

    let encoded_png =
        base64::Engine::encode(&base64::engine::general_purpose::STANDARD, png_buffer);

    format!("data:image/png;base64,{}", encoded_png)
}
