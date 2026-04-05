use rsnark::jsnark_interface::zkay_interface::run_snark;
fn main() {
    run_snark::mains(
        5,
        &[
            "run",
            "keygen",
            "/Users/lisheng/mygit/arron/zkay-rs/survey_compiled/zk__Verify_Survey_publish_results_out",
            "/Users/lisheng/mygit/arron/zkay-rs/survey_compiled/zk__Verify_Survey_publish_results_out",
            "1",
        ],
    );
}
