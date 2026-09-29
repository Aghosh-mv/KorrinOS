// Launch the PACKAGED app and smoke-test it.
//
// The executable path used to be hardcoded:
//   release/mac-arm64/Nibra BetterLife.app/Contents/MacOS/Nibra BetterLife
// so this test could only ever run on an Apple-silicon Mac, even though
// electron-builder is configured for mac, windows and linux. Resolve the
// binary for the current platform instead, and exit with a clear SKIP (not a
// crash) when the app has not been packaged yet.

import { existsSync, readdirSync } from 'node:fs';
import { mkdtemp, rm } from 'node:fs/promises';
import os from 'node:os';
import path from 'node:path';
import process from 'node:process';

const PRODUCT = 'Nibra BetterLife';
const RELEASE = path.resolve('release');

/** Return the packaged executable for the current platform, or null. */
export function findPackagedExecutable(releaseDir = RELEASE, platform = process.platform) {
  if (!existsSync(releaseDir)) return null;

  if (platform === 'darwin') {
    // release/mac*/<Product>.app/Contents/MacOS/<Product>
    for (const entry of readdirSync(releaseDir)) {
      if (!entry.startsWith('mac')) continue;
      const exe = path.join(releaseDir, entry, `${PRODUCT}.app`, 'Contents', 'MacOS', PRODUCT);
      if (existsSync(exe)) return exe;
    }
    return null;
  }

  if (platform === 'win32') {
    for (const entry of readdirSync(releaseDir)) {
      if (!entry.toLowerCase().startsWith('win')) continue;
      const exe = path.join(releaseDir, entry, `${PRODUCT}.exe`);
      if (existsSync(exe)) return exe;
    }
    return null;
  }

  // linux: an AppImage, unpacked, or a directory with a runnable binary
  for (const entry of readdirSync(releaseDir)) {
    if (!entry.toLowerCase().startsWith('linux')) continue;
    const base = path.join(releaseDir, entry);
    if (entry.toLowerCase().endsWith('.appimage') && existsSync(base)) return base;
    for (const f of readdirSync(base)) {
      if (f === PRODUCT || f === `${PRODUCT}.bin`) {
        const exe = path.join(base, f);
        if (existsSync(exe)) return exe;
      }
    }
  }
  return null;
}

async function main() {
  const exe = findPackagedExecutable();
  if (!exe) {
    console.log(
      `# SKIP package-smoke: no packaged build found under ${RELEASE}. ` +
      `Run one of: npm run package:linux | package:win | package:mac`,
    );
    process.exit(0);
  }

  const { _electron: electron, expect } = await import('@playwright/test');
  const dir = await mkdtemp(path.join(os.tmpdir(), 'nibra-package-'));
  const app = await electron.launch({ executablePath: exe, args: [`--user-data-dir=${dir}`] });
  try {
    const p = await app.firstWindow();
    await expect(p.getByRole('heading', { name: 'Meet your digital chief of staff.' })).toBeVisible();
    if (await p.getByText('Start with my workspace', { exact: true }).isVisible()) {
      await p.getByText('Start with my workspace', { exact: true }).click({ force: true });
    }
    await expect(p.getByRole('heading', { name: 'Make space for what matters most.' })).toBeVisible();
    const next = app.waitForEvent('window');
    await p.evaluate(() => window.nibra.sidecar());
    const side = await next;
    await side.waitForLoadState();
    await expect(side.getByRole('textbox', { name: 'Ask Nibra', exact: true })).toBeVisible();
    const top = await app.evaluate(({ BrowserWindow }) =>
      BrowserWindow.getAllWindows().some((w) => w.isAlwaysOnTop()),
    );
    expect(top).toBe(true);
    await side.close();
    console.log(`PASS: packaged ${process.platform} app launch, welcome, empty workspace, always-on-top sidecar.`);
  } finally {
    await app.close();
    await rm(dir, { recursive: true, force: true });
  }
}

if (import.meta.url === `file://${process.argv[1]}`) {
  await main();
}
