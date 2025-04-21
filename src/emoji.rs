use gtk::prelude::*;
use relm4::{prelude::*, typed_view::grid::RelmGridItem};
use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Emoji {
    pub symbol: String,
    pub name: String,
}

impl Emoji {
    pub fn new(symbol: &str, name: &str) -> Self {
        Self {
            symbol: symbol.to_owned(),
            name: name.to_owned(),
        }
    }
}

#[derive(Debug)]
pub struct Widgets {
    emoji_button: gtk::Button,
}

impl RelmGridItem for Emoji {
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
        widgets.emoji_button.set_label(&self.symbol);
        widgets.emoji_button.set_tooltip_text(Some(&self.name));
    }
}
