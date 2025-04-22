use gtk::prelude::*;
use relm4::{
    factory::FactoryView,
    prelude::*,
    typed_view::grid::TypedGridView,
};

use crate::emoji::Emoji;
use crate::Msg;

pub struct EmojiCollection {
    category: String,
    grid_view: TypedGridView<Emoji, gtk::SingleSelection>,
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

        let mut grid_view: TypedGridView<Emoji, gtk::SingleSelection> = TypedGridView::new();
        grid_view.extend_from_iter(emojis);

        Self {
            category,
            grid_view,
        }
    }

    fn init_widgets(
        &mut self,
        _index: &DynamicIndex,
        _root: Self::Root,
        _returned_widget: &<Self::ParentWidget as FactoryView>::ReturnedWidget,
        _sender: FactorySender<Self>,
    ) -> Self::Widgets {
        let widgets = view_output!();

        // Attach the GridView’s internal view to the container
        widgets.emoji_container.set_child(Some(self.grid_view.view));

        widgets
    }

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_spacing: 5,
            set_margin_all: 5,

            gtk::Label {
                set_label: &self.category,
            },

            #[name = "emoji_container"]
            gtk::ScrolledWindow {
                set_min_content_height: 200,
                set_min_content_width: 300,
            }
        }
    }

    fn update(&mut self, msg: Self::Input, _sender: FactorySender<Self>) {
        match msg {
            Msg::OnlyShowEven(show_only_even) => {
                // Toggle filter status (if you added filters later)
                self.grid_view.set_filter_status(0, show_only_even);
            }
        }
    }
}
