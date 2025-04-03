use std::fmt::Display;
use git2::{Repository, StatusOptions, StatusShow, BranchType};
use anyhow::{anyhow, Result, Context};

/// Represents the current state of the git repository
#[derive(Default, Debug, Clone)]
pub struct GitStatus {
    // Repository information
    pub current_branch: String,
    pub upstream_branch: Option<String>,
    pub ahead_count: usize,
    pub behind_count: usize,
    pub has_stash: bool,
    
    // Staged changes
    pub staged_added: Vec<String>,
    pub staged_modified: Vec<String>,
    pub staged_deleted: Vec<String>,
    pub staged_renamed: Vec<(String, String)>, // (from, to)
    pub staged_copied: Vec<(String, String)>,  // (from, to)
    
    // Working tree changes
    pub unstaged_modified: Vec<String>,
    pub unstaged_deleted: Vec<String>,
    pub unstaged_added: Vec<String>,
    
    // Special cases
    pub untracked: Vec<String>,
    pub ignored: Vec<String>,
    
    // Combined statuses
    pub staged_modified_unstaged_modified: Vec<String>,
    pub staged_added_unstaged_modified: Vec<String>,
    pub staged_added_unstaged_deleted: Vec<String>,
    pub staged_deleted_unstaged_modified: Vec<String>,
    pub staged_renamed_unstaged_modified: Vec<String>,
    pub staged_copied_unstaged_modified: Vec<String>,
}

/// Display options for formatting git status output
#[derive(Debug, Clone)]
pub struct DisplayOptions {
    pub show_branch_info: bool,
    pub show_staged: bool,
    pub show_unstaged: bool,
    pub show_untracked: bool,
    pub show_ignored: bool,
    pub use_symbols: bool,
    pub group_by_status: bool,
    pub max_path_length: Option<usize>,
}

impl Default for DisplayOptions {
    fn default() -> Self {
        Self {
            show_branch_info: true,
            show_staged: true,
            show_unstaged: true,
            show_untracked: true,
            show_ignored: false,
            use_symbols: true,
            group_by_status: true,
            max_path_length: None,
        }
    }
}

/// Git file status with symbols for display
pub struct StatusSymbols {
    pub added: &'static str,
    pub modified: &'static str,
    pub deleted: &'static str,
    pub renamed: &'static str,
    pub copied: &'static str,
    pub untracked: &'static str,
    pub ignored: &'static str,
}

impl Default for StatusSymbols {
    fn default() -> Self {
        Self {
            added: "A",
            modified: "M",
            deleted: "D",
            renamed: "R",
            copied: "C",
            untracked: "?",
            ignored: "!",
        }
    }
}

impl Display for GitStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Use default display options
        self.fmt_with_options(f, &DisplayOptions::default(), &StatusSymbols::default())
    }
}

