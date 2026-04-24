use rsnark::jsnark_interface::zkay_interface::run_snark;
use tracing::{span,Level};
fn main() {
   tracing_subscriber::fmt()
        
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::FULL)
        .init();
 let span=span!(Level::INFO,"main");
    let _=span.enter();
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
