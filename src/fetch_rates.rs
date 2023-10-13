use std::collections::HashMap;

use leptos::{leptos_dom::logging::console_error, Serializable};
use serde::{Deserialize, Serialize};
use web_sys::AbortController;

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Quotes {
    pub data: Data,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Data {
    currency: String,
    pub rates: Rates,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct Rates(HashMap<String, String>);

impl Rates {
    pub fn replace(&self, expr: &str) -> String {
        self.0
            .iter()
            .filter(|(k, _)| k.as_str() != "00")
            .fold(expr.to_string(), |acc, (key, el)| {
                acc.replace(key, &format!("({el})"))
            })
    }
}

async fn fetch_api<T>(path: &str) -> Option<T>
where
    T: Serializable,
{
    let abort_controller = AbortController::new().ok();
    let abort_signal = abort_controller.as_ref().map(|a| a.signal());

    // abort in-flight requests if, e.g., we've navigated away from this page
    leptos::on_cleanup(move || {
        if let Some(abort_controller) = abort_controller {
            abort_controller.abort()
        }
    });

    let json = gloo_net::http::Request::get(path)
        .abort_signal(abort_signal.as_ref())
        .send()
        .await
        .map_err(|e| console_error(&format!("{e}")))
        .ok()?
        .text()
        .await
        .ok()?;

    T::de(&json).ok()
}

pub async fn fetch_rates() -> Option<Quotes> {
    fetch_api("https://api.coinbase.com/v2/exchange-rates").await
}
