use gtk::prelude::*;
use relm4::{
    prelude::*,
    typed_view::grid::{RelmGridItem, TypedGridView},
};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
struct Emoji {
    symbol: String,
    name: String,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct MyGridItem {
    emoji: String,
    value: u8,
    name: String,
}

impl MyGridItem {
    fn new(value: u8, emoji: &str, emoji_name: &str) -> Self {
        Self {
            emoji: emoji.to_owned(),
            value,
            name: String::from(emoji_name),
        }
    }
}

struct Widgets {
    emoji_button: gtk::Button,
}

impl RelmGridItem for MyGridItem {
    type Root = gtk::Box;
    type Widgets = Widgets;

    fn setup(_item: &gtk::ListItem) -> (gtk::Box, Widgets) {
        relm4::view! {
            my_box = gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                set_margin_all: 2,
                set_spacing: 5,

                #[name = "emoji_button"]
                gtk::Button {
                    add_css_class: "flat",
                },
            }
        }

        let widgets = Widgets {
            emoji_button,
        };

        (my_box, widgets)
    }

    fn bind(&mut self, widgets: &mut Self::Widgets, _root: &mut Self::Root) {
        let Widgets {
            emoji_button,
        } = widgets;

        emoji_button.set_label(&self.emoji);
        emoji_button.set_tooltip_text(Some(&self.name));
    }
}

struct App {
    counter: u8,
    grid_view_wrapper: TypedGridView<MyGridItem, gtk::SingleSelection>,
}

#[derive(Debug)]
enum Msg {
    Append,
    OnlyShowEven(bool),
}

#[relm4::component]
impl SimpleComponent for App {
    type Init = u8;
    type Input = Msg;
    type Output = ();

    view! {
        gtk::Window {
            set_title: Some("Actually idiomatic grid view possible?"),
            set_default_size: (350, 250),

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 5,
                set_margin_all: 5,

                gtk::Button {
                    set_label: "Append 10 items",
                    connect_clicked => Msg::Append,
                },

                gtk::ToggleButton {
                    set_label: "Only show even numbers",
                    connect_clicked[sender] => move |btn| {
                        sender.input(Msg::OnlyShowEven(btn.is_active()));
                    }
                },

                gtk::ScrolledWindow {
                    set_vexpand: true,

                    #[local_ref]
                    my_view -> gtk::GridView {
                        set_orientation: gtk::Orientation::Vertical,
                        set_max_columns: 10,
                    }
                }
            }
        }
    }

    fn init(
        counter: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        // Initialize the GridView wrapper
        let mut grid_view_wrapper: TypedGridView<MyGridItem, gtk::SingleSelection> =
            TypedGridView::new();

        // Add a filter and disable it
        grid_view_wrapper.add_filter(|item| item.name  == "smile");
        grid_view_wrapper.set_filter_status(0, false);
        grid_view_wrapper.append(MyGridItem::new(0, "🍓", "strawberry"));

        let model = App {
            counter,
            grid_view_wrapper,
        };



        let my_view = &model.grid_view_wrapper.view;

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            Msg::Append => {
                // Add 10 items
                for _ in 0..10 {
                    self.counter = self.counter.wrapping_add(1);
                    self.grid_view_wrapper.append(MyGridItem::new(self.counter, "😄", "smile"));
                }

            }
            Msg::OnlyShowEven(show_only_even) => {
                // Disable or enable the first filter
                self.grid_view_wrapper.set_filter_status(0, show_only_even);
            }
        }
    }
}

fn main() {
    // Include the JSON data as a static string
    const EMOJIS_JSON: &str = include_str!("emojis.json");

    // Parse the JSON data into a HashMap
    let categories: HashMap<String, Vec<Emoji>> = serde_json::from_str(EMOJIS_JSON).unwrap();

    // Print the emojis grouped by category
    for (category, emojis) in &categories {
        println!("Category: {}", category);
        for emoji in emojis {
            println!("  Symbol: {}, Name: {}", emoji.symbol, emoji.name);
        }
    }

    let app = RelmApp::new("relm4.example.typed-grid-view");
    app.run::<App>(0);
}
