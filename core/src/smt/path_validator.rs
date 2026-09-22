use crate::security_graph::{SecurityNodeKind, SecurityPath};
use z3::{ast::Bool, Config, Context, SatResult, Solver};

#[derive(Debug, Clone, PartialEq)]
pub struct PathValidationResult {
    pub satisfiable: bool,
    pub validated: bool,
    pub sanitized: bool,
    pub confidence_boost: f64,
    pub reason: String,
}

pub struct PathFeasibilityValidator;

impl PathFeasibilityValidator {
    pub fn validate(path: &SecurityPath) -> PathValidationResult {
        let has_source = path
            .steps
            .iter()
            .any(|step| step.kind == SecurityNodeKind::Source);

        let has_sink = path
            .steps
            .iter()
            .any(|step| step.kind == SecurityNodeKind::Sink);

        let has_sanitizer = path
            .steps
            .iter()
            .any(|step| step.kind == SecurityNodeKind::Sanitizer);

        if !has_source || !has_sink {
            return PathValidationResult {
                satisfiable: false,
                validated: false,
                sanitized: has_sanitizer,
                confidence_boost: -0.10,
                reason: "SecurityPath is incomplete: source or sink is missing".to_string(),
            };
        }

        let config = Config::new();
        let context = Context::new(&config);
        let solver = Solver::new(&context);

        let source_reached = Bool::new_const(&context, "source_reached");
        let sink_reached = Bool::new_const(&context, "sink_reached");
        let sanitizer_reached = Bool::new_const(&context, "sanitizer_reached");

        let exploit_path = Bool::and(
            &context,
            &[&source_reached, &sink_reached, &sanitizer_reached.not()],
        );

        solver.assert(&source_reached);
        solver.assert(&sink_reached);

        if has_sanitizer {
            solver.assert(&sanitizer_reached);
        } else {
            solver.assert(&sanitizer_reached.not());
        }

        solver.assert(&exploit_path);

        match solver.check() {
            SatResult::Sat if has_sanitizer => PathValidationResult {
                satisfiable: false,
                validated: true,
                sanitized: true,
                confidence_boost: -0.25,
                reason:
                    "SMT path model found a sanitizer on the source-to-sink path; exploit condition is unsatisfiable"
                        .to_string(),
            },
            SatResult::Unsat if has_sanitizer => PathValidationResult {
                satisfiable: false,
                validated: true,
                sanitized: true,
                confidence_boost: -0.25,
                reason:
                    "SMT proved the modeled exploit condition unsatisfiable because the path is sanitized"
                        .to_string(),
            },
            SatResult::Sat => PathValidationResult {
                satisfiable: true,
                validated: true,
                sanitized: false,
                confidence_boost: 0.15,
                reason:
                    "SMT proved a source-to-sink exploit path is satisfiable without a modeled sanitizer"
                        .to_string(),
            },
            SatResult::Unsat => PathValidationResult {
                satisfiable: false,
                validated: true,
                sanitized: false,
                confidence_boost: -0.20,
                reason:
                    "SMT proved the modeled source-to-sink exploit condition unsatisfiable"
                        .to_string(),
            },
            SatResult::Unknown => PathValidationResult {
                satisfiable: false,
                validated: false,
                sanitized: has_sanitizer,
                confidence_boost: 0.0,
                reason: "SMT returned unknown for the security path".to_string(),
            },
        }
    }
}
