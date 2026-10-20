/*
Copyright (c) 2026 AI Venture Holdings LLC
Licensed under the Business Source License 1.1
*/

//! Local-first audit log and reversible operation history for CX CLI actions.

use super::command_sandbox::run_sandboxed_shell;
use anyhow::{Context, Result};
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::fs::{self, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;

const AUDIT_LOG: &str = "audit.log";
const TRANSACTIONS_LOG: &str = "transactions.jsonl";

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditEntry {
    pub ts: String,
    pub event: String,
    pub command: Option<String>,
    pub detail: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionRecord {
    pub id: String,
    pub ts: String,
    pub command: String,
    pub undo: String,
}

pub fn cx_data_dir() -> PathBuf {
    dirs::home_dir()
        .map(|h| h.join(".cx"))
        .unwrap_or_else(|| PathBuf::from(".cx"))
}

fn audit_path() -> PathBuf {
    cx_data_dir().join(AUDIT_LOG)
}

fn transactions_path() -> PathBuf {
    cx_data_dir().join(TRANSACTIONS_LOG)
}

fn ensure_data_dir() -> Result<PathBuf> {
    let dir = cx_data_dir();
    fs::create_dir_all(&dir).with_context(|| format!("create {}", dir.display()))?;
    Ok(dir)
}

/// Append a line to `~/.cx/audit.log` (JSON per line).
pub fn append_audit(event: &str, command: Option<&str>, detail: serde_json::Value) -> Result<()> {
    ensure_data_dir()?;
    let entry = AuditEntry {
        ts: chrono::Utc::now().to_rfc3339(),
        event: event.to_string(),
        command: command.map(|s| s.to_string()),
        detail,
    };
    let line = serde_json::to_string(&entry)?;
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(audit_path())?;
    writeln!(file, "{}", line)?;
    Ok(())
}

/// Record a reversible command and return the transaction id.
pub fn record_transaction(command: &str, undo: &str) -> Result<String> {
    ensure_data_dir()?;
    let id = format!("tx-{}", chrono::Utc::now().timestamp_millis());
    let record = TransactionRecord {
        id: id.clone(),
        ts: chrono::Utc::now().to_rfc3339(),
        command: command.to_string(),
        undo: undo.to_string(),
    };
    let line = serde_json::to_string(&record)?;
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(transactions_path())?;
    writeln!(file, "{}", line)?;
    Ok(id)
}

fn read_last_transaction() -> Result<Option<TransactionRecord>> {
    let path = transactions_path();
    if !path.exists() {
        return Ok(None);
    }
    let file = fs::File::open(&path)?;
    let reader = BufReader::new(file);
    let mut last: Option<TransactionRecord> = None;
    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        last = Some(serde_json::from_str(&line)?);
    }
    Ok(last)
}

/// Undo the most recent recorded transaction.
pub fn undo_last_transaction() -> Result<()> {
    let record = read_last_transaction()?
        .ok_or_else(|| anyhow::anyhow!("No reversible operations in ~/.cx/transactions.jsonl"))?;

    eprintln!("Undoing {} ({})", record.id, record.command);
    eprintln!("  $ {}", record.undo);

    let status = run_sandboxed_shell(&record.undo).context("failed to run undo command")?;

    if !status.success() {
        anyhow::bail!("Undo command failed with {:?}", status.code());
    }

    append_audit(
        "undo",
        Some(&record.undo),
        serde_json::json!({
            "transaction_id": record.id,
            "original_command": record.command,
        }),
    )?;

    eprintln!("✅ Undone.");
    Ok(())
}

/// Show the tail of the audit log (for demos and troubleshooting).
pub fn show_audit_tail(lines: usize) -> Result<()> {
    let path = audit_path();
    if !path.exists() {
        println!("(no audit log at {})", path.display());
        return Ok(());
    }
    let file = fs::File::open(&path)?;
    let reader = BufReader::new(file);
    let all: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    let start = all.len().saturating_sub(lines);
    for line in &all[start..] {
        println!("{}", line);
    }
    Ok(())
}

/// Remove CX local data (audit, transactions, license). Does not remove the Debian package.
pub fn clean_local_data() -> Result<()> {
    let dir = cx_data_dir();
    if dir.exists() {
        fs::remove_dir_all(&dir).with_context(|| format!("remove {}", dir.display()))?;
        println!("Removed {}", dir.display());
    } else {
        println!("No {} directory", dir.display());
    }
    Ok(())
}

#[derive(Debug, Parser, Clone)]
pub struct UndoCommand {}

impl UndoCommand {
    pub fn run(&self) -> Result<()> {
        undo_last_transaction()
    }
}

#[derive(Debug, Parser, Clone)]
pub struct AuditCommand {
    /// Number of recent audit entries to print
    #[arg(long, short = 'n', default_value_t = 5)]
    pub tail: usize,
}

impl AuditCommand {
    pub fn run(&self) -> Result<()> {
        show_audit_tail(self.tail)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn audit_roundtrip_in_temp_home() {
        let tmp = env::temp_dir().join(format!("cx-audit-test-{}", std::process::id()));
        fs::create_dir_all(&tmp).unwrap();
        // ops uses dirs::home_dir; we only test serialization helpers here
        let entry = AuditEntry {
            ts: "2026-01-01T00:00:00Z".into(),
            event: "test".into(),
            command: Some("echo hi".into()),
            detail: serde_json::json!({}),
        };
        let line = serde_json::to_string(&entry).unwrap();
        let back: AuditEntry = serde_json::from_str(&line).unwrap();
        assert_eq!(back.event, "test");
        let _ = fs::remove_dir_all(tmp);
    }
}
