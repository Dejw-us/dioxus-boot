use std::{cell::RefCell, rc::Rc};

use dioxus::prelude::*;

use crate::hooks::FormContext;

pub struct FormSubmitEvent<T> {
  pub event: FormEvent,
  pub data: T,
}

pub trait UpdateFormData {
  fn update(&mut self, name: &str, value: &str);
}

#[derive(Props, PartialEq, Clone)]
pub struct FormProps<T: Clone + PartialEq + UpdateFormData + 'static> {
  pub default: T,
  pub children: Element,
  pub onsubmit: EventHandler<FormSubmitEvent<T>>,
}

#[component]
pub fn Form<T: Clone + PartialEq + UpdateFormData + 'static>(props: FormProps<T>) -> Element {
  let state = use_signal(|| FormContext {
    data: Rc::new(RefCell::new(props.default)),
  });

  provide_context(state);

  let onsubmit = move |event: Event<FormData>| {
    event.prevent_default();
    props.onsubmit.call(FormSubmitEvent {
      event,
      data: (*state().data.borrow()).clone(),
    })
  };

  rsx! {
    form { onsubmit, {props.children} }
  }
}
