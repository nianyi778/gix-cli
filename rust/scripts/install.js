#!/usr/bin/env node

const { platform, arch } = process;
const { execSync } = require('child_process');
const path = require('path');
const fs = require('fs');

const BINARY_NAME = 'gix';
const BIN_DIR = path.join(__dirname, '..', 'bin');

// Download pre-built binary from GitHub Releases
async function install() {
  const platformMap = {
    darwin: 'apple-darwin',
    linux: 'unknown-linux-gnu',
    win32: 'pc-windows-msvc'
  };

  const archMap = {
    x64: 'x86_64',
    arm64: 'aarch64'
  };

  const target = `${archMap[arch]}-${platformMap[platform]}`;
  const ext = platform === 'win32' ? '.exe' : '';
  const binaryPath = path.join(BIN_DIR, BINARY_NAME + ext);

  // Create bin directory
  if (!fs.existsSync(BIN_DIR)) {
    fs.mkdirSync(BIN_DIR, { recursive: true });
  }

  // Try to download from GitHub releases
  const version = require('../package.json').version;
  const url = `https://github.com/nianyi778/gix-cli/releases/download/v${version}/gix-${target}${ext}`;

  try {
    console.log(`Downloading gix binary for ${target}...`);
    execSync(`curl -L ${url} -o ${binaryPath}`, { stdio: 'inherit' });
    fs.chmodSync(binaryPath, 0o755);
    console.log('✅ Installation successful!');
  } catch (error) {
    console.error('❌ Failed to download binary. You may need to build from source.');
    console.error('Run: cd rust && cargo build --release');
    process.exit(1);
  }
}

install();
