use dioxus::{document::eval, document::Script, prelude::*};

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
    let search_query = "prediction markets";
    // URL-encode the query to handle spaces and special characters safely
    let encoded_query = urlencoding::encode(search_query);
    let search_url = format!("https://twitter.com/search?q={}", encoded_query);
    let placeholder = format!("Tweets about {search_query}");
    rsx!(
            h3{"Mission"}
            p{"Our mission is to empower individuals and organizations across Asia with actionable foresight. We are building the region's most trusted and liquid marketplace for future events, transforming collective knowledge into clear, tradable probabilities. PredictionMart provides a powerful and transparent tool for navigating the uncertainty of tomorrow's most critical events in technology, trade, economics, and geopolitics."}
            h3{"Our Target Audience: Who Trades on PredictionMart?"}
            p{"We built PredictionMart for those who value insight and seek to leverage their knowledge. Our platform serves a diverse community of forward-thinkers."}
            p{class:"subheading ","Persona 1: The Financial Professional"}
            p{"Meet Arjun: A quantitative analyst at a hedge fund in Mumbai. Arjun uses PredictionMart to hedge his portfolio against specific geopolitical risks and to discover new sources of alpha that aren't correlated with traditional stock markets. He values the platform for its ability to isolate event-specific outcomes, like 'Will the RBI cut interest rates in the next quarter?'"}
            p{class:"subheading ","Persona 2: The Industry Insider"}
            p{"Meet Priya: A senior product manager at a major tech firm in Bangalore. With deep domain expertise in AI development, Priya uses PredictionMart to monetize her specialized knowledge. She trades on markets like, 'Will Apple release an AI-powered iPhone by June 2025?' because her understanding of supply chains and R&D timelines gives her an edge over the broader market."}
            p{class:"subheading ","Persona 3: The Informed Strategist"}
            p{"Meet Kenji: A political science researcher in Singapore. Kenji is an expert on international trade policy. He uses PredictionMart to test his hypotheses on geopolitical events in a real-world environment. For him, the platform is a tool to validate his analysis and profit from his insights on complex topics like, 'Will the China-US trade deficit narrow by 5% in 2026?'"}
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
                    "Update!"
                }

    }
     h2 { "Live Feeds" }
            div {
                style: "display: flex; flex-direction: row; gap: 20px; justify-content: space-around;",

                div { style: "flex: 1;", X_Feed {
                        feed_url: search_url,
                        placeholder_text: placeholder
                    } }}
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
fn X_Feed(feed_url: String, placeholder_text: String) -> Element {
    // 1. Create an evaluator to run JavaScript. This is the modern Dioxus hook.

    // 2. Use an effect that runs after every render.
    // By providing no dependencies array, this effect is re-run each time
    // the component renders, which is exactly what we need.
    use_effect(move || {
        let js = r#"
            // Check if the Twitter script object (twttr) is available on the window
            if (window.twttr) {
                // If it is, tell it to scan the document and load any new widgets
                window.twttr.widgets.load();
            }
        "#;
        // Run the JavaScript
        eval(js);
    });

    rsx! {
        Script { src:"https://elfsightcdn.com/platform.js" }
        div {class:"elfsight-app-24f98a0a-0215-4d25-b455-da7a7407ed67" }

    }
}
