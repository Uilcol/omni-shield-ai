use crate::findings::finding::Finding;
use crate::smt::constraints::ConstraintBuilder;
use crate::smt::solver::SMTSolver;

pub struct SMTValidator;

impl SMTValidator {
    pub fn validate(mut finding: Finding) -> Finding {
        let constraints = ConstraintBuilder::from_finding(&finding);
        let result = SMTSolver::solve(&constraints);

        let boosted = (finding.confidence + result.confidence_boost).clamp(0.10, 1.00);

        finding.confidence = boosted;

        if result.validated {
            finding.recommendation = format!(
                "{} | SMT validated: {}",
                finding.recommendation, result.reason
            );
        } else {
            finding.recommendation =
                format!("{} | SMT review: {}", finding.recommendation, result.reason);
        }

        finding
    }

    pub fn validate_all(findings: Vec<Finding>) -> Vec<Finding> {
        findings.into_iter().map(Self::validate).collect()
    }
}
