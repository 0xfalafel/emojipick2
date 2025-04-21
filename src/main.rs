use gtk::prelude::*;
use relm4::{
    prelude::*,
    typed_view::grid::TypedGridView,
};
use std::collections::HashMap;

mod emoji;
use emoji::Emoji;

mod categories;
use categories::EmojiCollection;


struct App {
    emoji_collection: FactoryVecDeque<EmojiCollection>,
    // grid_view_wrapper: TypedGridView<Emoji, gtk::SingleSelection>,
}

#[derive(Debug)]
pub enum Msg {
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
                    emoji_collection_box -> gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,
                        // set_max_columns: 10,
                    }
                }
            }
        }
    }

    fn init(
        emojis_input: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {

        let emoji_collection = FactoryVecDeque::builder()
            .launch_default()
            .detach();

        let mut model = App {
            emoji_collection,
        };
    
        // Print the emojis grouped by category
        for (category, emojis) in emojis_input {
            let mut collections = model.emoji_collection.guard();
            collections.push_back((category, emojis));
        }

        let emoji_collection_box = model.emoji_collection.widget();

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    // fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
    //     match msg {
    //         Msg::OnlyShowEven(show_only_even) => {
    //             // Disable or enable the first filter
    //             self.grid_view_wrapper.set_filter_status(0, show_only_even);
    //         }
    //     }
    // }
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
