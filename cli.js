#!/usr/bin/env node

const path = require('path');
const os = require('os');
const { spawnSync } = require('child_process');

// detect OS and pick correct binary
const platform = os.platform();
const binaryName =
  platform === 'win32'
    ? 'guardstack-win.exe'
    : platform === 'darwin'
      ? 'guardstack-macos'
      : 'guardstack-linux';

const binaryPath = path.join(__dirname, 'bin', binaryName);

// pass ALL argumets through to RUST binary
const result = spawnSync(binaryPath, process.argv.slice(2), {
  stdio: 'inherit',
});

process.exit(result.status);
