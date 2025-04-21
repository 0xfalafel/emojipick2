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

#[relm4::component]
impl SimpleComponent for EmojiCollection {
    type Init = (String, Vec<Emoji>);
    type Input = Msg;
    type Output = ();

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_spacing: 5,
            set_margin_all: 5,

            gtk::Label {
                set_label: "Hi mom!"
            },

            #[local_ref]
            my_view -> gtk::GridView {
                set_orientation: gtk::Orientation::Vertical,
                set_max_columns: 10,
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

        let (category, emojis) = emoji_collection;

        // Add the emojis to the GridView
        for emoji in emojis {
            grid_view_wrapper.append(
                Emoji::new(&emoji.symbol, &emoji.name)
            );
        }
        
        let model = EmojiCollection {
            category,
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
