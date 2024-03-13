use std::io::Write;

use chrono::{Datelike, NaiveDate, Weekday};
use html_site_generator::html::div::Div;
use html_site_generator::html::hyperlink::HyperlinkBuilder;
use html_site_generator::html::line_break::LineBreak;
use html_site_generator::html::paragraph::Paragraph;
use html_site_generator::html::{IntoHtmlNode, IntoHtmlNodeResult};

#[derive(Debug)]
pub struct Employment {
    inner: Div,
}

fn is_weekend(date: NaiveDate) -> bool {
    matches!(date.weekday(), Weekday::Sat) || matches!(date.weekday(), Weekday::Sun)
}

fn get_first_working_of_the_month(year: i32, month: u32) -> Option<NaiveDate> {
    let mut current_day = NaiveDate::from_ymd_opt(year, month, 1)?;

    while !is_weekend(current_day) {
        current_day = current_day.succ_opt()?;
    }

    Some(current_day)
}

fn get_last_working_of_the_month(year: i32, month: u32) -> Option<NaiveDate> {
    let mut current_day = NaiveDate::from_ymd_opt(year, month + 1, 1)
        .unwrap_or_else(
            || NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap(), // TODO unwrap
        )
        .pred_opt()?;

    while !is_weekend(current_day) {
        current_day = current_day.pred_opt()?;
    }

    Some(current_day)
}

fn format_naive_date(date: NaiveDate) -> String {
    date.format("%B %C%y").to_string()
}

impl Employment {
    pub fn new<S1: Into<String>, S2: Into<String>, S3: Into<String>>(
        company_name: S1,
        company_link: S2,
        description: Option<S3>,
        start_date: (i32, u32),
        end_date: Option<(i32, u32)>,
    ) -> Self {
        let description = description.map(|item| item.into());
        let start_date = get_first_working_of_the_month(start_date.0, start_date.1).unwrap();
        let end_date = end_date.map(|item| get_last_working_of_the_month(item.0, item.1).unwrap());

        let mut d = Div::new();

        let mut p = Paragraph::new();

        let mut h = HyperlinkBuilder::default()
            .href(company_link)
            .build()
            .unwrap();

        // TODO company name in bold
        // h.add_element(TextElement::new_with_styling(
        //     company_name,
        //     TextElementStyling::Bold,
        // ));
        h.add_element(company_name.into());
        p.add_element(h);
        p.add_element(LineBreak::new());

        let start_date_formatted = format_naive_date(start_date);
        let today = chrono::Utc::now().date_naive();

        let start_date_in_future = (start_date - today).num_seconds() >= 0;

        p.add_element(format!(
            "Duration: {}",
            match end_date {
                Some(end_date) =>
                    format!("{} - {}", start_date_formatted, format_naive_date(end_date)),
                None => {
                    let word = if start_date_in_future {
                        "starting"
                    } else {
                        "since"
                    };

                    format!("{} {}", word, start_date_formatted)
                }
            }
        ));
        p.add_element(LineBreak::new());

        if let Some(description) = description {
            p.add_element(description);
            p.add_element(LineBreak::new());
        }

        d.add_element(p);

        Self { inner: d }
    }
}

impl IntoHtmlNode for Employment {
    fn transform_into_raw_html(&self, buffer: &mut dyn Write) -> IntoHtmlNodeResult<()> {
        self.inner.transform_into_raw_html(buffer)?;

        Ok(())
    }

    fn transform_into_raw_css(&self, buffer: &mut dyn Write) -> IntoHtmlNodeResult<()> {
        self.inner.transform_into_raw_css(buffer)?;

        Ok(())
    }

    fn transform_into_raw_js(&self, buffer: &mut dyn Write) -> IntoHtmlNodeResult<()> {
        self.inner.transform_into_raw_js(buffer)?;

        Ok(())
    }
}
