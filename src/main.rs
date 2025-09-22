use dioxus::{document::Script, prelude::*};

use image::ImageEncoder;
use plotters::prelude::*;
use rand::Rng;
use std::io::Cursor;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let data = update_data(Vec::new());
    let mut data_signal: Signal<Vec<f32>> = use_signal(|| data);
    let current_val = *data_signal.read().get(49).unwrap();
    let mut shares_held = use_signal(String::new);
    let num_shares_held = shares_held().parse::<i64>().unwrap_or_default();
    let payout = num_shares_held as f32 * current_val;
    rsx!(
                style {
                "body {{
                background-color: #f0f2f5;
                font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
                color: #333;
                margin: 0;
                padding: 20px;
            }}"
                }
         div {
                style: "max-width: 1200px; margin: auto;",
                blah_blah{}
                div {
                        style: "display: flex; flex-wrap: wrap; gap: 30px; align-items: flex-start;",



                div{ line_chart {data:data_signal} }
                div { p { "{current_val}" } }
                input {value: "{shares_held}",oninput: move |event| shares_held.set(event.value()) }
                div {  p {  class:"payout","Payout at the current price of {current_val} for {shares_held} shares held: {payout}"}}
                div {
    button {
                            onclick: move |_| {
                                let current_data = data_signal.read().clone();
                                data_signal.set(update_data(current_data));
                            },
                            id: "update",
                            style: "padding: 12px 20px; background-color: #007bff; color: white; border: none; border-radius: 6px; cursor: pointer; font-size: 1rem; font-weight: bold; transition: background-color 0.2s ease;",
                            "Update Chart"
                        }
        },
                right_column{}                }}
    )
}

#[component]
fn blah_blah() -> Element {
    rsx!(           h2 {
                        style: "color: #1e2a38; border-bottom: 2px solid #e0e6ed; padding-bottom: 10px; margin-top: 0;",
                        "Mission & Personas"
                    }
                h3{                    style: "color: #2c3e50; margin-top: 25px;",
        "Our Mission"}
                p{                    style: "line-height: 1.6; color: #555;",
    "Our mission is to empower individuals and organizations across Asia with actionable foresight. We are building the region's most trusted and liquid marketplace for future events, transforming collective knowledge into clear, tradable probabilities. PredictionMart provides a powerful and transparent tool for navigating the uncertainty of tomorrow's most critical events in technology, trade, economics, and geopolitics."}
                h3{                    style: "color: #2c3e50; margin-top: 25px;",
    "Our Target Audience: Who Trades on PredictionMart?"}
                p{                    style: "line-height: 1.6; color: #555;",
    "We built PredictionMart for those who value insight and seek to leverage their knowledge. Our platform serves a diverse community of forward-thinkers."}
                p{style: "font-weight: bold; color: #1e2a38; margin-top: 20px;",class:"subheading ","Persona 1: The Financial Professional"}
                p{                    style: "line-height: 1.6; color: #555; font-style: italic;",
    "Meet Arjun: A quantitative analyst at a hedge fund in Mumbai. Arjun uses PredictionMart to hedge his portfolio against specific geopolitical risks and to discover new sources of alpha that aren't correlated with traditional stock markets. He values the platform for its ability to isolate event-specific outcomes, like 'Will the RBI cut interest rates in the next quarter?'"}
                p{style: "font-weight: bold; color: #1e2a38; margin-top: 20px;",class:"subheading ","Persona 2: The Industry Insider"}
                p{                    style: "line-height: 1.6; color: #555; font-style: italic;",
    "Meet Priya: A senior product manager at a major tech firm in Bangalore. With deep domain expertise in AI development, Priya uses PredictionMart to monetize her specialized knowledge. She trades on markets like, 'Will Apple release an AI-powered iPhone by June 2025?' because her understanding of supply chains and R&D timelines gives her an edge over the broader market."}
                p{style: "font-weight: bold; color: #1e2a38; margin-top: 20px;",class:"subheading ","Persona 3: The Informed Strategist"}
                p{                    style: "line-height: 1.6; color: #555; font-style: italic;",
    "Meet Kenji: A political science researcher in Singapore. Kenji is an expert on international trade policy. He uses PredictionMart to test his hypotheses on geopolitical events in a real-world environment. For him, the platform is a tool to validate his analysis and profit from his insights on complex topics like, 'Will the China-US trade deficit narrow by 5% in 2026?'"}
    )
}
#[component]
fn right_column() -> Element {
    let search_query = "prediction markets";
    // URL-encode the query to handle spaces and special characters safely
    let encoded_query = urlencoding::encode(search_query);
    let search_url = format!("https://twitter.com/search?q={}", encoded_query);
    let placeholder = format!("Tweets about {search_query}");

    rsx!(              div {
        style: "flex: 1; min-width: 300px; background: white; border-radius: 8px; box-shadow: 0 2px 8px rgba(0,0,0,0.1); padding: 25px;",
        h2 {
             style: "color: #1e2a38; border-bottom: 2px solid #e0e6ed; padding-bottom: 10px; margin-top: 0;",
             "Live Feed"

        },
        // x_feed {
        //     feed_url: search_url,
        //     placeholder_text: placeholder
        // },
    })
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
fn line_chart(data: Signal<Vec<f32>>) -> Element {
    let chart_resource = use_resource(move || {
        let data_for_chart = data.clone();

        async move {
            let resource_value = generate_chart(data_for_chart);
            resource_value
        }
    });

    let resource_value = chart_resource.read();

    match resource_value.as_ref() {
        Some(data_url) => rsx! {

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
fn generate_chart(data: Signal<Vec<f32>>) -> String {
    let width = 600;
    let height = 400;
    let mut buffer = vec![0; (width * height * 3) as usize];
    {
        let root = BitMapBackend::with_buffer(&mut buffer, (width, height)).into_drawing_area();
        root.fill(&WHITE).unwrap();

        let mut chart = ChartBuilder::on(&root)
            .margin(10)
            .build_cartesian_2d(-0.5f32..49.5f32, 0f32..1f32)
            .unwrap();
        let data = data();
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

#[component]
fn x_feed(feed_url: String, placeholder_text: String) -> Element {
    rsx! {
        Script { src:"https://elfsightcdn.com/platform.js" }
        div {class:"elfsight-app-24f98a0a-0215-4d25-b455-da7a7407ed67" }

    }
}
//   aa