impl GitStatus {
    /// Format status with custom options
    pub fn fmt_with_options(
        &self, 
        f: &mut std::fmt::Formatter<'_>, 
        options: &DisplayOptions,
        symbols: &StatusSymbols,
    ) -> std::fmt::Result {
        let mut lines = Vec::with_capacity(50); // Pre-allocate reasonable capacity

        // Branch information
        if options.show_branch_info {
            lines.push(format!("On branch {}", self.current_branch));
            
            if let Some(upstream) = &self.upstream_branch {
                let relation = if self.ahead_count > 0 && self.behind_count > 0 {
                    format!("ahead {}, behind {}", self.ahead_count, self.behind_count)
                } else if self.ahead_count > 0 {
                    format!("ahead {}", self.ahead_count)
                } else if self.behind_count > 0 {
                    format!("behind {}", self.behind_count)
                } else {
                    "up to date".to_string()
                };
                
                lines.push(format!("Your branch is {} with '{}'", relation, upstream));
            } else if !self.current_branch.is_empty() {
                lines.push("Your branch is not tracking a remote branch".to_string());
            }
            
            if self.has_stash {
                lines.push("You have stashed changes".to_string());
            }
            
            lines.push(String::new()); // Empty line after branch info
        }

        let has_staged = !self.staged_added.is_empty() 
            || !self.staged_modified.is_empty()
            || !self.staged_deleted.is_empty()
            || !self.staged_renamed.is_empty()
            || !self.staged_copied.is_empty()
            || !self.staged_modified_unstaged_modified.is_empty()
            || !self.staged_added_unstaged_modified.is_empty()
            || !self.staged_added_unstaged_deleted.is_empty()
            || !self.staged_deleted_unstaged_modified.is_empty()
            || !self.staged_renamed_unstaged_modified.is_empty()
            || !self.staged_copied_unstaged_modified.is_empty();
            
        let has_unstaged = !self.unstaged_modified.is_empty() 
            || !self.unstaged_deleted.is_empty() 
            || !self.unstaged_added.is_empty()
            || !self.staged_modified_unstaged_modified.is_empty()
            || !self.staged_added_unstaged_modified.is_empty()
            || !self.staged_added_unstaged_deleted.is_empty()
            || !self.staged_deleted_unstaged_modified.is_empty()
            || !self.staged_renamed_unstaged_modified.is_empty()
            || !self.staged_copied_unstaged_modified.is_empty();
            
        // Show summary if nothing to display
        if !has_staged && !has_unstaged && self.untracked.is_empty() && self.ignored.is_empty() {
            lines.push("Nothing to commit, working tree clean".to_string());
        }
        
        // Staged changes
        if options.show_staged && has_staged {
            lines.push("Changes to be committed:".to_string());
            
            if options.group_by_status {
                // Add staged added files
                for item in &self.staged_added {
                    let path = self.maybe_truncate_path(item, options.max_path_length);
                    lines.push(format!("  {:<2} {}", symbols.added, path));
                }
                
                // Add staged modified files
                for item in &self.staged_modified {
                    let path = self.maybe_truncate_path(item, options.max_path_length);
                    lines.push(format!("  {:<2} {}", symbols.modified, path));
                }
                
                // Add staged deleted files
                for item in &self.staged_deleted {
                    let path = self.maybe_truncate_path(item, options.max_path_length);
                    lines.push(format!("  {:<2} {}", symbols.deleted, path));
                }
                
                // Add renamed files
                for (from, to) in &self.staged_renamed {
                    let from_path = self.maybe_truncate_path(from, options.max_path_length);
                    let to_path = self.maybe_truncate_path(to, options.max_path_length);
                    lines.push(format!("  {:<2} {} -> {}", symbols.renamed, from_path, to_path));
                }
                
                // Add copied files
                for (from, to) in &self.staged_copied {
                    let from_path = self.maybe_truncate_path(from, options.max_path_length);
                    let to_path = self.maybe_truncate_path(to, options.max_path_length);
                    lines.push(format!("  {:<2} {} -> {}", symbols.copied, from_path, to_path));
                }
            }
            
            // Combined states
            // Add staged and unstaged modified files
            for item in &self.staged_modified_unstaged_modified {
                let path = self.maybe_truncate_path(item, options.max_path_length);
                lines.push(format!("  {}{}  {}", symbols.modified, symbols.modified, path));
            }
            
            // Add staged added and unstaged modified files
            for item in &self.staged_added_unstaged_modified {
                let path = self.maybe_truncate_path(item, options.max_path_length);
                lines.push(format!("  {}{}  {}", symbols.added, symbols.modified, path));
            }
            
            // Add staged added and unstaged deleted files
            for item in &self.staged_added_unstaged_deleted {
                let path = self.maybe_truncate_path(item, options.max_path_length);
                lines.push(format!("  {}{}  {}", symbols.added, symbols.deleted, path));
            }
            
            // Add staged deleted and unstaged modified files
            for item in &self.staged_deleted_unstaged_modified {
                let path = self.maybe_truncate_path(item, options.max_path_length);
                lines.push(format!("  {}{}  {}", symbols.deleted, symbols.modified, path));
            }
            
            // Add staged renamed and unstaged modified files
            for item in &self.staged_renamed_unstaged_modified {
                let path = self.maybe_truncate_path(item, options.max_path_length);
                lines.push(format!("  {}{}  {}", symbols.renamed, symbols.modified, path));
            }
            
            // Add staged copied and unstaged modified files
            for item in &self.staged_copied_unstaged_modified {
                let path = self.maybe_truncate_path(item, options.max_path_length);
                lines.push(format!("  {}{}  {}", symbols.copied, symbols.modified, path));
            }
            
            lines.push(String::new()); // Empty line after section
        }
        
        // Unstaged changes
        if options.show_unstaged && has_unstaged {
            lines.push("Changes not staged for commit:".to_string());
            
            // Add unstaged modified files
            for item in &self.unstaged_modified {
                let path = self.maybe_truncate_path(item, options.max_path_length);
                lines.push(format!("  {:<2} {}", symbols.modified, path));
            }
            
            // Add unstaged deleted files
            for item in &self.unstaged_deleted {
                let path = self.maybe_truncate_path(item, options.max_path_length);
                lines.push(format!("  {:<2} {}", symbols.deleted, path));
            }
            
            // Add unstaged added files
            for item in &self.unstaged_added {
                let path = self.maybe_truncate_path(item, options.max_path_length);
                lines.push(format!("  {:<2} {}", symbols.added, path));
            }
            
            lines.push(String::new()); // Empty line after section
        }
        
        // Untracked files
        if options.show_untracked && !self.untracked.is_empty() {
            lines.push("Untracked files:".to_string());
            for item in &self.untracked {
                let path = self.maybe_truncate_path(item, options.max_path_length);
                lines.push(format!("  {:<2} {}", symbols.untracked, path));
            }
            lines.push(String::new()); // Empty line after section
        }
        
        // Ignored files
        if options.show_ignored && !self.ignored.is_empty() {
            lines.push("Ignored files:".to_string());
            for item in &self.ignored {
                let path = self.maybe_truncate_path(item, options.max_path_length);
                lines.push(format!("  {:<2} {}", symbols.ignored, path));
            }
        }

        write!(f, "{}", lines.join("\n"))
    }

