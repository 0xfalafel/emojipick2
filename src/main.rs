use gtk::prelude::*;
use relm4::{
    prelude::*,
    typed_view::grid::TypedGridView,
};
use std::collections::HashMap;

mod emoji;
use emoji::Emoji;

struct App {
    emoji_collection: HashMap<String, Vec<Emoji>>,
    grid_view_wrapper: TypedGridView<Emoji, gtk::SingleSelection>,
}

#[derive(Debug)]
enum Msg {
    OnlyShowEven(bool),
}

#[relm4::component]
impl SimpleComponent for App {
    type Init = HashMap<String, Vec<Emoji>>;
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
        emoji_collection: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        // Initialize the GridView wrapper
        let mut grid_view_wrapper: TypedGridView<Emoji, gtk::SingleSelection> =
            TypedGridView::new();

        // Add a filter and disable it
        grid_view_wrapper.add_filter(|item| item.name == "smile");
        grid_view_wrapper.set_filter_status(0, false);

        // Add the emojis to the GridView
        for (_category, emojis) in &emoji_collection {
            for emoji in emojis {
                grid_view_wrapper.append(Emoji::new(&emoji.symbol, &emoji.name));
            }
        }
        
        let model = App {
            emoji_collection,
            grid_view_wrapper,
        };

        let my_view = &model.grid_view_wrapper.view;

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
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
    app.run::<App>(categories);
}
