// Mock data for git stats (in a real plugin, this would come from git commands)
const mockStats = {
  totalCommits: 152,
  totalBranches: 8,
  activeBranches: 3,
  contributors: [
    { name: 'Alice Smith', commits: 67, email: 'alice@example.com' },
    { name: 'Bob Johnson', commits: 45, email: 'bob@example.com' },
    { name: 'Charlie Brown', commits: 40, email: 'charlie@example.com' }
  ],
  lastCommit: {
    hash: 'a1b2c3d',
    author: 'Alice Smith',
    date: '2023-05-15',
    message: 'feat: add new feature'
  },
  fileStats: {
    totalFiles: 243,
    byLanguage: [
      { language: 'Rust', files: 156, lines: 12500 },
      { language: 'TypeScript', files: 45, lines: 3200 },
      { language: 'JavaScript', files: 22, lines: 1800 },
      { language: 'Other', files: 20, lines: 950 }
    ]
  }
};

// Generate a summary of repository statistics
function generateSummary() {
  return `
📊 Git Repository Summary
========================
Total commits: ${mockStats.totalCommits}
Total branches: ${mockStats.totalBranches} (${mockStats.activeBranches} active)
Contributors: ${mockStats.contributors.length}

Last commit:
  ${mockStats.lastCommit.hash} - ${mockStats.lastCommit.message}
  by ${mockStats.lastCommit.author} on ${mockStats.lastCommit.date}
`;
}

// Generate contributor statistics
function generateContributorStats() {
  const contributors = mockStats.contributors
    .sort((a, b) => b.commits - a.commits)
    .map((c, i) => `  ${i+1}. ${c.name} (${c.commits} commits)`)
    .join('\n');
  
  return `
👥 Contributor Statistics
========================
Total contributors: ${mockStats.contributors.length}
Total commits: ${mockStats.totalCommits}

Top contributors:
${contributors}
`;
}

// Generate file statistics
function generateFileStats() {
  const languages = mockStats.fileStats.byLanguage
    .sort((a, b) => b.files - a.files)
    .map(l => `  ${l.language}: ${l.files} files (${l.lines} lines)`)
    .join('\n');
  
  return `
📁 File Statistics
========================
Total files: ${mockStats.fileStats.totalFiles}

Files by language:
${languages}
`;
}

// Generate all statistics
function generateAllStats() {
  return `${generateSummary()}\n${generateContributorStats()}\n${generateFileStats()}`;
}

// CLI command to show git stats
function run(input) {
  const cliArgs = JSON.parse(input);
  
  // Parse command line arguments
  if (cliArgs.args.length > 0 && cliArgs.args[0] === 'help') {
    const help = `Git Stats Plugin\n\nUsage: sage plugin git-stats [option]\n\n` +
                 `Options:\n` +
                 `  help        - Show this help message\n` +
                 `  summary     - Show a summary of repository statistics\n` +
                 `  contributors - Show contributor statistics\n` +
                 `  files       - Show file statistics\n` +
                 `  (no option) - Show all statistics\n`;
    
    const reply = {
      kind: 'ok',
      message: help
    };
    return JSON.stringify(reply);
  }
  
  let output = '';
  const option = cliArgs.args.length > 0 ? cliArgs.args[0] : 'all';
  
  switch (option) {
    case 'summary':
      output = generateSummary();
      break;
    case 'contributors':
      output = generateContributorStats();
      break;
    case 'files':
      output = generateFileStats();
      break;
    case 'all':
    default:
      output = generateAllStats();
      break;
  }
  
  const reply = {
    kind: 'ok',
    message: output
  };
  return JSON.stringify(reply);
}

// Export the functions
module.exports = { run };
