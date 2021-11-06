#[cfg(test)]
mod tuner_tests {
    use inanis::utils::tuner;
    use std::sync::Once;

    static INIT: Once = Once::new();

    #[test]
    fn tuner_validate() {
        INIT.call_once(|| {
            inanis::init();
        });

        assert!(tuner::validate())
    }
}
