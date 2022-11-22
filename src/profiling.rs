use yew::prelude::*;

use crate::navigation::Navigation;

use yewdux::prelude::*;
use yewdux_input::{Checkbox, InputDispatch};

#[derive(Store, Default, PartialEq, Clone)]
struct Form {
    sort_data: Checkbox,
    dataset: Option<String>,
}

struct CheckboxData {
    label: String,
    value: String,
}

struct InputCheckboxProps {
    

}

#[function_component]
fn InputCheckbox() -> Html {
    let (store, dispatch) = use_store::<Form>();
    let onchange = dispatch.input_mut(|s, value| {
        s.sort_data = value;
    });

    html! {
        <>
        <p>{store.sort_data.checked()}</p>
        <input type="checkbox" {onchange} />
        </>
    }
}

#[function_component]
fn InputRadio() -> Html {
    let (store, dispatch) = use_store::<Form>();
    let onchange = dispatch.input_mut(|s, value| {
        s.dataset = Some(value);
    });

    html! {
        <>
        <p>{store.dataset.clone().unwrap_or_default()}</p>
        <input onchange={onchange.clone()} type="radio" id="dog" name="animal" value="cat"/ >
        <label for="cat">{ "cat" }</label><br />
        <input onchange={onchange.clone()} type="radio" id="cat" name="animal" value="dog"/ >
        <label for="dog">{"dog"}</label><br />
        </>
    }
}

#[function_component(Profiling)]
pub fn profiling() -> Html {

    html!{
        <div>
            <h2>{"Profiling"}</h2>
            <Navigation />
            
        </div>
    }
}