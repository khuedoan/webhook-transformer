use jrsonnet_evaluator::{State, Thunk, Val};
use jrsonnet_stdlib::StateExt;
use serde_json::{json, Value};

pub fn transform(jsonnet_config: String, payload: Value) -> Value {
    let state = State::default();

    state.with_stdlib();
    state.add_global(
        "body".into(),
        Thunk::evaluated(Val::from_serde(payload).expect("failed to convert body to jsonnet")),
    );

    json!(state
        .evaluate_snippet("config.jsonnet", jsonnet_config)
        .expect("failed to evaluate config"))
}

#[cfg(test)]
mod tests {
    use serde_json::{from_str, Value};
    use std::fs;

    fn run_example(name: &str) {
        let config: String = fs::read_to_string(format!("examples/{name}/config.jsonnet"))
            .expect("failed to read config file");

        let input: Value = from_str(
            &fs::read_to_string(format!("examples/{name}/input.json"))
                .expect("failed to read input file"),
        )
        .expect("failed parse input to JSON");

        let output: Value = from_str(
            &fs::read_to_string(format!("examples/{name}/output.json"))
                .expect("failed to read output file"),
        )
        .expect("failed parse output to JSON");

        assert_eq!(crate::transform(config, input), output);
    }

    #[test]
    fn noop() {
        run_example("noop");
    }

    #[test]
    fn basic() {
        run_example("basic");
    }

    #[test]
    fn alertmanager_to_ntfy() {
        run_example("alertmanager-to-ntfy");
    }
}
