import { Plugin, Json } from '@extism/js-pdk';

// Define the event structure that matches the host's Event enum
interface PrePushEvent {
  event: 'pre_push';
  branch: string;
}

interface PostCommitEvent {
  event: 'post_commit';
  oid: string;
}

type Event = PrePushEvent | PostCommitEvent;

// Define the reply structure that matches the host's Reply enum
interface OkReply {
  kind: 'ok';
  message: string;
}

interface ErrorReply {
  kind: 'error';
  message: string;
}

type Reply = OkReply | ErrorReply;

// Define the CLI args structure
interface CliArgs {
  args: string[];
}

// Branch naming patterns
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

// Pre-push hook to validate branch names
Plugin.registerFunction('pre_push', (input: string): string => {
  const event = Json.parse<Event>(input);
  
  if (event.event !== 'pre_push') {
    const reply: ErrorReply = {
      kind: 'error',
      message: 'Unexpected event type for pre_push function'
    };
    return Json.stringify(reply);
  }
  
  const { branch } = event;
  const validationResult = validateBranchName(branch);
  
  if (validationResult.valid) {
    const reply: OkReply = {
      kind: 'ok',
      message: `Branch name '${branch}' follows naming convention`
    };
    return Json.stringify(reply);
  } else {
    const reply: ErrorReply = {
      kind: 'error',
      message: validationResult.message
    };
    return Json.stringify(reply);
  }
});

// CLI command to validate a branch name
Plugin.registerFunction('run', (input: string): string => {
  const cliArgs = Json.parse<CliArgs>(input);
  
  if (cliArgs.args.length === 0) {
    const validPatterns = VALID_BRANCH_PATTERNS.map(p => p.description).join('\n  - ');
    const help = `Branch Validator Plugin\n\nUsage: sage plugin branch-validator <branch-name>\n\n` +
                 `Validates that branch names follow one of these patterns:\n  - ${validPatterns}\n`;
    
    const reply: OkReply = {
      kind: 'ok',
      message: help
    };
    return Json.stringify(reply);
  }
  
  const branchName = cliArgs.args[0];
  const validationResult = validateBranchName(branchName);
  
  if (validationResult.valid) {
    const reply: OkReply = {
      kind: 'ok',
      message: `✅ Branch name '${branchName}' follows naming convention`
    };
    return Json.stringify(reply);
  } else {
    const reply: ErrorReply = {
      kind: 'error',
      message: `❌ ${validationResult.message}`
    };
    return Json.stringify(reply);
  }
});

// Helper function to validate a branch name
function validateBranchName(branchName: string): { valid: boolean; message: string } {
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