    /// Create a simple summary of the status
    #[inline]
    pub fn summary(&self) -> String {
        let mut parts = Vec::new();
        
        let staged_count = self.staged_files_count();
        let unstaged_count = self.unstaged_files_count();
        let untracked_count = self.untracked.len();
        
        if staged_count > 0 {
            parts.push(format!("{} staged", staged_count));
        }
        
        if unstaged_count > 0 {
            parts.push(format!("{} not staged", unstaged_count));
        }
        
        if untracked_count > 0 {
            parts.push(format!("{} untracked", untracked_count));
        }
        
        if parts.is_empty() {
            "clean".to_string()
        } else {
            parts.join(", ")
        }
    }

    /// Returns a compact status string (e.g., for prompts)
    pub fn compact_status(&self) -> String {
        let mut status = String::with_capacity(50); // Pre-allocate reasonable capacity
        
        if !self.current_branch.is_empty() {
            status.push_str(&self.current_branch);
        } else {
            status.push_str("detached");
        }
        
        let staged = self.staged_files_count();
        let unstaged = self.unstaged_files_count();
        let untracked = self.untracked.len();
        
        if staged > 0 || unstaged > 0 || untracked > 0 {
            status.push_str(" [");
            
            if staged > 0 {
                status.push_str(&format!("+{}", staged));
            }
            
            if unstaged > 0 {
                status.push_str(&format!("!{}", unstaged));
            }
            
            if untracked > 0 {
                status.push_str(&format!("?{}", untracked));
            }
            
            status.push(']');
        }
        
        if self.ahead_count > 0 {
            status.push_str(&format!(" ↑{}", self.ahead_count));
        }
        
        if self.behind_count > 0 {
            status.push_str(&format!(" ↓{}", self.behind_count));
        }
        
        if self.has_stash {
            status.push_str(" $");
        }
        
        status
    }

    // Helper utility methods
    
    /// Truncate path if max_length is specified
    #[inline]
    fn maybe_truncate_path(&self, path: &str, max_length: Option<usize>) -> String {
        if let Some(max) = max_length {
            if path.len() > max {
                let mut truncated = String::with_capacity(max + 3);
                truncated.push_str("...");
                truncated.push_str(&path[path.len().saturating_sub(max - 3)..]);
                return truncated;
            }
        }
        path.to_string()
    }

    /// Checks if there are any changes (staged or unstaged)
    #[inline]
    pub fn has_changes(&self) -> bool {
        !self.staged_added.is_empty()
            || !self.staged_modified.is_empty()
            || !self.staged_deleted.is_empty()
            || !self.staged_renamed.is_empty()
            || !self.staged_copied.is_empty()
            || !self.unstaged_modified.is_empty()
            || !self.unstaged_deleted.is_empty()
            || !self.unstaged_added.is_empty()
            || !self.untracked.is_empty()
            || !self.staged_modified_unstaged_modified.is_empty()
            || !self.staged_added_unstaged_modified.is_empty()
            || !self.staged_added_unstaged_deleted.is_empty()
            || !self.staged_deleted_unstaged_modified.is_empty()
            || !self.staged_renamed_unstaged_modified.is_empty()
            || !self.staged_copied_unstaged_modified.is_empty()
    }

    /// Checks if there are any staged changes
    #[inline]
    pub fn has_staged_changes(&self) -> bool {
        !self.staged_added.is_empty()
            || !self.staged_modified.is_empty()
            || !self.staged_deleted.is_empty()
            || !self.staged_renamed.is_empty()
            || !self.staged_copied.is_empty()
            || !self.staged_modified_unstaged_modified.is_empty()
            || !self.staged_added_unstaged_modified.is_empty()
            || !self.staged_added_unstaged_deleted.is_empty()
            || !self.staged_deleted_unstaged_modified.is_empty()
            || !self.staged_renamed_unstaged_modified.is_empty()
            || !self.staged_copied_unstaged_modified.is_empty()
    }

    /// Checks if there are any unstaged changes
    #[inline]
    pub fn has_unstaged_changes(&self) -> bool {
        !self.unstaged_modified.is_empty()
            || !self.unstaged_deleted.is_empty()
            || !self.unstaged_added.is_empty()
            || !self.staged_modified_unstaged_modified.is_empty()
            || !self.staged_added_unstaged_modified.is_empty()
            || !self.staged_added_unstaged_deleted.is_empty()
            || !self.staged_deleted_unstaged_modified.is_empty()
            || !self.staged_renamed_unstaged_modified.is_empty()
            || !self.staged_copied_unstaged_modified.is_empty()
    }

