/*
 * Compiler-Bot: compiler bot for Unofficial.CSE
 * Copyright (C) 2025  Unofficial.CSE contributors
 *
 * Compiler-Bot is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published
 * by the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * Compiler-Bot is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with Compiler-Bot.  If not, see <https://www.gnu.org/licenses/>.
 */

use std::{process::Stdio, time::Duration};

use tokio::{io::AsyncWriteExt, process::Command};
use uuid::Uuid;

use crate::{config::BotConfig, runners::LANGUAGES};

#[derive(Debug)]
pub struct ExecutionResult {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: Option<i32>,
    pub timed_out: bool,
}

pub struct DockerExecutor {
    pub config: BotConfig,
}

impl DockerExecutor {
    pub fn new() -> Self {
        Self {
            config: BotConfig::default(),
        }
    }

    pub fn supported_languages(&self) -> Vec<&'static str> {
        LANGUAGES.keys().copied().collect()
    }

    pub async fn execute(&self, language: &str, code: &str) -> Result<ExecutionResult, String> {
        // Validate input
        if code.trim().is_empty() {
            return Ok(ExecutionResult {
                stdout: String::new(),
                stderr: "Error: Code is empty or contains only whitespace.".into(),
                exit_code: Some(1),
                timed_out: false,
            });
        }

        let config = LANGUAGES
            .get(language)
            .ok_or_else(|| format!("Unsupported language: {language}"))?;

        // Generate unique container name
        let container_name = format!("sandbox_{}_{}", language, Uuid::new_v4());

        // Build Docker command
        let mut docker_cmd = Command::new("docker");
        docker_cmd
            .arg("run")
            .arg("--rm")
            .arg("--name")
            .arg(&container_name)
            .arg("--network")
            .arg("none") // Disable network
            .arg("--cpus")
            .arg(&self.config.security.cpu_limit) // Limit CPU
            .arg("--memory")
            .arg(&self.config.security.memory_limit) // Limit memory
            .arg("--pids-limit")
            .arg(self.config.security.pids_limit.to_string()) // Limit number of processes
            .arg("--ulimit")
            .arg(format!(
                "nofile={}",
                &self.config.security.file_descriptor_limit
            )) // Limit file descriptors
            .arg("--security-opt")
            .arg("no-new-privileges:true") // Security hardening
            .arg("-i") // Interactive mode for stdin
            .arg(config.docker_image());

        // Add command
        let command = config.command();

        if config.is_compiled() {
            docker_cmd.args(["bash", "-c", command]);
        } else {
            docker_cmd.arg(command);
        }

        // Configure stdio
        docker_cmd
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        tracing::info!("Executing Docker command for language: {language}");

        // Start the process
        let mut child = docker_cmd
            .spawn()
            .map_err(|e| format!("Failed to spawn Docker process: {e}"))?;

        // Write code to stdin
        if let Some(mut stdin) = child.stdin.take() {
            if let Err(e) = stdin.write_all(code.as_bytes()).await {
                tracing::error!("Failed to write to stdin: {e}");
                // Try to kill the container
                let _ = Self::kill_container(&container_name).await;
                return Err(format!("Failed to write code to container: {e}"));
            }
            // Close stdin to signal EOF
            drop(stdin);
        }

        // Wait for execution with timeout
        let result = tokio::time::timeout(
            Duration::from_secs(self.config.security.timeout_duration),
            child.wait_with_output(),
        )
        .await;

        match result {
            Ok(Ok(output)) => Ok(ExecutionResult {
                stdout: String::from_utf8_lossy(&output.stdout).into(),
                stderr: String::from_utf8_lossy(&output.stderr).into(),
                exit_code: output.status.code(),
                timed_out: false,
            }),
            Ok(Err(e)) => {
                let _ = Self::kill_container(&container_name).await;
                Err(format!("Process execution failed: {e}"))
            }
            Err(_) => {
                // Timeout occurred
                let _ = Self::kill_container(&container_name).await;
                Ok(ExecutionResult {
                    stdout: String::new(),
                    stderr: "Execution timed out.".into(),
                    exit_code: Some(124), // Standard timeout exit code
                    timed_out: true,
                })
            }
        }
    }

    async fn kill_container(container_name: &str) -> Result<(), String> {
        tracing::warn!("Attempting to kill container: {}", container_name);

        let kill_result = tokio::time::timeout(
            Duration::from_secs(5),
            Command::new("docker")
                .args(["kill", container_name])
                .output(),
        )
        .await;

        match kill_result {
            Ok(Ok(output)) => {
                if output.status.success() {
                    tracing::info!("Successfully killed container: {container_name}");
                    Ok(())
                } else {
                    let error = String::from_utf8_lossy(&output.stderr);
                    tracing::error!("Failed to kill container {container_name}: {error}");
                    Err(format!("Failed to kill container: {error}"))
                }
            }
            Ok(Err(e)) => {
                tracing::error!("Error executing docker kill: {e}");
                Err(format!("Error executing docker kill: {e}"))
            }
            Err(_) => {
                tracing::error!("Timeout while killing container: {container_name}");
                Err("Timeout while killing container".into())
            }
        }
    }
}

impl Default for DockerExecutor {
    fn default() -> Self {
        Self::new()
    }
}
