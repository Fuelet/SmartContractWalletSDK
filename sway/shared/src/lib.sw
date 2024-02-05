library;

abi RecoveryChecker {
    #[storage(read)]
    fn check_cooldown_passed();

    #[storage(write)]
    fn start_recovery();

    #[storage(write)]
    fn stop_recovery();
}