    /// Checks if there are any untracked files
    #[inline]
    pub fn has_untracked(&self) -> bool {
        !self.untracked.is_empty()
    }
    
    /// Count total number of staged files
    #[inline]
    pub fn staged_files_count(&self) -> usize {
        self.staged_added.len()
            + self.staged_modified.len()
            + self.staged_deleted.len()
            + self.staged_renamed.len()
            + self.staged_copied.len()
    }
    
    /// Count total number of unstaged files
    #[inline]
    pub fn unstaged_files_count(&self) -> usize {
        self.unstaged_modified.len()
            + self.unstaged_deleted.len()
            + self.unstaged_added.len()
    }
    
    /// Count total number of combined status files
    #[inline]
    pub fn combined_status_files_count(&self) -> usize {
        self.staged_modified_unstaged_modified.len()
            + self.staged_added_unstaged_modified.len()
            + self.staged_added_unstaged_deleted.len()
            + self.staged_deleted_unstaged_modified.len()
            + self.staged_renamed_unstaged_modified.len()
            + self.staged_copied_unstaged_modified.len()
    }
    
    /// Get all modified files (both staged and unstaged)
    pub fn all_modified_files(&self) -> Vec<String> {
        let total_size = self.staged_modified.len() + 
                         self.unstaged_modified.len() + 
                         self.staged_modified_unstaged_modified.len() +
                         self.staged_added_unstaged_modified.len() +
                         self.staged_deleted_unstaged_modified.len() +
                         self.staged_renamed_unstaged_modified.len() +
                         self.staged_copied_unstaged_modified.len();
                         
        let mut files = Vec::with_capacity(total_size);
        files.extend_from_slice(&self.staged_modified);
        files.extend_from_slice(&self.unstaged_modified);
        files.extend_from_slice(&self.staged_modified_unstaged_modified);
        files.extend_from_slice(&self.staged_added_unstaged_modified);
        files.extend_from_slice(&self.staged_deleted_unstaged_modified);
        files.extend_from_slice(&self.staged_renamed_unstaged_modified);
        files.extend_from_slice(&self.staged_copied_unstaged_modified);
        files
    }
    
    /// Get all added files (both staged and unstaged)
    pub fn all_added_files(&self) -> Vec<String> {
        let total_size = self.staged_added.len() + self.unstaged_added.len();
        let mut files = Vec::with_capacity(total_size);
        files.extend_from_slice(&self.staged_added);
        files.extend_from_slice(&self.unstaged_added);
        files
    }
    
    /// Get all deleted files (both staged and unstaged)
    pub fn all_deleted_files(&self) -> Vec<String> {
        let total_size = self.staged_deleted.len() + 
                         self.unstaged_deleted.len() + 
                         self.staged_added_unstaged_deleted.len();
        let mut files = Vec::with_capacity(total_size);
        files.extend_from_slice(&self.staged_deleted);
        files.extend_from_slice(&self.unstaged_deleted);
        files.extend_from_slice(&self.staged_added_unstaged_deleted);
        files
    }
    
    /// Get all renamed files
    pub fn all_renamed_files(&self) -> Vec<(String, String)> {
        self.staged_renamed.clone()
    }
    
    /// Get all copied files
    pub fn all_copied_files(&self) -> Vec<(String, String)> {
        self.staged_copied.clone()
    }
    
    /// Check if a specific file is staged
    pub fn is_file_staged(&self, path: &str) -> bool {
        self.staged_added.contains(&path.to_string())
            || self.staged_modified.contains(&path.to_string())
            || self.staged_deleted.contains(&path.to_string())
            || self.staged_renamed.iter().any(|(_, to)| to == path)
            || self.staged_copied.iter().any(|(_, to)| to == path)
            || self.staged_modified_unstaged_modified.contains(&path.to_string())
            || self.staged_added_unstaged_modified.contains(&path.to_string())
            || self.staged_added_unstaged_deleted.contains(&path.to_string())
            || self.staged_deleted_unstaged_modified.contains(&path.to_string())
            || self.staged_renamed_unstaged_modified.contains(&path.to_string())
            || self.staged_copied_unstaged_modified.contains(&path.to_string())
    }
    
    /// Check if a specific file is unstaged
    pub fn is_file_unstaged(&self, path: &str) -> bool {
        self.unstaged_modified.contains(&path.to_string())
            || self.unstaged_deleted.contains(&path.to_string())
            || self.unstaged_added.contains(&path.to_string())
            || self.staged_modified_unstaged_modified.contains(&path.to_string())
            || self.staged_added_unstaged_modified.contains(&path.to_string())
            || self.staged_added_unstaged_deleted.contains(&path.to_string())
            || self.staged_deleted_unstaged_modified.contains(&path.to_string())
            || self.staged_renamed_unstaged_modified.contains(&path.to_string())
            || self.staged_copied_unstaged_modified.contains(&path.to_string())
    }
    
