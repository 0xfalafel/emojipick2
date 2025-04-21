pub struct EmojiCollection {
    category: String,
    category: HashMap<String, Vec<Emoji>>,
    grid_view_wrapper: TypedGridView<Emoji, gtk::SingleSelection>,
}
