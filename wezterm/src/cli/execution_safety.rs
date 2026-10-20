/*
Copyright (c) 2026 AI Venture Holdings LLC
Licensed under the Business Source License 1.1
*/

//! Dry-run-by-default guards for `cx ask` and related natural-language entry points.

use std::io::{self, Write};

/// True when a matched or suggested `cx` CLI invocation is read-only and may run without `--execute`.
pub fn is_read_only_cx_cli(command: &str) -> bool {
    let cmd = command.trim();
    cmd.starts_with("cx snapshots -l")
        || cmd.starts_with("cx snapshots --list")
        || cmd == "cx new --list"
        || cmd.starts_with("cx daemon status")
        || cmd.starts_with("cx daemon health")
        || cmd.starts_with("cx daemon alerts")
}

/// True when a shell command is treated as read-only (introspection only).
pub fn is_read_only_shell_command(command: &str) -> bool {
    let cmd = command.trim();
    if cmd.is_empty() {
        return true;
    }
    if is_read_only_cx_cli(cmd) {
        return true;
    }

    let first = cmd
        .split_whitespace()
        .next()
        .unwrap_or("")
        .to_ascii_lowercase();

    matches!(
        first.as_str(),
        "ls" | "cat"
            | "head"
            | "tail"
            | "pwd"
            | "whoami"
            | "df"
            | "free"
            | "echo"
            | "which"
            | "type"
            | "date"
            | "uptime"
            | "id"
            | "groups"
            | "printenv"
            | "env"
    ) || cmd.starts_with("git status")
        || cmd.starts_with("git log")
        || cmd.starts_with("git branch")
        || cmd.starts_with("docker ps")
        || cmd.starts_with("docker images")
}

/// Print the dry-run preview banner and exact command(s).
pub fn print_dry_run_preview(commands: &[&str]) {
    eprintln!();
    eprintln!("┌─ Dry-run preview (no changes applied) ─────────────────");
    for c in commands {
        eprintln!("│  $ {}", c);
    }
    eprintln!("└────────────────────────────────────────────────────────");
    eprintln!("To run: add --execute (-d) and confirm at the prompt, or use -d -y");
    eprintln!();
}

/// Whether execution is allowed for a state-changing command.
///
/// `-y` / `auto_confirm` is ignored unless `execute_flag` is set — preview-only stays preview-only.
pub fn execution_gate(execute_flag: bool, auto_confirm: bool) -> ExecutionGate {
    if !execute_flag {
        return ExecutionGate::PreviewOnly;
    }
    if auto_confirm && execute_flag {
        return ExecutionGate::RunNow;
    }
    ExecutionGate::ConfirmFirst
}

/// Effective auto-confirm: `-y` only applies together with `--execute` / `-d`.
pub fn effective_auto_confirm(execute_flag: bool, auto_confirm: bool) -> bool {
    execute_flag && auto_confirm
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutionGate {
    PreviewOnly,
    ConfirmFirst,
    RunNow,
}

/// Interpret the answer to a `[Y/n]` prompt (default yes when interactive).
pub fn confirmed_default_yes(answer: Option<&str>) -> bool {
    match answer {
        None => false,
        Some(line) => {
            let line = line.trim();
            line.is_empty() || line.eq_ignore_ascii_case("y")
        }
    }
}

/// Interpret the answer to a `[y/N]` prompt (default no).
pub fn confirmed_default_no(answer: Option<&str>) -> bool {
    match answer {
        None => false,
        Some(line) => line.trim().eq_ignore_ascii_case("y"),
    }
}

/// Read a confirmation line; `None` means EOF (never consent).
pub fn read_confirmation_line() -> io::Result<Option<String>> {
    let mut input = String::new();
    let bytes_read = io::stdin().read_line(&mut input)?;
    if bytes_read == 0 {
        Ok(None)
    } else {
        Ok(Some(input))
    }
}

/// Prompt for `[Y/n]` with default yes; returns false on EOF or explicit no.
pub fn prompt_default_yes(message: &str) -> io::Result<bool> {
    eprint!("{}", message);
    io::stderr().flush()?;
    let answer = read_confirmation_line()?;
    Ok(confirmed_default_yes(answer.as_deref()))
}

/// Prompt for `[y/N]` with default no; returns false on EOF or empty line.
pub fn prompt_default_no(message: &str) -> io::Result<bool> {
    eprint!("{}", message);
    io::stderr().flush()?;
    let answer = read_confirmation_line()?;
    Ok(confirmed_default_no(answer.as_deref()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_only_cx_listing() {
        assert!(is_read_only_cx_cli("cx snapshots -l"));
        assert!(is_read_only_cx_cli("cx new --list"));
        assert!(!is_read_only_cx_cli("cx save my-work"));
    }

    #[test]
    fn read_only_shell_introspection() {
        assert!(is_read_only_shell_command("ls -la"));
        assert!(is_read_only_shell_command("git status"));
        assert!(!is_read_only_shell_command("rm -rf /"));
    }

    #[test]
    fn execution_gate_preview_by_default() {
        assert_eq!(execution_gate(false, false), ExecutionGate::PreviewOnly);
        assert_eq!(
            execution_gate(false, true),
            ExecutionGate::PreviewOnly,
            "-y without --execute must not run"
        );
        assert_eq!(execution_gate(true, true), ExecutionGate::RunNow);
        assert_eq!(execution_gate(true, false), ExecutionGate::ConfirmFirst);
    }

    #[test]
    fn effective_auto_confirm_requires_execute() {
        assert!(!effective_auto_confirm(false, true));
        assert!(effective_auto_confirm(true, true));
    }

    #[test]
    fn eof_is_never_consent_yes_default() {
        assert!(!confirmed_default_yes(None));
    }

    #[test]
    fn eof_is_never_consent_no_default() {
        assert!(!confirmed_default_no(None));
        assert!(!confirmed_default_no(Some("\n")));
    }

    #[test]
    fn explicit_yes_confirms_no_default() {
        assert!(confirmed_default_no(Some("y\n")));
    }
}
