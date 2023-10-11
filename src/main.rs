use evalexpr::eval;
use leptos::{html::Input, leptos_dom::logging::console_log, wasm_bindgen::JsValue, *};

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
    value: Signal<f64>,
    set_value: SignalSetter<f64>,
) -> impl IntoView {
    let node = create_node_ref::<Input>();

    let (local, set_local) = create_signal::<String>("".to_string());

    let text_value = (move || {
        let active = window().document().and_then(|doc| doc.active_element());
        let is_focused = match (active, node.get()) {
            (Some(active), Some(node)) => active == ***node,
            _ => false,
        };
        // let is_focused = true;
        // console_log(&format!("el: {} active: {}", label, is_focused));

        let local = local.get();
        let value = value.get().to_string();
        if is_focused {
            local
        } else {
            value
        }
    })
    .into_signal();
    view! {
        <div class="control">
            <p><label for={label}>{label}</label></p>
            <p><input _ref=node id={label} type="text" prop:value=text_value
                on:input=move |ev| {
                    let value = event_target_value(&ev);
                    set_local(event_target_value(&ev));
                    set_value(eval_expr(&value).unwrap_or_default());
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
    let (amount, set_amount) = create_signal::<f64>(0.0);
    let (yearly, set_yearly) = create_signal::<f64>(0.0);

    // let (daily, set_daily) = create_signal("".to_string());
    // let (monthly, set_monthly) = create_signal("".to_string());
    // let (yearly_5, set_yearly_5) = create_signal("".to_string());
    // let (yearly_10, set_yearly_10) = create_signal("".to_string());

    // let (daily_amount, set_daily_amount) = create_signal("".to_string());
    // let (monthly_amount, set_monthly_amount) = create_signal("".to_string());
    // let (yearly_amount, set_yearly_amount) = create_signal("".to_string());
    // let (yearly_5_amount, set_yearly_5_amount) = create_signal("".to_string());
    // let (yearly_10_amount, set_yearly_10_amount) = create_signal("".to_string());

    // let daily_expr = (move || daily.with(eval_expr).unwrap_or_default()).into_signal();
    // let monthly_expr = (move || monthly.with(eval_expr).unwrap_or_default()).into_signal();
    // let yearly_expr = (move || yearly.with(eval_expr).unwrap_or_default()).into_signal();
    // let yearly_5_expr = (move || yearly_5.with(eval_expr).unwrap_or_default()).into_signal();
    // let yearly_10_expr = (move || yearly_10.with(eval_expr).unwrap_or_default()).into_signal();

    // let amount_external = (move || amount.get()).into_signal();

    // let (base_daily, set_base_daily) = create_signal(0.0f64);
    // let (base_monthly, set_base_monthly) = create_signal(0.0f64);
    // let (base_yearly, set_base_yearly) = create_signal(0.0f64);
    // let (base_yearly_5, set_base_yearly_5) = create_signal(0.0f64);
    // let (base_yearly_10, set_base_yearly_10) = create_signal(0.0f64);

    // let daily_amount_external =
    //     (move || (base_daily.get() * calculated_amount.get()).to_string()).into_signal();
    // let monthly_amount_external =
    //     (move || (base_monthly.get() * calculated_amount.get()).to_string()).into_signal();
    // let yearly_amount_external =
    //     (move || (base_yearly.get() * calculated_amount.get()).to_string()).into_signal();
    // let yearly_5_amount_external =
    //     (move || (base_yearly_5.get() * calculated_amount.get()).to_string()).into_signal();
    // let yearly_10_amount_external =
    //     (move || (base_yearly_10.get() * calculated_amount.get()).to_string()).into_signal();

    // create_effect(move |_| {
    //     let rate = (daily_expr.get() + 100.0) / 100.0;
    //     set_base_monthly(rate.powf(30.0));
    //     set_base_yearly(rate.powf(365.0));
    //     set_base_yearly_5(rate.powf(5.0 * 365.0));
    //     set_base_yearly_10(rate.powf(10.0 * 365.0));
    // });
    // create_effect(move |_| {
    //     let rate = (monthly_expr.get() + 100.0) / 100.0;
    //     set_base_daily(rate.powf(1.0 / 30.0));
    //     set_base_yearly(rate.powf(12.0));
    //     set_base_yearly_5(rate.powf(5.0 * 12.0));
    //     set_base_yearly_10(rate.powf(10.0 * 12.0));
    // });
    // create_effect(move |_| {
    //     let rate = (yearly_expr.get() + 100.0) / 100.0;
    //     set_base_daily(rate.powf(1.0 / 365.0));
    //     set_base_monthly(rate.powf(1.0 / 12.0));
    //     set_base_yearly_5(rate.powf(5.0));
    //     set_base_yearly_10(rate.powf(10.0));
    // });
    // create_effect(move |_| {
    //     let rate = (yearly_5_expr.get() + 100.0) / 100.0;
    //     set_base_daily(rate.powf(1.0 / 5.0 / 365.0));
    //     set_base_monthly(rate.powf(1.0 / 5.0 / 12.0));
    //     set_base_yearly(rate.powf(1.0 / 5.0));
    //     set_base_yearly_10(rate.powf(2.0));
    // });
    // create_effect(move |_| {
    //     let rate = (yearly_10_expr.get() + 100.0) / 100.0;
    //     set_base_daily(rate.powf(1.0 / 10.0 / 365.0));
    //     set_base_monthly(rate.powf(1.0 / 10.0 / 12.0));
    //     set_base_yearly(rate.powf(1.0 / 10.0));
    //     set_base_yearly_5(rate.powf(1.0 / 2.0));
    // });

    // let daily_external = (move || ((base_daily.get() - 1.0) * 100.0).to_string()).into_signal();
    // let monthly_external = (move || ((base_monthly.get() - 1.0) * 100.0).to_string()).into_signal();
    // let yearly_external = (move || ((base_yearly.get() - 1.0) * 100.0).to_string()).into_signal();
    // let yearly_5_external =
    //     (move || ((base_yearly_5.get() - 1.0) * 100.0).to_string()).into_signal();
    // let yearly_10_external =
    //     (move || ((base_yearly_10.get() - 1.0) * 100.0).to_string()).into_signal();

    view! {
        <div class="line">
            <Controlled label="Amount" description="Amount to multiply by the interest" value=amount.into() set_value=set_amount.into() />
            <Uncontrolled label="Calculated amount" description="Calculate value of the amount" value=amount.into()/>
        </div>
        // <div class="line">
        //     <Controlled label="Daily" description="Daily interest rate in %" value=daily external=daily_external set_value=set_daily/>
        //     <Controlled label="Daily amount" description="Amount earned in a day." value=daily_amount external=daily_amount_external set_value=set_daily_amount />
        // </div>
        // <div class="line">
        //     <Controlled label="Monthly" description="Monthly interest rate in %" value=monthly external=monthly_external set_value=set_monthly />
        //     <Controlled label="Monthly amount" description="Amount earned in a month." value=monthly_amount external=monthly_amount_external set_value=set_monthly_amount />
        // </div>
        <div class="line">
            <Controlled label="Yearly" description="Yearly interest rate in %" value=yearly.into() set_value=set_yearly.into() />
            <Controlled label="Yearly amount" description="Amount earned in a year." value=(move || { yearly.get() * amount.get()}).into_signal() set_value=(move |a| set_amount.set(a / yearly.get())).into_signal_setter() />
        </div>
        // <div class="line">
        //     <Controlled label="5 years" description="5 years interest rate in %" value=yearly_5 external=yearly_5_external set_value=set_yearly_5 />
        //     <Controlled label="5 years amount" description="Amount earned in 5 years." value=yearly_5_amount external=yearly_5_amount_external set_value=set_yearly_5_amount />
        // </div>
        // <div class="line">
        //     <Controlled label="10 years" description="10 years interest rate in %" value=yearly_10 external=yearly_10_external set_value=set_yearly_10 />
        //     <Controlled label="10 years amount" description="Amount earned in 10 years." value=yearly_10_amount external=yearly_10_amount_external set_value=set_yearly_10_amount />
        // </div>
    }
}

fn main() {
    leptos::mount_to_body(|| view! { <App/> })
}
