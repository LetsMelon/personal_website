use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use html_site_generator::html::IntoHtmlNode;
use html_site_generator::raw_writer::RawWriter;

pub fn invoke(out_dir: PathBuf) -> anyhow::Result<()> {
    let pages: Vec<(Box<dyn Fn() -> Box<dyn IntoHtmlNode>>, &str)> = vec![
        (
            Box::new(|| Box::new(crate::website::index::build())),
            "index",
        ),
        (
            Box::new(|| Box::new(crate::website::blog::build())),
            "blogs",
        ),
    ];

    for (doc_fct, name) in pages {
        let doc = doc_fct();

        let mut writer = RawWriter::new();

        doc.transform_into_html_node_with_css_and_js(&mut writer)
            .unwrap();

        let (html_writer, css_writer, js_writer) = writer.writers();

        let mut path = out_dir.clone();
        path.push(format!("{}.{}", name, "html"));

        let mut file = File::create(path).unwrap();
        file.write_all(html_writer.data()).unwrap();

        let mut path = out_dir.clone();
        path.push(format!("{}.{}", name, "css"));

        let mut file = File::create(path).unwrap();
        file.write_all(css_writer.data()).unwrap();

        let mut path = out_dir.clone();
        path.push(format!("{}.{}", name, "js"));

        let mut file = File::create(path).unwrap();
        file.write_all(js_writer.data()).unwrap();
    }

    Ok(())
}
