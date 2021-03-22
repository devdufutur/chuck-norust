use std::sync::Mutex;
use once_cell::sync::Lazy;
use crate::combobox_model::ComboBoxModel;

pub static LAST_QUOTE: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(String::new()));

lazy_static! {
    pub static ref DURATION_MODEL: Vec<ComboBoxModel<u32>> = {
        let model = vec![
            ComboBoxModel::new("Every 10 seconds", 10000),
            ComboBoxModel::new("Every 15 seconds", 15000),
            ComboBoxModel::new("Every 20 seconds", 20000),
            ComboBoxModel::new("Every 30 seconds", 30000),
            ComboBoxModel::new("Every 45 seconds", 45000),
            ComboBoxModel::new("Every minute", 60000),
            ComboBoxModel::new("Every 2 minutes", 2 * 60000),
            ComboBoxModel::new("Every 5 minutes", 5 * 60000),
            ComboBoxModel::new("Every 10 minutes", 10 * 60000),
            ComboBoxModel::new("Every 15 minutes", 15 * 60000),
            ComboBoxModel::new("Every 20 minutes", 20 * 60000),
            ComboBoxModel::new("Every 30 minutes", 30 * 60000),
            ComboBoxModel::new("Every 45 minutes", 45 * 60000),
            ComboBoxModel::new("Every hour", 60 * 60000),
        ];
        model
    };
}