#!/usr/bin/env node
// Guard for tests that need `node:sqlite`, which only exists in Node >= 22.5.
//
// Without this, running the suite on Node 20 dies with
//   ERR_UNKNOWN_BUILTIN_MODULE: No such built-in module: node:sqlite
// which reads like a broken project rather than "wrong Node version".
//
// Usage:
//   node -e "import('./tests/require-node-sqlite.mjs').then(m => m.run('label', fn))"
// or simply:
//   node tests/require-node-sqlite.mjs <label> <file-to-run>

const MIN_MAJOR = 22;
const MIN_MINOR = 5;

export function hasNodeSqlite() {
  const [major, minor] = process.versions.node.split(".").map(Number);
  if (major > MIN_MAJOR) return true;
  if (major === MIN_MAJOR && minor >= MIN_MINOR) return true;
  return false;
}

export function skipReason() {
  return (
    `requires Node >= ${MIN_MAJOR}.${MIN_MINOR} for node:sqlite ` +
    `(running ${process.versions.node})`
  );
}

// Run `fn` only when node:sqlite is present; otherwise print a clear SKIP and
// exit 0 so a `&&` test chain is not aborted by an environment mismatch.
export function run(label, fn) {
  if (!hasNodeSqlite()) {
    console.log(`# SKIP ${label}: ${skipReason()}`);
    process.exit(0);
  }
  return fn();
}

if (import.meta.url === `file://${process.argv[1]}`) {
  const [, , , label, file] = process.argv;
  run(label || "suite", async () => {
    await import(file);
  });
}
