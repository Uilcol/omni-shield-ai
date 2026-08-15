use omniuil_core::runtime::executor::RuntimeExecutor;
use omniuil_core::output::compliance::ComplianceReport;

pub fn run(path: &str) {
    let findings = RuntimeExecutor::execute(path);
    ComplianceReport::export(&findings, "omniuil-compliance-report.txt");
}
