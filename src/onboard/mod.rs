pub mod wizard;

pub use wizard::{
    run_channels_repair_wizard, run_models_refresh, run_quick_setup_with_force,
    run_wizard_with_force,
};

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_reexport_exists<F>(_value: F) {}

    #[test]
    fn wizard_functions_are_reexported() {
        assert_reexport_exists(run_wizard_with_force);
        assert_reexport_exists(run_channels_repair_wizard);
        assert_reexport_exists(run_quick_setup_with_force);
        assert_reexport_exists(run_models_refresh);
    }
}
