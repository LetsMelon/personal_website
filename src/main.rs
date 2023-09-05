#![feature(const_option)]

use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::str::FromStr;

use html_site_generator::html::IntoHtmlNode;
use html_site_generator::raw_writer::RawWriter;

mod blog;
mod index;
pub(crate) mod utils;
pub(crate) mod widget;

fn main() {
    let pages: Vec<(Box<dyn Fn() -> Box<dyn IntoHtmlNode>>, &str)> = vec![
        (Box::new(|| Box::new(index::build())), "index"),
        (Box::new(|| Box::new(blog::build())), "blogs"),
    ];

    // TODO make an env variable
    let dst = PathBuf::from_str("./dst").unwrap();

    for (doc_fct, name) in pages {
        let doc = doc_fct();

        let mut writer = RawWriter::new();

        doc.transform_into_html_node_with_css_and_js(&mut writer)
            .unwrap();

        let (html_writer, css_writer, js_writer) = writer.writers();

        let mut path = dst.clone();
        path.push(format!("{}.{}", name, "html"));

        let mut file = File::create(path).unwrap();
        file.write_all(html_writer.data()).unwrap();

        let mut path = dst.clone();
        path.push(format!("{}.{}", name, "css"));

        let mut file = File::create(path).unwrap();
        file.write_all(css_writer.data()).unwrap();

        let mut path = dst.clone();
        path.push(format!("{}.{}", name, "js"));

        let mut file = File::create(path).unwrap();
        file.write_all(js_writer.data()).unwrap();
    }
}

/*
// TODO use this colors
--text: #060605;
--background: #edede9;
--primary: #7a7ad6;
--secondary: #d1d1f5;
--accent: #7d2dcd;
*/
