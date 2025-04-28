use anyhow::{Result, anyhow};
use clap::Args;
use std::path::Path;
use std::time::{Duration, Instant};
use crate::util::*;
use sage_plugin_api::PluginManager;

#[derive(Args)]
pub struct Benchmark {
    /// Path to the plugin file (.wasm or .js)
    pub path: String,

    /// Function to benchmark (pre-push, post-commit, or run)
    #[arg(short, long)]
    pub function: String,

    /// Number of iterations to run
    #[arg(short, long, default_value = "100")]
    pub iterations: usize,

    /// Branch name for pre-push events
    #[arg(short, long, default_value = "main")]
    pub branch: String,

    /// Commit ID for post-commit events
    #[arg(short, long, default_value = "abcdef1234567890")]
    pub commit_id: String,

    /// Arguments for run function
    #[arg(short, long)]
    pub args: Vec<String>,
}

impl Benchmark {
    pub fn run(&self) -> Result<()> {
        let plugin_path = Path::new(&self.path);

        // Load the plugin
        println!("Loading plugin from {}", plugin_path.display());
        let (mut plugin_manager, plugin_name) = load_plugin(plugin_path)?;

        // Check if the plugin has the requested function
        let functions = plugin_manager.get_plugin_functions(&plugin_name)
            .ok_or_else(|| anyhow!("Plugin not found: {}", plugin_name))?;

        if !functions.contains(&self.function) {
            return Err(anyhow!("Plugin does not support {} function", self.function));
        }

        println!("Benchmarking function: {}", self.function);
        println!("Plugin: {}", plugin_name);
        println!("Iterations: {}", self.iterations);

        match self.function.as_str() {
            "pre-push" | "pre_push" => self.benchmark_pre_push(&mut plugin_manager, &plugin_name)?,
            "post-commit" | "post_commit" => self.benchmark_post_commit(&mut plugin_manager, &plugin_name)?,
            "run" => self.benchmark_run(&mut plugin_manager, &plugin_name)?,
            _ => return Err(anyhow!("Unsupported function: {}", self.function)),
        }

        Ok(())
    }

    fn benchmark_pre_push(&self, plugin_manager: &mut PluginManager, plugin_name: &str) -> Result<()> {
        println!("\nBenchmarking pre-push with branch: {}", self.branch);

        let mut durations = Vec::with_capacity(self.iterations);
        let mut successes = 0;
        let mut failures = 0;

        for i in 0..self.iterations {
            print!("\rRunning iteration {}/{}...", i + 1, self.iterations);

            let start = Instant::now();
            match plugin_manager.cli(plugin_name, &[self.branch.clone()]) {
                Ok(_) => successes += 1,
                Err(_) => failures += 1,
            }
            let duration = start.elapsed();
            durations.push(duration);
        }

        println!("\n");
        self.print_benchmark_results(durations, successes, failures);

        Ok(())
    }

    fn benchmark_post_commit(&self, plugin_manager: &mut PluginManager, plugin_name: &str) -> Result<()> {
        println!("\nBenchmarking post-commit with commit ID: {}", self.commit_id);

        let mut durations = Vec::with_capacity(self.iterations);
        let mut successes = 0;
        let mut failures = 0;

        for i in 0..self.iterations {
            print!("\rRunning iteration {}/{}...", i + 1, self.iterations);

            let start = Instant::now();
            match plugin_manager.cli(plugin_name, &[self.commit_id.clone()]) {
                Ok(_) => successes += 1,
                Err(_) => failures += 1,
            }
            let duration = start.elapsed();
            durations.push(duration);
        }

        println!("\n");
        self.print_benchmark_results(durations, successes, failures);

        Ok(())
    }

    fn benchmark_run(&self, plugin_manager: &mut PluginManager, plugin_name: &str) -> Result<()> {
        println!("\nBenchmarking run with args: {:?}", self.args);

        let mut durations = Vec::with_capacity(self.iterations);
        let mut successes = 0;
        let mut failures = 0;

        for i in 0..self.iterations {
            print!("\rRunning iteration {}/{}...", i + 1, self.iterations);

            let start = Instant::now();
            match plugin_manager.cli(plugin_name, &self.args) {
                Ok(_) => successes += 1,
                Err(_) => failures += 1,
            }
            let duration = start.elapsed();
            durations.push(duration);
        }

        println!("\n");
        self.print_benchmark_results(durations, successes, failures);

        Ok(())
    }

    fn print_benchmark_results(&self, durations: Vec<Duration>, successes: usize, failures: usize) {
        // Calculate statistics
        let total_duration: Duration = durations.iter().sum();
        let avg_duration = total_duration / durations.len() as u32;

        // Sort durations for percentiles
        let mut sorted_durations = durations.clone();
        sorted_durations.sort();

        let min = sorted_durations.first().unwrap_or(&Duration::ZERO);
        let max = sorted_durations.last().unwrap_or(&Duration::ZERO);

        let p50_idx = (self.iterations * 50) / 100;
        let p90_idx = (self.iterations * 90) / 100;
        let p95_idx = (self.iterations * 95) / 100;
        let p99_idx = (self.iterations * 99) / 100;

        let p50 = sorted_durations.get(p50_idx).unwrap_or(&Duration::ZERO);
        let p90 = sorted_durations.get(p90_idx).unwrap_or(&Duration::ZERO);
        let p95 = sorted_durations.get(p95_idx).unwrap_or(&Duration::ZERO);
        let p99 = sorted_durations.get(p99_idx).unwrap_or(&Duration::ZERO);

        // Print results
        println!("Benchmark Results:");
        println!("  Total iterations: {}", self.iterations);
        println!("  Successful calls: {} ({}%)", successes, (successes * 100) / self.iterations);
        println!("  Failed calls:     {} ({}%)", failures, (failures * 100) / self.iterations);
        println!("  Total time:       {:?}", total_duration);
        println!("  Average time:     {:?}", avg_duration);
        println!("  Min time:         {:?}", min);
        println!("  Max time:         {:?}", max);
        println!("  P50 (median):     {:?}", p50);
        println!("  P90:              {:?}", p90);
        println!("  P95:              {:?}", p95);
        println!("  P99:              {:?}", p99);
        println!("  Calls per second: {:.2}", self.iterations as f64 / total_duration.as_secs_f64());
    }
}
