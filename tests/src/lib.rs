use extism_pdk::*;
use xtp_test;

#[plugin_fn]
pub fn test() -> FnResult<()> {
    let password = "password";
    let hash: String = xtp_test::call("hash", password)?;

    xtp_test::assert(
        format!("hash: {hash}"),
        !hash.is_empty(),
        format!("Password hash: {hash}"),
    );

    let verify: String = xtp_test::call(
        "verify",
        format!(r#"{{"password": "{password}", "hash": "{hash}"}}"#),
    )?;

    xtp_test::assert("verify true", verify == "true", format!("Verify: {verify}"));

    let verify: String = xtp_test::call(
        "verify",
        format!(r#"{{"password": "{password}", "hash": "xxx"}}"#),
    )?;

    xtp_test::assert(
        "verify false: wrong hash",
        verify == "false",
        format!("Verify: {verify}"),
    );

    let verify: String = xtp_test::call(
        "verify",
        r#"{"password": "wrong-password", "hash": "$argon2id$v=19$m=19456,t=2,p=1$U4x/lrFkvxuXu59LtHLonw$SJXaZB3xACnlG5osLzhQvCKQkPQiuNkknkdi6QzcP6k"}"#,
    )?;

    xtp_test::assert(
        "verify false: wrong password",
        verify == "false",
        format!("Verify: {verify}"),
    );

    let verify: String = xtp_test::call(
        "verify",
        format!(r#"{{"password": "{password}", "hash": ""}}"#),
    )?;

    xtp_test::assert(
        "verify false: empty hash",
        verify == "false",
        format!("Verify: {verify}"),
    );

    let verify: String = xtp_test::call("verify", format!(r#"{{"password": "", "hash": "xxx"}}"#))?;

    xtp_test::assert(
        "verify false: empty password",
        verify == "false",
        format!("Verify: {verify}"),
    );

    Ok(())
}
