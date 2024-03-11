use html_site_generator::html::document::Document;
use html_site_generator::html::{IntoHtmlNode, IntoHtmlNodeResult};

#[derive(Debug)]
pub struct CustomDocument {
    inner: Document,
}

impl CustomDocument {
    pub fn new(document: Document) -> Self {
        CustomDocument { inner: document }
    }
}

impl IntoHtmlNode for CustomDocument {
    fn transform_into_raw_html(&self, buffer: &mut dyn std::io::Write) -> IntoHtmlNodeResult<()> {
        self.inner.transform_into_raw_html(buffer)?;

        Ok(())
    }

    fn transform_into_raw_css(&self, buffer: &mut dyn std::io::Write) -> IntoHtmlNodeResult<()> {
        writeln!(
            buffer,
            ":root {{
    --text: #060605;
    --background: #edede9;
    --primary: #7a7ad6;
    --secondary: #3939B8;
    --accent: #7d2dcd;
}}

@font-face {{
    font-family: \"Space Mono\";
    src: url(\"../fonts/SpaceMono-Regular.ttf\"),
    url(\"../fonts/SpaceMono-Bold.ttf\"),
    url(\"../fonts/SpaceMono-BoldItalic.ttf\"),
    url(\"../fonts/SpaceMono-Italic.ttf\");
}}

body {{
    background-color: var(--background);
    font-family: \"Space Mono\";
    color: var(--text);
    min-width: 20rem;
    max-width: 65rem;
}}

#rust-lang-icon {{
    height: 1em;
    position: relative;
    top: 0.125em;
    left: 0.125em;
}}

.navbar {{
    list-style-type: none;
    margin: 0;
    padding: 0;
}}
  
.navbar_item {{
    display: inline;
    margin-right: 10px;
}}

a {{
    color: var(--primary);
}}

a:link {{
    color: var(--primary);
}}

a:visited {{
    color: var(--secondary);
}}

a:hover {{
    color: var(--accent);
}}
"
        )?;

        self.inner.transform_into_raw_css(buffer)?;

        Ok(())
    }

    fn transform_into_raw_js(&self, buffer: &mut dyn std::io::Write) -> IntoHtmlNodeResult<()> {
        self.inner.transform_into_raw_js(buffer)?;

        Ok(())
    }
}