    /// Check if a specific file is untracked
    pub fn is_file_untracked(&self, path: &str) -> bool {
        self.untracked.contains(&path.to_string())
    }
    
    /// Get the status of a specific file
    pub fn get_file_status(&self, path: &str) -> Vec<&'static str> {
        let path_str = path.to_string();
        let mut statuses = Vec::new();
        
        if self.staged_added.contains(&path_str) {
            statuses.push("staged added");
        }
        
        if self.staged_modified.contains(&path_str) {
            statuses.push("staged modified");
        }
        
        if self.staged_deleted.contains(&path_str) {
            statuses.push("staged deleted");
        }
        
        if self.staged_renamed.iter().any(|(_, to)| to == path) {
            statuses.push("staged renamed");
        }
        
        if self.staged_copied.iter().any(|(_, to)| to == path) {
            statuses.push("staged copied");
        }
        
        if self.unstaged_modified.contains(&path_str) {
            statuses.push("unstaged modified");
        }
        
        if self.unstaged_deleted.contains(&path_str) {
            statuses.push("unstaged deleted");
        }
        
        if self.unstaged_added.contains(&path_str) {
            statuses.push("unstaged added");
        }
        
        if self.staged_modified_unstaged_modified.contains(&path_str) {
            statuses.push("staged modified, unstaged modified");
        }
        
        if self.staged_added_unstaged_modified.contains(&path_str) {
            statuses.push("staged added, unstaged modified");
        }
        
        if self.staged_added_unstaged_deleted.contains(&path_str) {
            statuses.push("staged added, unstaged deleted");
        }
        
        if self.staged_deleted_unstaged_modified.contains(&path_str) {
            statuses.push("staged deleted, unstaged modified");
        }
        
        if self.staged_renamed_unstaged_modified.contains(&path_str) {
            statuses.push("staged renamed, unstaged modified");
        }
        
        if self.staged_copied_unstaged_modified.contains(&path_str) {
            statuses.push("staged copied, unstaged modified");
        }
        
        if self.untracked.contains(&path_str) {
            statuses.push("untracked");
        }
        
        if self.ignored.contains(&path_str) {
            statuses.push("ignored");
        }
        
        statuses
    }
    
    /// Filter the status to only include files in a given directory
    pub fn filter_by_directory(&self, directory: &str) -> GitStatus {
        let dir_path = if directory.ends_with('/') {
            directory.to_string()
        } else {
            format!("{}/", directory)
        };
        
        let filter_vec = |files: &[String]| -> Vec<String> {
            files
                .iter()
                .filter(|file| file.starts_with(&dir_path) || file == &directory)
                .cloned()
                .collect()
        };
        
        let filter_pair_vec = |pairs: &[(String, String)]| -> Vec<(String, String)> {
            pairs
                .iter()
                .filter(|(from, to)| 
                    from.starts_with(&dir_path) || from == &directory || 
                    to.starts_with(&dir_path) || to == &directory
                )
                .cloned()
                .collect()
        };
        
        GitStatus {
            current_branch: self.current_branch.clone(),
            upstream_branch: self.upstream_branch.clone(),
            ahead_count: self.ahead_count,
            behind_count: self.behind_count,
            has_stash: self.has_stash,
            
            staged_added: filter_vec(&self.staged_added),
            staged_modified: filter_vec(&self.staged_modified),
            staged_deleted: filter_vec(&self.staged_deleted),
            staged_renamed: filter_pair_vec(&self.staged_renamed),
            staged_copied: filter_pair_vec(&self.staged_copied),
            
            unstaged_modified: filter_vec(&self.unstaged_modified),
            unstaged_deleted: filter_vec(&self.unstaged_deleted),
            unstaged_added: filter_vec(&self.unstaged_added),
            
            untracked: filter_vec(&self.untracked),
            ignored: filter_vec(&self.ignored),
            
            staged_modified_unstaged_modified: filter_vec(&self.staged_modified_unstaged_modified),
            staged_added_unstaged_modified: filter_vec(&self.staged_added_unstaged_modified),
            staged_added_unstaged_deleted: filter_vec(&self.staged_added_unstaged_deleted),
            staged_deleted_unstaged_modified: filter_vec(&self.staged_deleted_unstaged_modified),
            staged_renamed_unstaged_modified: filter_vec(&self.staged_renamed_unstaged_modified),
            staged_copied_unstaged_modified: filter_vec(&self.staged_copied_unstaged_modified),
        }
    }

    /// Checks if the repository is clean (has no changes)
    #[inline]
    pub fn is_clean(&self) -> bool {
        !self.has_changes() && self.untracked.is_empty()
    }

    /// Checks if the repository is dirty (has changes)
    #[inline]
    pub fn is_dirty(&self) -> bool {
        !self.is_clean()
    }

    /// Checks if the local branch has diverged from its upstream branch
    /// A branch is considered diverged when it has both ahead and behind commits
    #[inline]
    pub fn is_diverged(&self) -> bool {
        self.ahead_count > 0 && self.behind_count > 0
    }

    /// Returns just the upstream status (ahead/behind) in a concise format
    pub fn upstream_status(&self) -> String {
        if self.ahead_count == 0 && self.behind_count == 0 {
            return String::new();
        }
        
        let mut status = String::with_capacity(15);
        status.push('[');
        
        if self.ahead_count > 0 {
            status.push_str(&format!("↑{}", self.ahead_count));
        }
        
        if self.behind_count > 0 {
            status.push_str(&format!("↓{}", self.behind_count));
        }
        
        status.push(']');
        status
    }

    /// Check if we need to push changes to remote
    pub fn needs_push(&self) -> bool {
        self.ahead_count > 0
    }

    /// Check if we need to pull changes from remote
    pub fn needs_pull(&self) -> bool {
        self.behind_count > 0
    }
}

