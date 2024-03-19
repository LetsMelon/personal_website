use std::sync::atomic::{AtomicBool, Ordering};

use html_site_generator::attributes::SetHtmlAttributes;
use html_site_generator::html::div::Div;
use html_site_generator::html::{IntoHtmlNode, IntoHtmlNodeResult};

#[derive(Debug)]
pub struct Separator;

impl IntoHtmlNode for Separator {
    fn transform_into_raw_html(&self, buffer: &mut dyn std::io::Write) -> IntoHtmlNodeResult<()> {
        let mut d = Div::new();
        d.add_class("separator");

        d.transform_into_raw_html(buffer)?;

        Ok(())
    }

    fn transform_into_raw_css(&self, buffer: &mut dyn std::io::Write) -> IntoHtmlNodeResult<()> {
        static WROTE_CSS: AtomicBool = AtomicBool::new(false);

        if !WROTE_CSS.load(Ordering::Relaxed) {
            writeln!(
                buffer,
                ".separator {{
    border-top: 2px solid var(--text);
    margin-top: 5px;
    margin-bottom: 10px;
}}"
            )?;

            // TODO this breaks because this function is called from `index` and `blog`, and it needs to be re-written in `blog`
            // WROTE_CSS.store(true, Ordering::Relaxed);
        }

        Ok(())
    }

    fn transform_into_raw_js(&self, _buffer: &mut dyn std::io::Write) -> IntoHtmlNodeResult<()> {
        Ok(())
    }
}
