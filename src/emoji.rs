use gtk::prelude::*;
use relm4::{prelude::*, typed_view::grid::RelmGridItem};
use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
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

impl RelmGridItem for Emoji {
    type Root = gtk::Box;
    type Widgets = gtk::Button;

    fn setup(_item: &gtk::ListItem) -> (gtk::Box, gtk::Button) {
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

        (my_box, emoji_button)
    }

    fn bind(&mut self, widget: &mut Self::Widgets, _root: &mut Self::Root) {
        widget.set_label(&self.symbol);
        widget.set_tooltip_text(Some(&self.name));
    }
}
