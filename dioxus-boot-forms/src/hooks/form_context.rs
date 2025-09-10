use dioxus::prelude::*;
use std::{cell::RefCell, rc::Rc};

#[derive(Clone)]
pub struct FormContext<T> {
  pub data: Rc<RefCell<T>>,
}

pub fn use_form_context<T>() -> Signal<FormContext<T>> {
  use_context::<Signal<FormContext<T>>>()
}
