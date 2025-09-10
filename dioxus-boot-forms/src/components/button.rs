use dioxus::prelude::*;

use crate::{components::UpdateFormData, hooks::use_form_context};

#[derive(Props, PartialEq, Clone, Debug)]
pub struct ButtonProps<T: Clone + UpdateFormData + PartialEq + 'static> {
  children: Element,
  onclick: EventHandler<T>,
}

#[component]
pub fn Button<T: Clone + UpdateFormData + PartialEq + 'static>(props: ButtonProps<T>) -> Element {
  let context = use_form_context::<T>();

  let onclick = move |_| {
    props.onclick.call((*context().data.borrow()).clone());
  };

  rsx! {
    button { onclick, "type": "button", {props.children} }
  }
}
