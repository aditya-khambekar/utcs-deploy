use std::str::FromStr;
use scraper::{Html, Selector};

pub async fn get_random() -> String {
    let url = "https://apps.cs.utexas.edu/unixlabstatus/";
    let response = reqwest::get(url).await.unwrap();
    let body = response.text().await.unwrap();
    let document = Html::parse_document(&body);

    let row_sel = Selector::parse("table tr").unwrap();
    let td_sel = Selector::parse("td").unwrap();

    let mut hosts = Vec::new();

    for row in document.select(&row_sel) {
        let cells: Vec<_> = row.select(&td_sel).collect();
        if cells.len() == 5 {
            let host = cells[0].text().collect::<String>().trim().to_string();
            let status = cells[1].text().collect::<String>().trim().to_string();
            if host != "Host" {   // skip the header row
                if status.eq("up") {
                    hosts.push((String::from_str(&host), String::from_str(&status)));
                }
            }
        }
    }

    hosts[(rand::random::<u64>() % hosts.len() as u64) as usize].0.clone().unwrap() + ".cs.utexas.edu"
}
