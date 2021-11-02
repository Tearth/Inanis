#[cfg(test)]
mod tuner_tests {
    use ina::utils::tuner;
    use std::sync::Once;

    static INIT: Once = Once::new();

    #[test]
    fn tuner_validate() {
        INIT.call_once(|| {
            ina::init();
        });

        assert_eq!(tuner::validate(), true);
    }
}
