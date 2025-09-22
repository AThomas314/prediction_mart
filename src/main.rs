use dioxus::{document::Script, prelude::*};

use image::ImageEncoder;
use plotters::prelude::*;
use rand::Rng;
use std::io::Cursor;

fn main() {
    dioxus::launch(App);
}

// --- High-Level Components ---

#[component]
fn App() -> Element {
    let data_signal: Signal<Vec<f32>> = use_signal(|| update_data(Vec::new()));
    let shares_held = use_signal(String::new);

    // Derived state can be calculated here and passed down
    let current_val = data_signal.read().last().copied().unwrap_or(0.0);
    let num_shares_held = shares_held().parse::<i64>().unwrap_or_default();
    let payout = num_shares_held as f32 * current_val;

    rsx! {
        // Global styles
        style {
            "body {{
                background-color: #f0f2f5;
                font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
                color: #333;
                margin: 0;
                padding: 20px;
            }}"
        }

        // Main layout container
        div {
            style: "max-width: 1200px; margin: auto;",
            InformationCard {}
            div {
                style: "display: flex; flex-wrap: wrap; gap: 30px; align-items: flex-start;",
                TradingDashboard {
                    data_signal: data_signal,
                    current_val: current_val,
                    shares_held: shares_held,
                    payout: payout,
                }
                LiveFeed {}
            }
        }
    }
}

#[component]
fn InformationCard() -> Element {
    rsx!(
        div {
            style: "background: white; border-radius: 8px; box-shadow: 0 2px 8px rgba(0,0,0,0.1); padding: 25px; margin-bottom: 30px;",
            h2 {
                style: "color: #1e2a38; border-bottom: 2px solid #e0e6ed; padding-bottom: 10px; margin-top: 0;",
                "Mission & Personas"
            }
            Mission {}
            h3 {
                style: "color: #2c3e50; margin-top: 25px;",
                "Our Target Audience: Who Trades on PredictionMart?"
            }
            p {
                style: "line-height: 1.6; color: #555;",
                "We built PredictionMart for those who value insight and seek to leverage their knowledge. Our platform serves a diverse community of forward-thinkers."
            }
            Persona {
                title: "The Financial Professional",
                description: "Meet Arjun: A quantitative analyst at a hedge fund in Mumbai. Arjun uses PredictionMart to hedge his portfolio against specific geopolitical risks and to discover new sources of alpha that aren't correlated with traditional stock markets. He values the platform for its ability to isolate event-specific outcomes, like 'Will the RBI cut interest rates in the next quarter?'"
            }
            Persona {
                title: "The Industry Insider",
                description: "Meet Priya: A senior product manager at a major tech firm in Bangalore. With deep domain expertise in AI development, Priya uses PredictionMart to monetize her specialized knowledge. She trades on markets like, 'Will Apple release an AI-powered iPhone by June 2025?' because her understanding of supply chains and R&D timelines gives her an edge over the broader market."
            }
            Persona {
                title: "The Informed Strategist",
                description: "Meet Kenji: A political science researcher in Singapore. Kenji is an expert on international trade policy. He uses PredictionMart to test his hypotheses on geopolitical events in a real-world environment. For him, the platform is a tool to validate his analysis and profit from his insights on complex topics like, 'Will the China-US trade deficit narrow by 5% in 2026?'"
            }
        }
    )
}

#[component]
fn TradingDashboard(
    data_signal: Signal<Vec<f32>>,
    current_val: f32,
    mut shares_held: Signal<String>,
    payout: f32,
) -> Element {
    rsx! {
        div {
            style: "flex: 2; min-width: 500px; background: #1e2a38; color: white; border-radius: 8px; box-shadow: 0 2px 8px rgba(0,0,0,0.1); padding: 25px;",
            h3 { style: "margin-top: 0; color: #9cb3d0; border-bottom: 1px solid #3b4a5f; padding-bottom: 10px;", "Market Data" },
            div {
                style: "background: #2c3e50; border-radius: 6px; margin-bottom: 20px;",
                line_chart { data: data_signal }
            },
            div {
                style: "display: flex; justify-content: space-between; align-items: center; background: #2c3e50; padding: 15px; border-radius: 6px; font-size: 1.2rem; font-family: 'Courier New', monospace;",
                span { "Current Price:" },
                span { style: "font-weight: bold; color: #4dffaf;", {format!("{:.4}", current_val)} }
            }
            div {
                style: "margin-top: 20px; display: flex; gap: 15px; align-items: center;",
                input {
                    value: "{shares_held}",
                    oninput: move |event| shares_held.set(event.value()),
                    placeholder: "Enter shares to trade",
                    style: "flex-grow: 1; padding: 12px; border: 1px solid #3b4a5f; background: #1e2a38; color: white; border-radius: 6px; font-size: 1rem;"
                }
                button {
                    onclick: move |_| {
                        let current_data = data_signal.read().clone();
                        data_signal.set(update_data(current_data));
                    },
                    style: "padding: 12px 20px; background-color: #007bff; color: white; border: none; border-radius: 6px; cursor: pointer; font-size: 1rem; font-weight: bold; transition: background-color 0.2s ease;",
                    "Update Chart"
                }
            },
            div {
                style: "margin-top: 20px; padding: 15px; background: #2c3e50; border-radius: 6px; font-size: 1.1rem; text-align: center;",
                p { class: "payout", style: "margin: 0; color: #e0e6ed;", "Payout for ",
                   span { style: "font-weight: bold; color: #58a6ff;", "{shares_held}" , " shares: "},
                   span { style: "font-weight: bold; color: #4dffaf;", "{payout}"},
                //    span { style: "font-weight: bold; color: #4dffaf;", "{format!("{:.2}", payout)}" }
                }
            }
        }
    }
}

