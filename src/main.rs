use gtk::prelude::*;
use relm4::{
    binding::{Binding, U8Binding},
    prelude::*,
    typed_view::grid::{RelmGridItem, TypedGridView},
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct MyGridItem {
    emoji: String,
    value: u8,
    binding: U8Binding,
    name: String,
}

impl MyGridItem {
    fn new(value: u8, emoji: &str, emoji_name: &str) -> Self {
        Self {
            emoji: emoji.to_owned(),
            value,
            binding: U8Binding::new(0),
            name: String::from(emoji_name),
        }
    }
}

struct Widgets {
    emoji_button: gtk::Button,
    label: gtk::Label,
}

impl Drop for Widgets {
    fn drop(&mut self) {
        dbg!(self.label.label());
    }
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
                gtk::Button,

                #[name = "label"]
                gtk::Label,
            }
        }

        let widgets = Widgets {
            emoji_button,
            label,
        };

        (my_box, widgets)
    }

    fn bind(&mut self, widgets: &mut Self::Widgets, _root: &mut Self::Root) {
        let Widgets {
            emoji_button,
            label,
        } = widgets;

        emoji_button.set_label(&self.emoji);
        emoji_button.set_tooltip_text(Some(&self.name));
        label.set_label(&format!("Value: {} ", self.value));
    }
}

struct App {
    counter: u8,
    grid_view_wrapper: TypedGridView<MyGridItem, gtk::SingleSelection>,
}

#[derive(Debug)]
enum Msg {
    Append,
    Remove,
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

                gtk::Button {
                    set_label: "Remove second item",
                    connect_clicked => Msg::Remove,
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
                        set_max_columns: 3,
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

                // self.grid_view_wrapper
                //     .iter()
                //     .for_each(|row| println!("item {}", row.borrow().value));

                // Count up the first item
                let first_item = self.grid_view_wrapper.get(0).unwrap();
                let first_binding = &mut first_item.borrow_mut().binding;
                let mut guard = first_binding.guard();
                *guard += 1;
            }
            Msg::Remove => {
                // Remove the second item
                self.grid_view_wrapper.remove(1);
            }
            Msg::OnlyShowEven(show_only_even) => {
                // Disable or enable the first filter
                self.grid_view_wrapper.set_filter_status(0, show_only_even);
            }
        }
    }
}

fn main() {
    let app = RelmApp::new("relm4.example.typed-grid-view");
    app.run::<App>(0);
}
