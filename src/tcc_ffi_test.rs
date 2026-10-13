use super::*;
use crate::support_test::cfg;

fn with(f: impl FnOnce(&mut TccConfig)) -> Result<i32, ProbeError> {
    let mut c = cfg().tcc;
    f(&mut c);
    preflight(&c)
}

#[test]
fn real_preflight_succeeds() {
    assert!(with(|_| {}).is_ok());
}

#[test]
fn framework_with_nul() {
    assert_eq!(
        with(|c| c.framework = "a\0b".into()),
        Err(ProbeError::InvalidCString)
    );
}

#[test]
fn framework_missing() {
    assert_eq!(
        with(|c| c.framework = "/nonexistent".into()),
        Err(ProbeError::LoadFailed)
    );
}

#[test]
fn symbol_with_nul() {
    assert_eq!(
        with(|c| c.symbol = "a\0b".into()),
        Err(ProbeError::InvalidCString)
    );
}

#[test]
fn symbol_missing() {
    assert_eq!(
        with(|c| c.symbol = "NoSuchSymbol".into()),
        Err(ProbeError::SymbolMissing)
    );
}

#[test]
fn service_with_nul() {
    assert_eq!(
        with(|c| c.service = "a\0b".into()),
        Err(ProbeError::InvalidCString)
    );
}

#[test]
fn service_not_encodable() {
    let r = with(|c| {
        c.cf_encoding = 0xFFFF_FFFF;
    });
    assert_eq!(r, Err(ProbeError::CfStringFailed));
}
