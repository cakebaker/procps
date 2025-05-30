// This file is part of the uutils procps package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

#[cfg(unix)]
use uutests::new_ucmd;
#[cfg(unix)]
use uutests::util::TestScenario;
#[cfg(unix)]
use uutests::util_name;

#[cfg(unix)]
#[test]
fn test_no_args() {
    new_ucmd!()
        .fails()
        .code_is(2)
        .no_stdout()
        .stderr_contains("no matching criteria specified");
}

#[cfg(unix)]
#[test]
fn test_non_matching_pattern() {
    new_ucmd!()
        .arg("NONMATCHING")
        .fails()
        .code_is(1)
        .no_output();
}

#[cfg(unix)]
#[test]
fn test_too_many_patterns() {
    new_ucmd!()
        .arg("sh")
        .arg("sh")
        .fails()
        .code_is(2)
        .no_stdout()
        .stderr_contains("only one pattern can be provided");
}

#[cfg(unix)]
#[test]
fn test_invalid_arg() {
    new_ucmd!().arg("--definitely-invalid").fails().code_is(1);
}

#[cfg(target_os = "linux")]
#[test]
fn test_inverse() {
    new_ucmd!()
        .arg("-0")
        .arg("--inverse")
        .arg("NONEXISTENT")
        .fails()
        .stderr_contains("Permission denied");
}

#[cfg(unix)]
#[test]
fn test_help() {
    new_ucmd!().arg("--help").succeeds();
}

#[test]
#[cfg(target_os = "linux")]
fn test_too_long_pattern() {
    new_ucmd!()
        .arg("THIS_IS_OVER_16_CHARS")
        .fails()
        .code_is(1)
        .stderr_contains("pattern that searches for process name longer than 15 characters will result in zero matches");
}
