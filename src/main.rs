use std::fs::File;
use std::path::PathBuf;
use std::str::FromStr;

use html_site_generator::html::IntoHtmlNode;

mod blog;
mod index;
pub(crate) mod utils;

fn main() {
    let pages = vec![
        (index::build(), "index.html"),
        (blog::build(), "blogs.html"),
    ];

    // TODO make an env variable
    let dst = PathBuf::from_str("./dst").unwrap();

    for (doc, name) in pages {
        let mut path = dst.clone();
        path.push(name);

        let mut file = File::create(path).unwrap();
        doc.transform_into_html_node(&mut file).unwrap();
    }
}
