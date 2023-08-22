use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::sync::Mutex;

use html_site_generator::datetime::DateTime;
use html_site_generator::strukt::document::Document;
use html_site_generator::strukt::object::list::{ListItem, ListObject, ListType};
use html_site_generator::strukt::object::text::TextObject;
use html_site_generator::strukt::object::Object;
use html_site_generator::strukt::paragraph::Paragraph;
use html_site_generator::transform::TransformAsHtml;
use once_cell::sync::Lazy;

static TEXT_DATA: Lazy<Mutex<HashMap<String, String>>> = Lazy::new(|| {
    let mut m = HashMap::new();

    include_str!("./long_text.txt")
        .split("\n")
        .for_each(|line| {
            let mut splitted = line.split("//");

            let name = splitted
                .next()
                .expect("Line in './long_text.txt' must have the structure: 'KEY//VALUE'");

            let text = splitted
                .next()
                .expect("Line in './long_text.txt' must have the structure: 'KEY//VALUE'");

            m.insert(name.to_string(), text.to_string());
        });

    Mutex::new(m)
});

fn main() {
    let text_data = TEXT_DATA.lock().unwrap();

    let document = Document {
        title: "Domenic Melcher Website".to_string(),
        keywords: vec![],
        publish_date: DateTime::from_ymd_opt(2023, 07, 10).unwrap(),
        paragraphs: vec![
            Paragraph {
                objects: vec![Object::List(ListObject {
                    // TODO make optional
                    text: TextObject {
                        content: "".to_string(),
                    },
                    items: vec![
                        // TODO I need here an `<a>...</a>` element
                        ListItem {
                            item: "Home".to_string(),
                        },
                        ListItem {
                            item: "Blogs".to_string(),
                        },
                    ],
                    list_type: ListType::Unordered,
                })],
            },
            Paragraph {
                objects: vec![
                    Object::Text(TextObject {
                        // TODO `<b>...</b>`
                        content: "Hi, my name is <b>Domenic Melcher</b>!".to_string(),
                    }),
                    Object::Text(TextObject {
                        content: text_data.get("main_text").unwrap().to_string(),
                    }),
                    Object::Text(TextObject {
                        content: text_data.get("programming").unwrap().to_string(),
                    }),
                    Object::List(ListObject {
                        text: TextObject {
                            content: "Some of my projects are:".to_string(),
                        },
                        items: vec![],
                        list_type: ListType::Unordered,
                    }),
                ],
            },
        ],
    };

    let file = File::create("out.html").unwrap();
    let mut writer = BufWriter::new(file);

    document.transform(&mut writer).unwrap();

    writer.flush().unwrap();
}
