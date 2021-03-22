use std::fmt::{Display, Formatter};

#[derive(Default, Clone, Copy)]
pub struct ComboBoxModel<T> {
    pub label: &'static str,
    pub value: T,
}

impl<T> ComboBoxModel<T> {
    pub fn new(label: &'static str, value: T) -> ComboBoxModel<T> {
        ComboBoxModel { label, value }
    }
}

impl<T> Display for ComboBoxModel<T> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.label)
    }
}