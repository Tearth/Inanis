#[cfg(test)]
mod tuner_tests {
    use inanis::utils::tuner;

    #[test]
    fn tuner_validate() {
        assert!(tuner::validate())
    }
}
