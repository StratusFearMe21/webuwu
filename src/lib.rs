use std::sync::{Arc, Mutex};

mod constants;
mod uwu;

use dominator::{clone, events, html, with_node, Dom};
use futures_signals::signal::Mutable;
use uwu::UwUify;
use wasm_bindgen::prelude::*;
use web_sys::{HtmlInputElement, HtmlTextAreaElement};

struct App {
    output: Mutable<String>,
    input: Mutex<String>,
    uwuify: UwUify,
}

impl App {
    fn new() -> Arc<Self> {
        Arc::new(App {
            input: Mutex::new(String::new()),
            output: Mutable::new(String::new()),
            uwuify: UwUify::default(),
        })
    }

    fn uwuify(&self) {
        let mut new = String::new();
        self.uwuify
            .uwuify_sentence(self.input.lock().unwrap().as_str(), unsafe {
                new.as_mut_vec()
            })
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
                    .with_node!(element => {
                        .event(clone!(app => move |_: events::Input| {
                            *app.input.lock().unwrap() = element.value();
                            app.uwuify();
                        }))
                    })
                }),

                html!("div", {
                    .class("row")
                    .style("white-space", "nowrap")
                    .style("overflow-x", "scroll")
                    .style("margin-top", "5px")
                    .style("margin-bottom", "5px")

                    .children(&mut [
                        html!("button", {
                            .style("margin-right", "5px")

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

                            .event(clone!(app => move |_: events::Click| {
                                app.uwuify.new_seed();
                                app.uwuify();
                            }))

                            .text("Regenerate Seed")
                        }),
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
