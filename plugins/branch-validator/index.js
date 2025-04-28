// Define the branch naming patterns
const VALID_BRANCH_PATTERNS = [
  { pattern: /^feature\/[a-z0-9-]+$/, description: 'feature/<feature-name>' },
  { pattern: /^bugfix\/[a-z0-9-]+$/, description: 'bugfix/<bug-name>' },
  { pattern: /^hotfix\/[a-z0-9-]+$/, description: 'hotfix/<fix-name>' },
  { pattern: /^release\/v\d+\.\d+(\.\d+)?$/, description: 'release/v<version>' },
  { pattern: /^docs\/[a-z0-9-]+$/, description: 'docs/<doc-name>' },
  { pattern: /^refactor\/[a-z0-9-]+$/, description: 'refactor/<name>' },
  { pattern: /^test\/[a-z0-9-]+$/, description: 'test/<test-name>' },
  { pattern: /^chore\/[a-z0-9-]+$/, description: 'chore/<chore-name>' },
  { pattern: /^main$/, description: 'main' },
  { pattern: /^develop$/, description: 'develop' }
];

// Helper function to validate a branch name
function validateBranchName(branchName) {
  for (const { pattern } of VALID_BRANCH_PATTERNS) {
    if (pattern.test(branchName)) {
      return { valid: true, message: '' };
    }
  }
  
  const validPatterns = VALID_BRANCH_PATTERNS.map(p => p.description).join(', ');
  return {
    valid: false,
    message: `Branch name '${branchName}' doesn't follow naming convention. Valid patterns: ${validPatterns}`
  };
}

// Pre-push hook to validate branch names
function pre_push(input) {
  const event = JSON.parse(input);
  
  if (event.event !== 'pre_push') {
    const reply = {
      kind: 'error',
      message: 'Unexpected event type for pre_push function'
    };
    return JSON.stringify(reply);
  }
  
  const { branch } = event;
  const validationResult = validateBranchName(branch);
  
  if (validationResult.valid) {
    const reply = {
      kind: 'ok',
      message: `Branch name '${branch}' follows naming convention`
    };
    return JSON.stringify(reply);
  } else {
    const reply = {
      kind: 'error',
      message: validationResult.message
    };
    return JSON.stringify(reply);
  }
}

// CLI command to validate a branch name
function run(input) {
  const cliArgs = JSON.parse(input);
  
  if (cliArgs.args.length === 0) {
    const validPatterns = VALID_BRANCH_PATTERNS.map(p => p.description).join('\n  - ');
    const help = `Branch Validator Plugin\n\nUsage: sage plugin branch-validator <branch-name>\n\n` +
                 `Validates that branch names follow one of these patterns:\n  - ${validPatterns}\n`;
    
    const reply = {
      kind: 'ok',
      message: help
    };
    return JSON.stringify(reply);
  }
  
  const branchName = cliArgs.args[0];
  const validationResult = validateBranchName(branchName);
  
  if (validationResult.valid) {
    const reply = {
      kind: 'ok',
      message: `✅ Branch name '${branchName}' follows naming convention`
    };
    return JSON.stringify(reply);
  } else {
    const reply = {
      kind: 'error',
      message: `❌ ${validationResult.message}`
    };
    return JSON.stringify(reply);
  }
}

// Export the functions
module.exports = { pre_push, run };
