use anyhow::Result;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{Duration, sleep};
use uuid::Uuid;

use crate::api_server;
use crate::config::Config;
use crate::paths::GoatPaths;
use crate::runtime::GoatRuntime;

pub async fn run(mut rt: GoatRuntime) -> Result<()> {
    let host = rt.config.daemon.host.clone();
    let port = rt.config.daemon.port;
    let auth_required = rt.config.daemon.auth_required;

    let token_path = rt.paths.data_dir.join("daemon.token");
    let pid_path = rt.paths.data_dir.join("daemon.pid");
    let log_path = rt.paths.data_dir.join("daemon.log");

    // Write PID
    let pid = std::process::id();
    fs::write(&pid_path, pid.to_string())?;

    // Load or generate token
    let token = if auth_required {
        if token_path.exists() {
            fs::read_to_string(&token_path)?.trim().to_string()
        } else {
            let new_token = format!("{}{}", Uuid::new_v4(), Uuid::new_v4()).replace("-", "");
            let mut file = OpenOptions::new()
                .create(true)
                .write(true)
                .mode(0o600)
                .open(&token_path)?;
            file.write_all(new_token.as_bytes())?;
            new_token
        }
    } else {
        String::new()
    };

    println!("[DAEMON] Starting GOAT Daemon (PID: {})", pid);
    if auth_required {
        println!("[DAEMON] Token Auth Required. Token path: {:?}", token_path);
    } else {
        println!("[DAEMON] WARNING: Authentication is disabled!");
    }

    let shared_rt = Arc::new(Mutex::new(rt));

    // Spawn scheduler ticker
    let rt_for_scheduler = shared_rt.clone();
    tokio::spawn(async move {
        loop {
            sleep(Duration::from_millis(5000)).await; // 5-second tick in daemon
            let mut rt = rt_for_scheduler.lock().await;
            let jobs = rt.scheduler_manager.tick();
            for job in jobs {
                let msg = format!(
                    "[DAEMON SCHEDULE] Executing job {}: {}",
                    job.id, job.prompt_or_command
                );
                // Log to daemon.log
                if let Ok(mut log_file) =
                    OpenOptions::new().create(true).append(true).open(&log_path)
                {
                    let _ = writeln!(log_file, "{}", msg);
                }

                rt.job_tracker.add_job(crate::jobs::BackgroundJob {
                    id: job.id.clone(),
                    r#type: "scheduled".to_string(),
                    status: "running".to_string(),
                    started_at: chrono::Utc::now().to_rfc3339(),
                    finished_at: None,
                    output_preview: None,
                    error: None,
                    approval_status: None,
                });

                rt.scheduler_manager.log_audit(&format!(
                    "Daemon Executed job {}: {}",
                    job.id, job.prompt_or_command
                ));
            }
        }
    });

    // Start API Server (this blocks)
    tokio::select! {
        res = api_server::start_server(&host, port, auth_required, token, shared_rt.clone()) => {
            if let Err(e) = res {
                eprintln!("[DAEMON ERROR] API Server crashed: {}", e);
            }
        }
        _ = tokio::signal::ctrl_c() => {
            println!("\n[DAEMON] Received Ctrl+C, shutting down...");
        }
    }

    // Cleanup
    let mut final_rt = shared_rt.lock().await;
    let _ = final_rt.mcp_manager.shutdown_all().await;
    let _ = fs::remove_file(pid_path);
    println!("[DAEMON] Clean shutdown complete.");

    Ok(())
}

pub fn get_status(paths: &GoatPaths) {
    let pid_path = paths.data_dir.join("daemon.pid");
    if pid_path.exists() {
        if let Ok(pid_str) = fs::read_to_string(&pid_path) {
            println!("[DAEMON] Status: Running (PID: {})", pid_str.trim());
            // TODO: Optional curl check to health endpoint
        }
    } else {
        println!("[DAEMON] Status: Stopped (No daemon.pid found)");
    }
}

pub fn print_doctor(paths: &GoatPaths, config: &Config) {
    println!("[DAEMON DOCTOR]");
    println!("  Enabled in config: {}", config.daemon.enabled);
    println!(
        "  Bind Address: {}:{}",
        config.daemon.host, config.daemon.port
    );
    println!("  Auth Required: {}", config.daemon.auth_required);

    let token_path = paths.data_dir.join("daemon.token");
    if token_path.exists() {
        println!("  Token Status: Exists at {}", token_path.display());
    } else {
        println!("  Token Status: Missing (will be generated on start)");
    }

    let pid_path = paths.data_dir.join("daemon.pid");
    if pid_path.exists() {
        if let Ok(pid_str) = fs::read_to_string(&pid_path) {
            // Check if process exists (Unix only simple check)
            let is_stale = if let Ok(pid) = pid_str.trim().parse::<i32>() {
                std::process::Command::new("kill")
                    .arg("-0")
                    .arg(pid_str.trim())
                    .stdout(std::process::Stdio::null())
                    .stderr(std::process::Stdio::null())
                    .status()
                    .map(|s| !s.success())
                    .unwrap_or(true)
            } else {
                true
            };

            if is_stale {
                println!("  PID Status: STALE (PID {} not running)", pid_str.trim());
            } else {
                println!("  PID Status: ACTIVE (PID {})", pid_str.trim());
            }
        }
    } else {
        println!("  PID Status: None");
    }
}
