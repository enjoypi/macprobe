use super::*;

fn outcome(status: Status) -> Outcome {
    Outcome {
        id: "id".into(),
        name: "name".into(),
        status,
        detail: String::new(),
    }
}

#[test]
fn overall_empty_is_granted() {
    assert_eq!(overall(&[]), Status::Granted);
}

#[test]
fn overall_all_granted() {
    assert_eq!(
        overall(&[outcome(Status::Granted), outcome(Status::Granted)]),
        Status::Granted
    );
}

#[test]
fn overall_denied_wins_over_granted() {
    assert_eq!(
        overall(&[outcome(Status::Granted), outcome(Status::Denied)]),
        Status::Denied
    );
}

#[test]
fn overall_unknown_wins_over_denied() {
    assert_eq!(
        overall(&[outcome(Status::Denied), outcome(Status::Unknown)]),
        Status::Unknown
    );
}

#[test]
fn status_as_str() {
    assert_eq!(Status::Granted.as_str(), "granted");
    assert_eq!(Status::Denied.as_str(), "denied");
    assert_eq!(Status::Unknown.as_str(), "unknown");
}
