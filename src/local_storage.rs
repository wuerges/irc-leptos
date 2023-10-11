use leptos::{create_signal, IntoSignalSetter, Signal, SignalSetter};
use std::str::FromStr;
use web_sys::Window;

pub fn create_local_storage<T>(
    window: &Window,
    key: &'static str,
    default: T,
) -> (Signal<T>, SignalSetter<T>)
where
    T: FromStr + ToString + Clone,
    <T as FromStr>::Err: std::fmt::Debug,
{
    let local_storage = window
        .local_storage()
        .expect("error getting local storage")
        .expect("local storage is None");
    let storage_value = local_storage.get_item(key).expect(&format!(
        "failed to get item from local storage with key: `{}'",
        key
    ));

    let initial_value = match storage_value {
        Some(v) => v.parse().expect(&format!(
            "failed parsing local storage value `{}' for key: `{}'",
            key, v
        )),
        None => default,
    };

    let (signal, set_signal) = create_signal::<T>(initial_value);

    let storage_writer = (move |new_value: T| {
        set_signal(new_value.clone());
        local_storage
            .set_item(key, new_value.clone().to_string().as_str())
            .expect(&format!(
                "failed to set item to local storage with key: `{}'",
                key
            ));
    })
    .into_signal_setter();

    (signal.into(), storage_writer)
}
