const fs = require('fs');
const assert = require('assert/strict');
const path = require('path');
const os = require('os');
const { spawnSync } = require('child_process');
const runtimePackages = process.env.AXIOM_RUNTIME_NODE_MODULES || path.join(os.homedir(), '.cache/codex-runtimes/codex-primary-runtime/dependencies/node/node_modules');
const { chromium } = require(require.resolve('playwright', { paths: [process.cwd(), runtimePackages] }));
const project = path.resolve(__dirname, '../../..');
const testUrl = process.env.AXIOM_TEST_URL || 'http://127.0.0.1:3000/';
const root = process.env.AXIOM_EVIDENCE_DIR || path.join(project, 'docs/evidence/browser');
fs.mkdirSync(root, { recursive: true });
const fixture = spawnSync(process.platform === 'win32' ? 'cargo.exe' : 'cargo', ['run', '--example', 'replay_fixture', '--manifest-path', path.join(project, 'crates/axiom-math/Cargo.toml'), '--locked', '--offline'], { cwd: project, encoding: 'utf8' });
if (fixture.status !== 0) throw new Error(fixture.stderr || 'Native replay fixture failed.');
fs.writeFileSync(path.join(root, 'native-replay-fixtures.json'), fixture.stdout);

(async () => {
  const browser = await chromium.launch({ ...(process.env.AXIOM_BROWSER_EXECUTABLE ? { executablePath: process.env.AXIOM_BROWSER_EXECUTABLE } : process.platform === 'win32' ? { channel: 'chrome' } : {}), headless: true, args: ['--no-proxy-server'] });
  const context = await browser.newContext({ viewport: { width: 1280, height: 900 } });
  const page = await context.newPage();
  const errors = [];
  page.on('pageerror', error => errors.push(String(error)));
  const evidence = [];
  page.on('requestfailed', request => console.log('REQUEST FAILED', request.url(), request.failure()?.errorText));
  try {
    await page.goto(testUrl, { waitUntil: 'domcontentloaded' });
    await page.waitForLoadState('networkidle');
    await page.getByRole('button', { name: 'AXIOM Mathematics Academy Home', exact: true }).waitFor();
    evidence.push('Application boots with the actual Rust WASM module.');
    const native = JSON.parse(fs.readFileSync(root + '/native-replay-fixtures.json', 'utf8'));
    const browserArtifacts = await page.evaluate(async () => {
      const core = await import('/src/engine/axiomCore.ts');
      await core.initializeMathCore();
      const names = ['fractions.compare.positive','fractions.equivalent.find','fractions.unit.identify','arithmetic.whole.addition','arithmetic.column.addition','arithmetic.column.subtraction','place_value.decompose','abacus.read.state','abacus.target.setting'];
      return Promise.all(names.map(name => core.generateExercise(name, '0'.repeat(64), 1, '0'.repeat(64))));
    });
    for (let i = 0; i < native.length; i++) {
      const { template_id, seed_hex, ...wire } = browserArtifacts[i];
      assert.deepEqual(wire, native[i]);
    }
    evidence.push('All nine exercise families reproduce identical native-Rust and browser-worker artifacts for the zero seed.');
    await page.getByRole('button', { name: 'Practice' }).click();
    await page.getByLabel('Choose an exercise').selectOption('arithmetic.whole.addition');
    await page.getByRole('heading', { name: /^Calculate \d+ \+ \d+\.$/ }).waitFor();
    const prompt = await page.getByRole('heading', { name: /^Calculate \d+ \+ \d+\.$/ }).innerText();
    const operands = prompt.match(/(\d+) \+ (\d+)/);
    const target = String(Number(operands[1]) + Number(operands[2]));
    await page.getByRole('textbox', { name: 'Your answer' }).fill(target + 'junk');
    await page.getByRole('button', { name: 'Check answer', exact: true }).click();
    await page.getByRole('heading', { name: 'Check how your answer is written' }).waitFor();
    assert.equal(await page.getByRole('heading', { name: 'Answer checked locally' }).count(), 0);
    evidence.push('Malformed integer suffix is rejected in the actual browser flow.');
    await page.getByRole('textbox', { name: 'Your answer' }).fill(target);
    await page.getByRole('button', { name: 'Check answer', exact: true }).click();
    await page.getByRole('heading', { name: 'Answer checked locally' }).waitFor();
    const firstXp = await page.locator('.axiom-badge-xp strong').innerText();
    await page.getByRole('button', { name: 'Check answer', exact: true }).click();
    await page.waitForTimeout(250);
    assert.equal(await page.locator('.axiom-badge-xp strong').innerText(), firstXp);
    evidence.push('A repeated successful submission does not duplicate its reward.');
    await page.getByLabel('Choose an exercise').selectOption('fractions.unit.identify');
    const unitPrompt = page.getByRole('heading', { name: /^A unit strip is divided into/ });
    await unitPrompt.waitFor();
    const denominator = (await unitPrompt.innerText()).match(/into (\d+) equal/)[1];
    await page.getByRole('textbox', { name: 'Your answer' }).fill(`(1/${denominator})*2/2`);
    await page.getByRole('button', { name: 'Check answer', exact: true }).click();
    await page.getByRole('heading', { name: 'Answer checked locally' }).waitFor();
    evidence.push('Equivalent unit-fraction expressions are accepted by the shared exact checker.');
    await page.getByLabel('Choose an exercise').selectOption('arithmetic.whole.addition');
    await page.getByRole('heading', { name: /^Calculate \d+ \+ \d+\.$/ }).waitFor();

    await page.getByRole('button', { name: 'New example' }).click();
    await page.getByRole('button', { name: 'Check answer', exact: true }).waitFor({ state: 'visible' });
    await page.getByRole('textbox', { name: 'Your answer' }).fill('unfinished 37');
    await page.reload();
    await page.waitForLoadState('networkidle');
    await page.getByRole('button', { name: 'Practice' }).click();
    await page.getByLabel('Choose an exercise').selectOption('arithmetic.whole.addition');
    await page.getByRole('textbox', { name: 'Your answer' }).waitFor();
    assert.equal(await page.getByRole('textbox', { name: 'Your answer' }).inputValue(), 'unfinished 37');
    evidence.push('Unfinished practice input survives page reload and returning to its exercise family.');
    await page.getByRole('button', { name: 'Lessons' }).click();
    await page.getByLabel('Choose a lesson').selectOption('foundation.fractions.compare');
    for (const heading of ['6. Complete a partly worked problem', '7. Try independently', '8. Explain the reasoning', '9. Transfer to a new situation', '10. Retrieve after a delay']) {
      await page.getByRole('heading', { name: heading, exact: true }).waitFor();
    }
    await page.getByLabel('Your explanation (saved on this device)').fill('Equal-sized parts let me compare the numerators.');
    await page.getByLabel('Choose a lesson').selectOption('foundation.place_value.regrouping');
    await page.getByLabel('Choose a lesson').selectOption('foundation.fractions.compare');
    assert.equal(await page.getByLabel('Your explanation (saved on this device)').inputValue(), 'Equal-sized parts let me compare the numerators.');
    evidence.push('Expanded content is rendered from lesson files and reasoning drafts survive lesson changes.');
    await page.getByRole('button', { name: 'Progress', exact: true }).click();
    await page.getByRole('heading', { name: 'Practice history — mastery not yet assessed' }).waitFor();
    assert.equal(await page.getByText('Audit-Verified', { exact: true }).count(), 0);
    evidence.push('No fabricated mastery percentages or audit-verification claims are displayed.');
    await page.setViewportSize({ width: 320, height: 740 });
    const dimensions = await page.evaluate(() => ({ width: innerWidth, scroll: document.documentElement.scrollWidth }));
    assert.ok(dimensions.scroll <= dimensions.width + 1, JSON.stringify(dimensions));
    evidence.push('The tested progress journey has no page-wide overflow at 320 CSS pixels.');
    for (const tab of ['Explore', 'Lessons', 'Practice', 'Abacus']) {
      await page.getByRole('button', { name: tab, exact: true }).click();
      await page.waitForTimeout(100);
      const size = await page.evaluate(() => ({ width: innerWidth, scroll: document.documentElement.scrollWidth }));
      assert.ok(size.scroll <= size.width + 1, tab + ': ' + JSON.stringify(size));
    }
    evidence.push('Atlas, lessons, practice, abacus, and progress fit at 320 CSS pixels without page-wide overflow.');

    await page.screenshot({ path: root + '/axiom-review-mobile.png', fullPage: true });
    await page.setViewportSize({ width: 1280, height: 900 });
    await page.getByRole('button', { name: 'Abacus' }).click();
    await page.getByRole('button', { name: 'Free exploration', exact: true }).click();
    await page.getByRole('button', { name: 'Demo 15', exact: true }).click();
    assert.equal(await page.locator('.axiom-value-display').innerText(), '15');
    const upper = page.getByRole('button', { name: 'Ones Upper Bead: Engaged (5)', exact: true });
    await upper.focus();
    await page.keyboard.press('Enter');
    assert.equal(await page.locator('.axiom-value-display').innerText(), '10');
    evidence.push('Soroban value uses Rust; Enter activates the focused upper-bead control.');
    await page.getByLabel('Lower beads on Ones').selectOption('3');
    assert.equal(await page.locator('.axiom-value-display').innerText(), '13');
    evidence.push('The equivalent accessible lower-bead selector changes the Rust-checked value.');

    await page.screenshot({ path: root + '/axiom-review-desktop.png', fullPage: true });
    assert.deepEqual(errors, []);
    const report = { passed: true, evidence, browser: 'Installed Chrome, fresh Playwright context', errors, limits: ['No human content review or full screen-reader audit', 'No deployed account/storage validation', 'Service-worker offline behavior requires separate production-preview checks'] };
    fs.writeFileSync(root + '/browser-evidence.json', JSON.stringify(report, null, 2));
    console.log(JSON.stringify(report, null, 2));
  } catch (error) {
    await page.screenshot({ path: root + '/browser-failure.png', fullPage: true }).catch(() => {});
    fs.writeFileSync(root + '/browser-evidence.json', JSON.stringify({ passed: false, evidence, error: String(error), errors }, null, 2));
    console.error(error);
    process.exitCode = 1;
  } finally { await context.close(); await browser.close(); }
})();