#[component]
fn LiveFeed() -> Element {
    let search_query = "prediction markets";
    let encoded_query = urlencoding::encode(search_query);
    let search_url = format!("https://twitter.com/search?q={}", encoded_query);

    rsx! {
        div {
            style: "flex: 1; min-width: 300px; background: white; border-radius: 8px; box-shadow: 0 2px 8px rgba(0,0,0,0.1); padding: 25px;",
            h2 {
                style: "color: #1e2a38; border-bottom: 2px solid #e0e6ed; padding-bottom: 10px; margin-top: 0;",
                "Live Feed"
            },
            TwitterFeed { feed_url: search_url }
        }
    }
}


// --- Low-Level & Utility Components ---

#[component]
fn Mission() -> Element {
    rsx! {
        h3 { style: "color: #2c3e50; margin-top: 25px;", "Our Mission" }
        p {
            style: "line-height: 1.6; color: #555;",
            "Our mission is to empower individuals and organizations across Asia with actionable foresight. We are building the region's most trusted and liquid marketplace for future events, transforming collective knowledge into clear, tradable probabilities. PredictionMart provides a powerful and transparent tool for navigating the uncertainty of tomorrow's most critical events in technology, trade, economics, and geopolitics."
        }
    }
}

#[component]
fn Persona(title: String, description: String) -> Element {
    rsx! {
        p { style: "font-weight: bold; color: #1e2a38; margin-top: 20px;", class: "subheading ", "{title}" }
        p {
            style: "line-height: 1.6; color: #555; font-style: italic;",
            "{description}"
        }
    }
}

#[component]
fn TwitterFeed(feed_url: String) -> Element {
    rsx! {
        Script { src: "https://elfsightcdn.com/platform.js" }
        div { class: "elfsight-app-24f98a0a-0215-4d25-b455-da7a7407ed67" }
    }
}

#[component]
fn line_chart(data: Signal<Vec<f32>>) -> Element {
    let chart_resource = use_resource(move || async move { generate_chart(data) });
    let cr = chart_resource.read();
    match cr.as_ref() {
        Some(data_url) => rsx! {
            img {
                src: "{data_url}",
                alt: "A generated plot",
                style: "max-width: 100%; border-radius: 6px;"
            }
        },
        None => rsx! { h1 { "Loading chart..." } },
    }
}


// --- Utility Functions (Unchanged) ---

fn update_data(mut data: Vec<f32>) -> Vec<f32> {
    let mut rng = rand::rng();
    if data.is_empty() {
        for _ in 0..50 {
            data.push(rng.random_range(0.0..=1.0));
        }
    } else {
        data.remove(0);
        data.push(rng.random_range(0.0..=1.0));
    }
    data
}

fn generate_chart(data: Signal<Vec<f32>>) -> String {
    let width = 600;
    let height = 400;
    let mut buffer = vec![0; (width * height * 3) as usize];
    {
        let root = BitMapBackend::with_buffer(&mut buffer, (width, height)).into_drawing_area();
        root.fill(&RGBColor(44, 62, 80)).unwrap();

        let mut chart = ChartBuilder::on(&root)
            .margin(15)
            .build_cartesian_2d(-0.5f32..49.5f32, 0f32..1f32)
            .unwrap();
        
        chart.configure_mesh()
            .axis_style(&RGBColor(156, 179, 208))
            .label_style(("sans-serif", 14).into_font().color(&RGBColor(156, 179, 208)))
            .draw().unwrap();

        chart.draw_series(LineSeries::new(
            (0..50).map(|x| (x as f32, data.read().get(x).copied().unwrap_or(0.0))),
            &RGBColor(77, 255, 175),
        )).unwrap();
    }

    let mut png_buffer = Vec::new();
    image::codecs::png::PngEncoder::new(Cursor::new(&mut png_buffer))
        .write_image(&buffer, width, height, image::ExtendedColorType::Rgb8)
        .unwrap();
    
    let encoded_png = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, png_buffer);
    format!("data:image/png;base64,{}", encoded_png)
}