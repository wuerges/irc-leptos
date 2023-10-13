use evalexpr::eval;
use irc_leptos::{
    fetch_rates::{fetch_rates, Rates},
    local_storage::create_local_storage,
};
use leptos::{html::Input, wasm_bindgen::JsValue, *};

#[derive(Clone, Copy)]
struct Percent(f64);

impl ToString for Percent {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl From<f64> for Percent {
    fn from(value: f64) -> Self {
        Self((value - 1.0) * 100.0)
    }
}

impl From<Percent> for f64 {
    fn from(value: Percent) -> Self {
        (value.0 / 100.0) + 1.0
    }
}

fn eval_expr(expr: &String) -> Result<f64, String> {
    if expr.trim().is_empty() {
        return Ok(0.0);
    }
    match eval(expr).and_then(|x| x.as_number()) {
        Ok(result) => Ok(result),
        Err(err) => Err(format!("{:?}", err)),
    }
}

fn is_focused(node: Option<HtmlElement<Input>>) -> bool {
    let active = window().document().and_then(|doc| doc.active_element());
    match (active, node) {
        (Some(active), Some(node)) => active == ***node,
        _ => false,
    }
}

#[component]
fn ControlledBase(
    rates: Signal<Rates>,
    label: &'static str,
    description: &'static str,
    value: Signal<f64>,
    set_value: SignalSetter<f64>,
    percent: bool,
) -> impl IntoView {
    let node = create_node_ref::<Input>();

    let (local, set_local) = create_signal::<String>("".to_string());

    let text_value = (move || {
        let input = node.get();
        let local = local.get();
        let value = match percent {
            true => Percent::from(value.get()).to_string(),
            false => value.get().to_string(),
        };

        if is_focused(input) {
            local
        } else {
            value
        }
    })
    .into_signal();
    view! {
        <div class="field">
            <p class="label"><label for={label}>{label}</label></p>
            <p class="control">
                <input class="input" _ref=node id={label} type="text" prop:value=text_value
                    on:input=move |ev| {
                        let value = event_target_value(&ev);
                        set_local(event_target_value(&ev));

                        let global_value = eval_expr(&rates.with(|r| r.replace(&value))).unwrap_or_default();
                        set_value(match percent {
                            true => Percent(global_value).into(),
                            false => global_value,
                        });
                    }
                />
                " "
                {percent.then_some(view! {<span>%</span>})}
            </p>
            <Show
                when=move || text_value.with(|x| eval_expr(&rates.with(|r| r.replace(x)))).is_err()
            >
                <p class="error"><span>"⚠" invalid expression: {text_value.with(|x| rates.with(|r| r.replace(x)))}</span></p>
            </Show>
            <p class="help" ><span>{description}</span></p>
        </div>
    }
}

#[component]
fn Controlled(
    rates: Signal<Rates>,
    label: &'static str,
    description: &'static str,
    value: Signal<f64>,
    set_value: SignalSetter<f64>,
) -> impl IntoView {
    view! {
        <ControlledBase rates label description value set_value percent=false />
    }
}

#[component]
fn ControlledPercent(
    rates: Signal<Rates>,
    label: &'static str,
    description: &'static str,
    value: Signal<f64>,
    set_value: SignalSetter<f64>,
) -> impl IntoView {
    view! {
        <ControlledBase rates label description value set_value percent=true />
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
        <div class="field">
            <p class="label"><label for={label}>{label}</label></p>
            <p class="control"><input class="input" id={label} type="text" prop:value=value disabled="true"/></p>
            <p><span class="help">{description}</span></p>
        </div>
    }
}

#[component]
fn App() -> impl IntoView {
    let rates_resource = leptos::create_resource(|| (), |_| fetch_rates());

    let rates = (move || {
        rates_resource
            .get()
            .flatten()
            .unwrap_or_default()
            .data
            .rates
    })
    .into_signal();

    let window = window();
    let (amount, set_amount) = create_local_storage::<f64>(&window, "amount", 100.0);
    let (yearly, set_yearly) = create_local_storage::<f64>(&window, "yearly", 1.07);

    let daily = (move || yearly.get().powf(1.0 / 365.0)).into_signal();
    let set_daily = (move |y: f64| set_yearly.set(y.powf(365.0))).into_signal_setter();

    let monthly = (move || yearly.get().powf(1.0 / 12.0)).into_signal();
    let set_monthly = (move |y: f64| set_yearly.set(y.powf(12.0))).into_signal_setter();

    let yearly_5 = (move || yearly.get().powf(5.0)).into_signal();
    let set_yearly_5 = (move |y: f64| set_yearly.set(y.powf(1.0 / 5.0))).into_signal_setter();

    let yearly_10 = (move || yearly.get().powf(10.0)).into_signal();
    let set_yearly_10 = (move |y: f64| set_yearly.set(y.powf(1.0 / 10.0))).into_signal_setter();

    let daily_amount = (move || (daily.get() - 1.0) * amount.get()).into_signal();
    let set_daily_amount =
        (move |a: f64| set_amount.set(a / (daily.get() - 1.0))).into_signal_setter();

    let monthly_amount = (move || (monthly.get() - 1.0) * amount.get()).into_signal();
    let set_monthly_amount =
        (move |a: f64| set_amount.set(a / (monthly.get() - 1.0))).into_signal_setter();

    let yearly_amount = (move || (yearly.get() - 1.0) * amount.get()).into_signal();
    let set_yearly_amount =
        (move |a| set_amount.set(a / (yearly.get() - 1.0))).into_signal_setter();

    let yearly_5_amount = (move || (yearly_5.get() - 1.0) * amount.get()).into_signal();
    let set_yearly_5_amount =
        (move |a: f64| set_amount.set(a / (yearly_5.get() - 1.0))).into_signal_setter();

    let yearly_10_amount = (move || (yearly_10.get() - 1.0) * amount.get()).into_signal();
    let set_yearly_10_amount =
        (move |a: f64| set_amount.set(a / (yearly_10.get() - 1.0))).into_signal_setter();

    view! {
        <div class="columns is-mobile">
            <div class="column">
                <Controlled rates label="Amount" description="Amount to multiply by the interest" value=amount.into() set_value=set_amount.into() />
            </div>
            <div class="column">
                <Uncontrolled label="Calculated amount" description="Calculate value of the amount" value=amount.into()/>
            </div>
        </div>
        <div class="columns is-mobile">
            <div class="column">
                <ControlledPercent rates label="Daily" description="Daily interest rate in %" value=daily set_value=set_daily />
            </div>
            <div class="column">
                <Controlled rates label="Daily amount" description="Amount earned in a day." value=daily_amount set_value=set_daily_amount />
            </div>
        </div>
        <div class="columns is-mobile">
            <div class="column">
                <ControlledPercent rates label="Monthly" description="Monthly interest rate in %" value=monthly set_value=set_monthly />
            </div>
            <div class="column">
                <Controlled rates label="Monthly amount" description="Amount earned in a month." value=monthly_amount set_value=set_monthly_amount />
            </div>
        </div>
        <div class="columns is-mobile">
            <div class="column">
                <ControlledPercent rates label="Yearly" description="Yearly interest rate in %" value=yearly.into() set_value=set_yearly.into() />
            </div>
            <div class="column">
                <Controlled rates label="Yearly amount" description="Amount earned in a year." value=yearly_amount set_value=set_yearly_amount />
            </div>
        </div>
        <div class="columns is-mobile">
            <div class="column">
                <ControlledPercent rates label="5 years" description="5 years interest rate in %" value=yearly_5 set_value=set_yearly_5 />
            </div>
            <div class="column">
                <Controlled rates label="5 years amount" description="Amount earned in 5 years." value=yearly_5_amount set_value=set_yearly_5_amount />
            </div>
        </div>
        <div class="columns is-mobile">
            <div class="column">
                <ControlledPercent rates label="10 years" description="10 years interest rate in %" value=yearly_10 set_value=set_yearly_10 />
            </div>
            <div class="column">
                <Controlled rates label="10 years amount" description="Amount earned in 10 years." value=yearly_10_amount set_value=set_yearly_10_amount />
            </div>
        </div>
    }
}

fn main() {
    leptos::mount_to_body(|| {
        view! {
            <section class="section">
                <div class="container">
                    <h1 class="title">
                        Interest Rate Calculator
                    </h1>
                    <App/>
                </div>
            </section>
        }
    })
}
