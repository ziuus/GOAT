impl ExternalAgentManager {
    pub fn record_run(&self, run: &ExternalAgentRun, stdout: Option<&str>, stderr: Option<&str>) {
        use std::fs::{self, OpenOptions};
        use std::io::Write;

        let run_dir = self.data_dir.join("external-agent-runs").join(&run.run_id);
        if !run_dir.exists() {
            let _ = fs::create_dir_all(&run_dir);
        }

        let mut run_to_save = run.clone();

        if let Some(out) = stdout {
            let stdout_path = run_dir.join("stdout.log");
            let _ = fs::write(&stdout_path, out);
            run_to_save.stdout_log_path = Some(stdout_path);
        }

        if let Some(err) = stderr {
            let stderr_path = run_dir.join("stderr.log");
            let _ = fs::write(&stderr_path, err);
            run_to_save.stderr_log_path = Some(stderr_path);
        }

        let meta_path = run_dir.join("metadata.json");
        if let Ok(json_str) = serde_json::to_string_pretty(&run_to_save) {
            let _ = fs::write(&meta_path, json_str);
        }

        // Also append to the JSONL global log
        let jsonl_path = self.data_dir.join("external-agent-runs.jsonl");
        if let Ok(mut json_file) = OpenOptions::new().create(true).append(true).open(&jsonl_path) {
            if let Ok(json_str) = serde_json::to_string(&run_to_save) {
                let _ = writeln!(json_file, "{}", json_str);
            }
        }
    }
}
