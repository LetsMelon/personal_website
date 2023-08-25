use std::io::Write;

use html_site_generator::html::IntoHtmlNode;

mod index;

fn main() {
    let index = index::build();

    let mut output = Box::new(std::io::stdout()) as Box<dyn Write>;
    index.transform_into_html_node(&mut output).unwrap();
}
