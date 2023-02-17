use std::sync::Arc;

mod constants;
mod uwu;

use dominator::{clone, events, html, with_node, Dom};
use futures_signals::signal::Mutable;
use uwu::UwUify;
use wasm_bindgen::prelude::*;
use web_sys::{HtmlInputElement, HtmlTextAreaElement};

struct App {
    output: Mutable<String>,
    input: Mutable<String>,
    uwuify: UwUify,
}

impl App {
    fn new() -> Arc<Self> {
        Arc::new(App {
            input: Mutable::new(String::new()),
            output: Mutable::new(String::new()),
            uwuify: UwUify::default(),
        })
    }

    fn uwuify(&self) {
        let mut new = String::new();
        self.uwuify
            .uwuify_sentence(&*self.input.lock_ref(), &mut new)
            .unwrap();
        self.output.set_neq(new);
    }

    fn render_main(app: Arc<Self>) -> Dom {
        html!("div", {
            .class("container")
            .style("margin-top", "10%")

            .children(&mut [
                html!("label", {
                    .attr("for", "uwu")
                    .text("Text to UwUify")
                }),

                html!("textarea" => HtmlTextAreaElement, {
                    .focused(true)
                    .class("u-full-width")
                    .attr("id", "uwu")
                    .prop_signal("value", app.input.signal_cloned())

                    .with_node!(element => {
                        .event(clone!(app => move |_: events::Input| {
                            app.input.set_neq(element.value());
                            app.uwuify();
                        }))
                    })
                }),

                html!("center", {
                    .class("row")
                    .style("white-space", "nowrap")
                    .style("overflow-x", "scroll")
                    .style("margin-top", "5px")
                    .style("margin-bottom", "5px")

                    .children(&mut [
                        html!("button", {
                            .style("margin-right", "5px")
                            .style("width", "19%")
                            .style("min-width", "min-content")

                            .event(clone!(app => move |_: events::Click| {
                                app.uwuify.words.set_neq(1.0);
                                app.uwuify.faces.set_neq(0.05);
                                app.uwuify.actions.set_neq(0.125);
                                app.uwuify.stutters.set_neq(0.225);
                                app.uwuify();
                            }))

                            .text("Reset all")
                        }),

                        html!("button", {
                            .style("margin-right", "5px")
                            .style("width", "19%")
                            .style("min-width", "min-content")

                            .event(clone!(app => move |_: events::Click| {
                                app.uwuify.words.set_neq(1.0);
                                app.uwuify.faces.set_neq(1.0);
                                app.uwuify.actions.set_neq(1.0);
                                app.uwuify.stutters.set_neq(1.0);
                                app.uwuify();
                            }))

                            .text("Maximum UwU")
                        }),

                        html!("button", {
                            .style("margin-right", "5px")
                            .style("width", "19%")
                            .style("min-width", "min-content")

                            .event(clone!(app => move |_: events::Click| {
                                app.input.set_neq(app.output.get_cloned());
                                app.uwuify();
                            }))

                            .text("Double UwU")
                        }),

                        html!("button", {
                            .style("margin-right", "5px")
                            .style("width", "19%")
                            .style("min-width", "min-content")

                            .event(clone!(app => move |_: events::Click| {
                                app.uwuify.new_seed();
                                app.uwuify();
                            }))

                            .text("Regenerate Seed")
                        }),

                        html!("button", {
                            .style("margin-right", "5px")
                            .style("width", "19%")
                            .style("min-width", "min-content")

                            .event(clone!(app => move |_: events::Click| {
                                if let Some(window) = web_sys::window() {
                                    if let Some(clipboard) = window.navigator().clipboard() {
                                        wasm_bindgen_futures::spawn_local(clone!(app => async move {
                                            wasm_bindgen_futures::JsFuture::from(
                                                clipboard.write_text(&*app.output.lock_ref())
                                            ).await.unwrap();
                                        }))
                                    }
                                }
                            }))

                            .text("Copy to Clipboard")
                        })
                    ])
                }),

                html!("div", {
                    .class("row")

                    .children(&mut [
                        html!("div", {
                            .class(["three", "columns"])

                            .children(&mut [
                                html!("label", {
                                    .attr("for", "words")
                                    .text("Words")
                                }),

                                html!("button", {
                                    .event(clone!(app => move |_: events::Click| {
                                        app.uwuify.words.set_neq(1.0);
                                        app.uwuify();
                                    }))

                                    .text("Reset")
                                }),

                                html!("input" => HtmlInputElement, {
                                    .class("u-full-width")
                                    .attr("type", "range")
                                    .attr("id", "words")
                                    .attr("min", "0")
                                    .attr("max", "1")
                                    .attr("value", "1.0")
                                    .attr("step", "0.001")
                                    .prop_signal("value", app.uwuify.words.signal())

                                    .with_node!(element => {
                                        .event(clone!(app => move |_: events::Input| {
                                            app.uwuify.words.set_neq(element.value_as_number());
                                            app.uwuify();
                                        }))
                                    })
                                })
                            ])
                        }),

                        html!("div", {
                            .class(["three", "columns"])

                            .children(&mut [
                                html!("label", {
                                    .attr("for", "faces")
                                    .text("Faces")
                                }),

                                html!("button", {
                                    .event(clone!(app => move |_: events::Click| {
                                        app.uwuify.faces.set_neq(0.05);
                                        app.uwuify();
                                    }))

                                    .text("Reset")
                                }),

                                html!("input" => HtmlInputElement, {
                                    .class("u-full-width")
                                    .attr("type", "range")
                                    .attr("id", "faces")
                                    .attr("min", "0")
                                    .attr("max", "1")
                                    .attr("value", "0.05")
                                    .attr("step", "0.001")
                                    .prop_signal("value", app.uwuify.faces.signal())

                                    .with_node!(element => {
                                        .event(clone!(app => move |_: events::Input| {
                                            app.uwuify.faces.set_neq(element.value_as_number());
                                            app.uwuify();
                                        }))
                                    })
                                })
                            ])
                        }),

                        html!("div", {
                            .class(["three", "columns"])

                            .children(&mut [
                                html!("label", {
                                    .attr("for", "actions")
                                    .text("Actions")
                                }),

                                html!("button", {
                                    .event(clone!(app => move |_: events::Click| {
                                        app.uwuify.actions.set_neq(0.125);
                                        app.uwuify();
                                    }))

                                    .text("Reset")
                                }),

                                html!("input" => HtmlInputElement, {
                                    .class("u-full-width")
                                    .attr("type", "range")
                                    .attr("id", "actions")
                                    .attr("min", "0")
                                    .attr("max", "1")
                                    .attr("value", "0.125")
                                    .attr("step", "0.001")
                                    .prop_signal("value", app.uwuify.actions.signal())

                                    .with_node!(element => {
                                        .event(clone!(app => move |_: events::Input| {
                                            app.uwuify.actions.set_neq(element.value_as_number());
                                            app.uwuify();
                                        }))
                                    })
                                })
                            ])
                        }),

                        html!("div", {
                            .class(["three", "columns"])

                            .children(&mut [
                                html!("label", {
                                    .attr("for", "stutters")
                                    .text("Stutters")
                                }),

                                html!("button", {
                                    .event(clone!(app => move |_: events::Click| {
                                        app.uwuify.stutters.set_neq(0.225);
                                        app.uwuify();
                                    }))

                                    .text("Reset")
                                }),

                                html!("input" => HtmlInputElement, {
                                    .class("u-full-width")
                                    .attr("type", "range")
                                    .attr("id", "stutters")
                                    .attr("min", "0")
                                    .attr("max", "1")
                                    .attr("value", "0.225")
                                    .attr("step", "0.001")
                                    .prop_signal("value", app.uwuify.stutters.signal())

                                    .with_node!(element => {
                                        .event(clone!(app => move |_: events::Input| {
                                            app.uwuify.stutters.set_neq(element.value_as_number());
                                            app.uwuify();
                                        }))
                                    })
                                })
                            ])
                        })
                    ])
                }),

                html!("p", {
                    .attr("style", "min-height:200px")
                    .text_signal(app.output.signal_cloned())
                })
            ])
        })
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    console_error_panic_hook::set_once();

    let app = App::new();
    dominator::append_dom(&dominator::body(), App::render_main(app));
}
