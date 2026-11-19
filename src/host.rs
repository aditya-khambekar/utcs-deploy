use anyhow::{Context, Result, bail};
use rand::seq::IndexedRandom;
use scraper::{Html, Selector};

const STATUS_URL: &str = "https://apps.cs.utexas.edu/unixlabstatus/";

/// Hosts listed as `up` in the status page HTML.
pub fn parse_up_hosts(html: &str) -> Vec<String> {
    let document = Html::parse_document(html);
    let row_sel = Selector::parse("table tr").unwrap();
    let td_sel = Selector::parse("td").unwrap();

    document
        .select(&row_sel)
        .filter_map(|row| {
            let cells: Vec<_> = row.select(&td_sel).collect();
            if cells.len() != 5 {
                return None;
            }
            let host = cells[0].text().collect::<String>().trim().to_string();
            let status = cells[1].text().collect::<String>().trim().to_string();
            (host != "Host" && status == "up").then_some(host)
        })
        .collect()
}

/// Pick a random `up` host, returned as `<host>.cs.utexas.edu`.
pub fn get_random() -> Result<String> {
    let body = reqwest::blocking::get(STATUS_URL)
        .and_then(|r| r.error_for_status())
        .and_then(|r| r.text())
        .context("fetching UTCS unix lab status page")?;
    let hosts = parse_up_hosts(&body);
    match hosts.choose(&mut rand::rng()) {
        Some(h) => Ok(format!("{h}.cs.utexas.edu")),
        None => bail!("no hosts are up on the status page"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_only_up_hosts() {
        let html = r#"<table>
            <tr><td>Host</td><td>Status</td><td>a</td><td>b</td><td>c</td></tr>
            <tr><td>alpha</td><td>up</td><td>1</td><td>2</td><td>3</td></tr>
            <tr><td>beta</td><td>down</td><td>1</td><td>2</td><td>3</td></tr>
            <tr><td>gamma</td><td> up </td><td>1</td><td>2</td><td>3</td></tr>
        </table>"#;
        assert_eq!(parse_up_hosts(html), vec!["alpha", "gamma"]);
    }
}