/// Get the current git status using git2 library
pub fn status() -> Result<GitStatus> {
    let mut gs = GitStatus::default();
    
    // Open the repository
    let repo = Repository::open_from_env()
        .context("Failed to open git repository")?;
    
    // Get branch information
    get_branch_info(&repo, &mut gs)?;
    
    // Check for stashes
    gs.has_stash = has_stash(&repo)?;
    
    // Get the detailed status
    get_status_details(&repo, &mut gs)?;
    
    Ok(gs)
}

/// Get branch information including upstream and ahead/behind counts
fn get_branch_info(repo: &Repository, gs: &mut GitStatus) -> Result<()> {
    // Get current branch
    if let Ok(head) = repo.head() {
        if head.is_branch() {
            if let Some(name) = head.shorthand() {
                gs.current_branch = name.to_string();
                
                // Get the upstream branch
                if let Ok(local_branch) = repo.find_branch(name, BranchType::Local) {
                    // upstream() returns Result<Branch>, not Result<Option<Branch>>
                    if let Ok(upstream_branch) = local_branch.upstream() {
                        // name() returns Result<Option<&str>>
                        if let Ok(Some(upstream_name)) = upstream_branch.name() {
                            gs.upstream_branch = Some(upstream_name.to_string());
                            
                            // Get ahead/behind counts
                            let local_oid = head.target().unwrap();
                            let upstream_oid = upstream_branch.get().target().unwrap();
                            
                            if let Ok((ahead, behind)) = repo.graph_ahead_behind(local_oid, upstream_oid) {
                                gs.ahead_count = ahead;
                                gs.behind_count = behind;
                            }
                        }
                    }
                }
            }
        } else {
            // Check if HEAD is detached (no branch name)
            // git2 doesn't have is_detached() method, so we check if it's not a branch
            if !head.is_branch() {
                if let Some(oid) = head.target() {
                    gs.current_branch = format!("detached@{}", oid.to_string()[..7].to_string());
                }
            }
        }
    }
    
    Ok(())
}

/// Check if there are any stashes
fn has_stash(repo: &Repository) -> Result<bool> {
    // Alternative approach to check for stashes without using stash_foreach
    // Look for the ref directly
    match repo.find_reference("refs/stash") {
        Ok(_) => Ok(true),  // Stash reference exists
        Err(_) => Ok(false) // No stash reference
    }
}

