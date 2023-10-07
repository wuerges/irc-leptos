use evalexpr::eval;
use leptos::{html::Input, leptos_dom::logging::console_log, svg::view, wasm_bindgen::JsValue, *};

fn eval_expr(expr: &String) -> Result<f64, String> {
    if expr.trim().is_empty() {
        return Ok(0.0);
    }
    match eval(expr).and_then(|x| x.as_number()) {
        Ok(result) => Ok(result),
        Err(err) => Err(format!("{:?}", err)),
    }
}

#[component]
fn Controlled(
    label: &'static str,
    description: &'static str,
    value: ReadSignal<String>,
    external: Signal<String>,
    set_value: WriteSignal<String>,
) -> impl IntoView {
    let node = create_node_ref::<Input>();
    let active = window().document().and_then(|doc| doc.active_element());
    let is_focused = match (active, node.get()) {
        (Some(active), Some(node)) => active == ***node,
        _ => false,
    };
    let text_value = (move || {
        if is_focused {
            value.get()
        } else {
            external.get()
        }
    })
    .into_signal();
    view! {
        <div class="control">
            <p><label for={label}>{label}</label></p>
            <p><input _ref=node id={label} type="text" prop:value=text_value
                on:input=move |ev| {
                    set_value(event_target_value(&ev))
                }
            /></p>
            <Show
                when=move || text_value.with(eval_expr).is_err()
            >
                <p><span class="error">ERROR</span></p>
            </Show>
            <p><span class="help">{description}</span></p>
        </div>
    }
}

#[component]
fn Uncontrolled<T>(
    label: &'static str,
    description: &'static str,
    value: Signal<T>,
) -> impl IntoView
where
    T: 'static + Clone,
    JsValue: From<T>,
{
    view! {
        <div class="control">
            <p><label for={label}>{label}</label></p>
            <p><input id={label} type="text" prop:value=value disabled="true"/></p>
            <p><span class="help">{description}</span></p>
        </div>
    }
}

#[component]
fn App() -> impl IntoView {
    let (amount, set_amount) = create_signal("".to_string());
    let calculated_amount = (move || amount.with(eval_expr).unwrap_or_default()).into_signal();

    let (daily, set_daily) = create_signal("".to_string());
    let (monthly, set_monthly) = create_signal("".to_string());
    let (yearly, set_yearly) = create_signal("".to_string());
    let (yearly_5, set_yearly_5) = create_signal("".to_string());
    let (yearly_10, set_yearly_10) = create_signal("".to_string());

    let (daily_amount, set_daily_amount) = create_signal("".to_string());
    let (monthly_amount, set_monthly_amount) = create_signal("".to_string());
    let (yearly_amount, set_yearly_amount) = create_signal("".to_string());
    let (yearly_5_amount, set_yearly_5_amount) = create_signal("".to_string());
    let (yearly_10_amount, set_yearly_10_amount) = create_signal("".to_string());

    let daily_expr = (move || daily.with(eval_expr).unwrap_or_default()).into_signal();
    let monthly_expr = (move || monthly.with(eval_expr).unwrap_or_default()).into_signal();
    let yearly_expr = (move || yearly.with(eval_expr).unwrap_or_default()).into_signal();
    let yearly_5_expr = (move || yearly_5.with(eval_expr).unwrap_or_default()).into_signal();
    let yearly_10_expr = (move || yearly_10.with(eval_expr).unwrap_or_default()).into_signal();

    let daily_amount_external =
        (move || (daily_expr.get() * calculated_amount.get()).to_string()).into_signal();
    let monthly_amount_external =
        (move || (monthly_expr.get() * calculated_amount.get()).to_string()).into_signal();
    let yearly_amount_external =
        (move || (yearly_expr.get() * calculated_amount.get()).to_string()).into_signal();
    let yearly_5_amount_external =
        (move || (yearly_5_expr.get() * calculated_amount.get()).to_string()).into_signal();
    let yearly_10_amount_external =
        (move || (yearly_10_expr.get() * calculated_amount.get()).to_string()).into_signal();

    let amount_external = (move || amount.get()).into_signal();
    let daily_external = (move || daily.get()).into_signal();
    let monthly_external = (move || monthly.get()).into_signal();
    let yearly_external = (move || yearly.get()).into_signal();
    let yearly_5_external = (move || yearly_5.get()).into_signal();
    let yearly_10_external = (move || yearly_10.get()).into_signal();

    view! {
        <div class="line">
            <Controlled label="Amount" description="Amount to multiply by the interest" value=amount external=amount_external set_value=set_amount />
            <Uncontrolled label="Calculated amount" description="Calculate value of the amount" value=calculated_amount/>
        </div>
        <div class="line">
            <Controlled label="Daily" description="Daily interest rate in %" value=daily external=daily_external set_value=set_daily/>
            <Controlled label="Daily amount" description="Amount earned in a day." value=daily_amount external=daily_amount_external set_value=set_daily_amount />
        </div>
        <div class="line">
            <Controlled label="Monthly" description="Monthly interest rate in %" value=monthly external=monthly_external set_value=set_monthly />
            <Controlled label="Monthly amount" description="Amount earned in a month." value=monthly_amount external=monthly_amount_external set_value=set_monthly_amount />
        </div>
        <div class="line">
            <Controlled label="Yearly" description="Yearly interest rate in %" value=yearly external=yearly_external set_value=set_yearly />
            <Controlled label="Yearly amount" description="Amount earned in a year." value=yearly_amount external=yearly_amount_external set_value=set_yearly_amount />
        </div>
        <div class="line">
            <Controlled label="5 years" description="5 years interest rate in %" value=yearly_5 external=yearly_5_external set_value=set_yearly_5 />
            <Controlled label="5 years amount" description="Amount earned in 5 years." value=yearly_5_amount external=yearly_5_amount_external set_value=set_yearly_5_amount />
        </div>
        <div class="line">
            <Controlled label="10 years" description="10 years interest rate in %" value=yearly_10 external=yearly_10_external set_value=set_yearly_10 />
            <Controlled label="10 years amount" description="Amount earned in 10 years." value=yearly_10_amount external=yearly_10_amount_external set_value=set_yearly_10_amount />
        </div>
    }
}

fn main() {
    leptos::mount_to_body(|| view! { <App/> })
}
