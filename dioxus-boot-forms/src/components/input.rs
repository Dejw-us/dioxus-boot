use dioxus::prelude::*;

use crate::{
  components::UpdateFormData,
  hooks::{FormContext, use_form_context},
};

#[derive(Props, PartialEq, Clone)]
pub struct InputProps {
  pub value: &'static str,
  pub name: &'static str,
}

#[component]
pub fn Input<T: Clone + PartialEq + UpdateFormData + 'static>(props: InputProps) -> Element {
  let context = use_form_context::<T>();

  let onchange = move |event: Event<FormData>| {
    context()
      .data
      .borrow_mut()
      .update(props.name, &event.value());
  };

  rsx! {
    input { value: props.value, onchange }
  }
}
