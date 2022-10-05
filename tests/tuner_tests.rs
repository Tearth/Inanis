#[cfg(test)]
mod tuner_tests {
    use inanis::tuning::tuner;

    #[test]
    fn tuner_validate() {
        assert!(tuner::validate())
    }
}
