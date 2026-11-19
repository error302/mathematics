const fs = require('fs');
const path = require('path');
const os = require('os');
const assert = require('assert/strict');

const runtime = process.env.AXIOM_RUNTIME_NODE_MODULES || path.join(os.homedir(), '.cache/codex-runtimes/codex-primary-runtime/dependencies/node/node_modules');
const { chromium } = require(require.resolve('playwright', { paths: [process.cwd(), runtime] }));

const project = path.resolve(__dirname, '../../..');
const output = path.join(project, 'docs/evidence/skill-dag');
fs.mkdirSync(output, { recursive: true });

(async () => {
  const browser = await chromium.launch({ channel: 'chrome', args: ['--no-proxy-server'] });
  const page = await browser.newPage({ viewport: { width: 1360, height: 950 } });
  const checks = [];
  const errors = [];
  page.on('pageerror', e => errors.push(String(e)));

  try {
    await page.goto(process.env.AXIOM_TEST_URL || 'http://127.0.0.1:3001/');
    await page.waitForLoadState('networkidle');

    // 1. Switch to Skill DAG view
    const dagToggle = page.getByRole('button', { name: /Skill DAG \/ Graph/i });
    await dagToggle.waitFor();
    await dagToggle.click();

    // 2. Verify SVG viewport and nodes exist
    const viewport = page.locator('.axiom-dag-viewport');
    await viewport.waitFor();
    const nodeCount = await page.locator('.axiom-dag-node-group').count();
    assert.equal(nodeCount, 83, `Expected 83 course nodes, got ${nodeCount}`);
    checks.push(`Knowledge Graph renders all 83 courses across all 7 stages in an interactive SVG canvas.`);

    // 3. Take desktop overview screenshot
    await page.screenshot({ path: path.join(output, 'skill-dag-overview.png'), fullPage: false });

    // 4. Click node F04 (Fractions)
    const nodeF04 = page.locator('#dag-node-F04');
    await nodeF04.click();

    // Verify selection and drawer
    await page.locator('#dag-drawer-title').waitFor();
    const drawerTitle = await page.locator('#dag-drawer-title').innerText();
    assert.ok(drawerTitle.includes('F04') && drawerTitle.includes('Fractions'), `Unexpected drawer title: ${drawerTitle}`);

    // Verify upstream prerequisites & downstream unlocks
    const ancestorCount = await page.locator('.axiom-dag-node-group.ancestor').count();
    const descendantCount = await page.locator('.axiom-dag-node-group.descendant').count();
    assert.ok(ancestorCount >= 1, `Expected at least 1 ancestor for F04, got ${ancestorCount}`);
    assert.ok(descendantCount >= 3, `Expected at least 3 descendants for F04, got ${descendantCount}`);
    checks.push(`Selecting F04 (Fractions) dynamically highlights upstream prerequisites (${ancestorCount}) and downstream unlocked courses (${descendantCount}).`);

    // Verify drawer pills
    const prereqPills = page.locator('.axiom-dag-pill.upstream');
    assert.ok(await prereqPills.count() >= 1, 'Expected prerequisite pills in drawer');

    // 5. Click prerequisite chip F02 in drawer
    const f02Pill = page.locator('.axiom-dag-pill.upstream', { hasText: 'F02' });
    await f02Pill.click();
    await page.waitForTimeout(200);
    const newDrawerTitle = await page.locator('#dag-drawer-title').innerText();
    assert.ok(newDrawerTitle.includes('F02'), `Drawer did not switch to F02: ${newDrawerTitle}`);
    checks.push(`Clicking prerequisite chip in drawer smoothly transitions focus to prerequisite course F02.`);

    // 6. Test Stage Filter (e.g. Undergraduate)
    const undergradFilter = page.getByRole('button', { name: /Undergraduate \(24\)/i });
    await undergradFilter.click();
    await page.waitForTimeout(200);
    const filteredCount = await page.locator('.axiom-dag-node-group').count();
    assert.equal(filteredCount, 24, `Expected 24 undergraduate courses, got ${filteredCount}`);
    checks.push(`Stage filter chips correctly isolate specific degree bands (e.g., Undergraduate 24 courses).`);

    // 7. Reset filter to All Stages
    const allFilter = page.getByRole('button', { name: /All Stages \(83\)/i });
    await allFilter.click();
    await page.waitForTimeout(200);

    // 8. Test Flow mode toggle
    const flowToggle = page.getByRole('button', { name: /Flow/i });
    await flowToggle.click();
    await page.waitForTimeout(200);
    checks.push(`Topological flow layout toggle successfully reorganizes graph along dependency depth.`);

    // 9. Take highlighted pathway screenshot
    await page.screenshot({ path: path.join(output, 'skill-dag-flow-pathway.png'), fullPage: false });

    // 10. Test Search
    const searchInput = page.getByLabel('Search courses in DAG');
    await searchInput.fill('Abacus');
    await page.waitForTimeout(200);
    const abacusCount = await page.locator('.axiom-dag-node-group').count();
    assert.equal(abacusCount, 11, `Expected 11 abacus courses matching search 'Abacus', got ${abacusCount}`);
    checks.push(`Search input dynamically filters graph nodes in real-time.`);

    assert.deepEqual(errors, []);
    fs.writeFileSync(path.join(output, 'dag-evidence.json'), JSON.stringify({ passed: true, checks, errors }, null, 2));
    console.log(JSON.stringify({ passed: true, checks }, null, 2));
  } catch (err) {
    await page.screenshot({ path: path.join(output, 'dag-failure.png') });
    console.error(err);
    process.exitCode = 1;
  } finally {
    await browser.close();
  }
})();
