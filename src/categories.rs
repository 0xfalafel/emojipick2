use gtk::prelude::*;
use relm4::{
    prelude::*,
    typed_view::grid::TypedGridView,
};

use crate::emoji::Emoji;
use crate::Msg;

pub struct EmojiCollection {
    category: String,
    grid_view_wrapper: TypedGridView<Emoji, gtk::SingleSelection>,
}

#[relm4::factory(pub)]
impl FactoryComponent for EmojiCollection {
    type Init = (String, Vec<Emoji>);
    type Input = Msg;
    type Output = ();
    type CommandOutput = ();
    type ParentWidget = gtk::Box;

    fn init_model(init: Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {

        let (category, emojis) = init;

        let mut grid_view_wrapper: TypedGridView<Emoji, gtk::SingleSelection> = TypedGridView::new();
        grid_view_wrapper.add_filter(|item| item.name == "smile");
        grid_view_wrapper.set_filter_status(0, false);
        for emoji in emojis {
            grid_view_wrapper.append(Emoji::new(&emoji.symbol, &emoji.name));
        }
        
        Self {
            category,
            grid_view_wrapper,
        }
    }

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_spacing: 5,
            set_margin_all: 5,

            gtk::Label {
                set_label: &self.category,
            },

            gtk::GridView {
                set_orientation: gtk::Orientation::Vertical,
                set_max_columns: 10,
            }
        }
    }

    fn update(&mut self, msg: Self::Input, _sender: FactorySender<Self>) {
        match msg {
            Msg::OnlyShowEven(show_only_even) => {
                // Disable or enable the first filter
                self.grid_view_wrapper.set_filter_status(0, show_only_even);
            }
        }
    }
}