/// Get detailed status information
fn get_status_details(repo: &Repository, gs: &mut GitStatus) -> Result<()> {
    let mut opts = StatusOptions::new();
    opts.include_untracked(true)
        .recurse_untracked_dirs(true)
        .include_ignored(true)
        .renames_head_to_index(true)  // Enable rename detection (instead of detect_rename)
        .renames_index_to_workdir(true)
        .show(StatusShow::IndexAndWorkdir);
    
    let statuses = repo.statuses(Some(&mut opts))
        .context("Failed to get git status")?;
    
    for entry in statuses.iter() {
        let path = match entry.path() {
            Some(p) => p.to_string(),
            None => continue,
        };
        
        let status = entry.status();
        
        // Handle index (staged) changes
        if status.is_index_new() {
            if !status.is_wt_deleted() && !status.is_wt_modified() {
                gs.staged_added.push(path.clone());
            }
        } else if status.is_index_modified() {
            if !status.is_wt_modified() {
                gs.staged_modified.push(path.clone());
            }
        } else if status.is_index_deleted() {
            if !status.is_wt_modified() {
                gs.staged_deleted.push(path.clone());
            }
        } else if status.is_index_renamed() {
            if let Some(diff) = entry.head_to_index() {
                if let Some(old_path) = diff.old_file().path() {
                    let old_path_str = old_path.to_string_lossy().to_string();
                    
                    if !status.is_wt_modified() {
                        gs.staged_renamed.push((old_path_str, path.clone()));
                    } else {
                        gs.staged_renamed.push((old_path_str, path.clone()));
                        gs.staged_renamed_unstaged_modified.push(path.clone());
                    }
                }
            }
        } else if status.is_index_typechange() {
            // Approximating "copied" with typechange - git2 doesn't have direct equivalent
            if let Some(diff) = entry.head_to_index() {
                if let Some(old_path) = diff.old_file().path() {
                    let old_path_str = old_path.to_string_lossy().to_string();
                    
                    if !status.is_wt_modified() {
                        gs.staged_copied.push((old_path_str, path.clone()));
                    } else {
                        gs.staged_copied.push((old_path_str, path.clone()));
                        gs.staged_copied_unstaged_modified.push(path.clone());
                    }
                }
            }
        }
        
        // Handle combined states
        if status.is_index_modified() && status.is_wt_modified() {
            gs.staged_modified_unstaged_modified.push(path.clone());
        } else if status.is_index_new() && status.is_wt_modified() {
            gs.staged_added_unstaged_modified.push(path.clone());
        } else if status.is_index_new() && status.is_wt_deleted() {
            gs.staged_added_unstaged_deleted.push(path.clone());
        } else if status.is_index_deleted() && status.is_wt_modified() {
            gs.staged_deleted_unstaged_modified.push(path.clone());
        }
        
        // Handle working tree (unstaged) status
        if status.is_wt_modified() {
            if !status.is_index_modified() && !status.is_index_new() && 
               !status.is_index_deleted() && !status.is_index_renamed() && 
               !status.is_index_typechange() {
                gs.unstaged_modified.push(path.clone());
            }
        } else if status.is_wt_deleted() {
            if !status.is_index_new() {
                gs.unstaged_deleted.push(path.clone());
            }
        }
        
        // Untracked files
        if status.is_wt_new() && !status.is_ignored() {
            gs.untracked.push(path.clone());
            
            // Also note as unstaged added
            if !gs.unstaged_added.contains(&path) {
                gs.unstaged_added.push(path.clone());
            }
        }
        
        // Ignored files
        if status.is_ignored() {
            gs.ignored.push(path.clone());
        }
    }
    
    Ok(())
}

/// Alternative implementation to get ahead/behind counts specifically
pub fn get_ahead_behind_counts() -> Result<(usize, usize)> {
    let repo = Repository::open_from_env()
        .context("Failed to open git repository")?;
    
    let head = repo.head()?;
    if !head.is_branch() {
        return Ok((0, 0)); // No branch, no ahead/behind
    }
    
    let branch_name = head.shorthand().unwrap_or("");
    let branch = repo.find_branch(branch_name, BranchType::Local)?;
    
    // upstream() returns Result<Branch>, not Result<Option<Branch>>
    let upstream = match branch.upstream() {
        Ok(upstream) => upstream,
        Err(_) => return Ok((0, 0)), // No upstream branch
    };
    
    let head_oid = head.target().ok_or_else(|| anyhow!("HEAD is not a direct reference"))?;
    let upstream_oid = upstream.get().target().ok_or_else(|| anyhow!("Upstream is not a direct reference"))?;
    
    match repo.graph_ahead_behind(head_oid, upstream_oid) {
        Ok((ahead, behind)) => Ok((ahead, behind)),
        Err(_) => Ok((0, 0)),
    }
}

/// A lightweight version of GitStatus that only contains summary information
#[derive(Debug, Default, Clone)]
pub struct LightweightStatus {
    pub branch_name: String,
    pub ahead_count: usize,
    pub behind_count: usize,
    pub has_staged_changes: bool,
    pub has_unstaged_changes: bool,
    pub untracked_count: usize,
    pub has_stashes: bool,
}

impl LightweightStatus {
    /// Checks if the repository is clean (has no changes)
    #[inline]
    pub fn is_clean(&self) -> bool {
        !self.has_staged_changes && !self.has_unstaged_changes && self.untracked_count == 0
    }

    /// Checks if the repository is dirty (has changes)
    #[inline]
    pub fn is_dirty(&self) -> bool {
        !self.is_clean()
    }

    /// Returns just the upstream status (ahead/behind) in a concise format
    pub fn upstream_status(&self) -> String {
        if self.ahead_count == 0 && self.behind_count == 0 {
            return String::new();
        }
        
        let mut status = String::with_capacity(15);
        status.push('[');
        
        if self.ahead_count > 0 {
            status.push_str(&format!("↑{}", self.ahead_count));
        }
        
        if self.behind_count > 0 {
            status.push_str(&format!("↓{}", self.behind_count));
        }
        
        status.push(']');
        status
    }
}

impl Display for LightweightStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.branch_name)?;
        
        let mut status_parts = Vec::new();
        
        if self.has_staged_changes {
            status_parts.push("+".to_string());
        }
        
        if self.has_unstaged_changes {
            status_parts.push("!".to_string());
        }
        
        if self.untracked_count > 0 {
            status_parts.push(format!("?{}", self.untracked_count));
        }
        
        if !status_parts.is_empty() {
            write!(f, " [{}]", status_parts.join(""))?;
        }
        
        if self.ahead_count > 0 {
            write!(f, " ↑{}", self.ahead_count)?;
        }
        
        if self.behind_count > 0 {
            write!(f, " ↓{}", self.behind_count)?;
        }
        
        if self.has_stashes {
            write!(f, " $")?;
        }
        
        Ok(())
    }
}

/// Get a lightweight status report that's more performant than the full status
pub fn lightweight_status() -> Result<LightweightStatus> {
    let mut status = LightweightStatus::default();
    
    let repo = Repository::open_from_env()
        .context("Failed to open git repository")?;
    
    // Get branch name
    if let Ok(head) = repo.head() {
        if let Some(name) = head.shorthand() {
            status.branch_name = name.to_string();
        } else {
            // Check if HEAD is detached (not pointing to a branch)
            if !head.is_branch() {
                if let Some(oid) = head.target() {
                    status.branch_name = format!("detached@{}", oid.to_string()[..7].to_string());
                }
            }
        }
    }
    
    // Get ahead/behind counts
    let (ahead, behind) = get_ahead_behind_counts().unwrap_or((0, 0));
    status.ahead_count = ahead;
    status.behind_count = behind;
    
    // Check for changes
    let mut opts = StatusOptions::new();
    opts.include_untracked(true)
        .show(StatusShow::IndexAndWorkdir);
    
    if let Ok(statuses) = repo.statuses(Some(&mut opts)) {
        for entry in statuses.iter() {
            let status_bits = entry.status();
            
            // Check for staged changes
            if status_bits.is_index_new() || status_bits.is_index_modified() || 
               status_bits.is_index_deleted() || status_bits.is_index_renamed() ||
               status_bits.is_index_typechange() {
                status.has_staged_changes = true;
            }
            
            // Check for unstaged changes
            if status_bits.is_wt_modified() || status_bits.is_wt_deleted() || 
               status_bits.is_wt_typechange() || status_bits.is_wt_renamed() {
                status.has_unstaged_changes = true;
            }
            
            // Count untracked files
            if status_bits.is_wt_new() && !status_bits.is_ignored() {
                status.untracked_count += 1;
            }
        }
    }
    
    // Check for stashes
    status.has_stashes = has_stash(&repo)?;
    
    Ok(status)
}

pub fn is_clean() -> Result<bool> {
    let status = status()?;
    Ok(status.is_clean())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;
    
    #[test]
    #[ignore] // Skip during normal test runs
    fn benchmark_status_methods() {
        println!("\n=== Git Status Performance Benchmark ===");
        
        // Benchmark full status
        let full_start = Instant::now();
        match status() {
            Ok(status) => {
                let full_duration = full_start.elapsed();
                println!("\n--- Full Status Method ---");
                println!("Time taken: {:?}", full_duration);
                println!("Status summary: {}", status.summary());
                println!("Branch: {}, Ahead: {}, Behind: {}", 
                         status.current_branch, 
                         status.ahead_count, 
                         status.behind_count);
                println!("Staged files: {}", status.staged_files_count());
                println!("Unstaged files: {}", status.unstaged_files_count());
                println!("Untracked files: {}", status.untracked.len());
                println!("Has stash: {}", status.has_stash);
            },
            Err(e) => println!("Full status error: {}", e),
        }
        
        // Benchmark ahead/behind counts only
        let ahead_behind_start = Instant::now();
        match get_ahead_behind_counts() {
            Ok((ahead, behind)) => {
                let ahead_behind_duration = ahead_behind_start.elapsed();
                println!("\n--- Ahead/Behind Counts Method ---");
                println!("Time taken: {:?}", ahead_behind_duration);
                println!("Ahead: {}, Behind: {}", ahead, behind);
            },
            Err(e) => println!("Ahead/behind error: {}", e),
        }
        
        // Benchmark lightweight status
        let lightweight_start = Instant::now();
        match lightweight_status() {
            Ok(status) => {
                let lightweight_duration = lightweight_start.elapsed();
                println!("\n--- Lightweight Status Method ---");
                println!("Time taken: {:?}", lightweight_duration);
                println!("Status: {}", status);
                println!("Branch: {}, Ahead: {}, Behind: {}", 
                         status.branch_name, 
                         status.ahead_count, 
                         status.behind_count);
                println!("Has staged changes: {}", status.has_staged_changes);
                println!("Has unstaged changes: {}", status.has_unstaged_changes);
                println!("Untracked files: {}", status.untracked_count);
                println!("Has stashes: {}", status.has_stashes);
            },
            Err(e) => println!("Lightweight status error: {}", e),
        }
        
        println!("\n=== Benchmark Complete ===");
    }
}