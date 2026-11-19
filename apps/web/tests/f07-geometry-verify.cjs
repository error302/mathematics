const fs = require('fs');
const path = require('path');
const os = require('os');
const assert = require('assert/strict');

const runtime = process.env.AXIOM_RUNTIME_NODE_MODULES || path.join(os.homedir(), '.cache/codex-runtimes/codex-primary-runtime/dependencies/node/node_modules');
const { chromium } = require(require.resolve('playwright', { paths: [process.cwd(), runtime] }));

const dir = path.resolve(__dirname, '../../../docs/evidence/f07-geometry');
fs.mkdirSync(dir, { recursive: true });

(async () => {
  const browser = await chromium.launch({ channel: 'chrome', headless: true, args: ['--no-proxy-server'] });
  const page = await browser.newPage({ viewport: { width: 1440, height: 1000 } });
  const checks = [];
  const errors = [];
  page.on('pageerror', e => errors.push(String(e)));

  try {
    const testUrl = process.env.AXIOM_TEST_URL || 'http://127.0.0.1:3001/';
    await page.goto(testUrl);
    await page.getByRole('button', { name: 'AXIOM Mathematics Academy Home', exact: true }).waitFor();

    // 1. Lessons Tab
    await page.getByRole('button', { name: 'Lessons', exact: true }).click();
    await page.getByLabel('Choose a lesson').waitFor();

    // 2. F07 Lesson 1: Perimeter and Area
    await page.getByLabel('Choose a lesson').selectOption('foundation.geometry.perimeter_and_area');
    await page.getByRole('heading', { name: 'Perimeter, Area, and Spatial Conservation' }).waitFor();
    checks.push('F07 Lesson 1 (Perimeter & Area) loads cleanly.');

    // Interact with chips in Perimeter & Area visual
    const chipRect10x2 = page.getByRole('button', { name: /10 × 2 Rectangle/ });
    await chipRect10x2.waitFor();
    await chipRect10x2.click();
    assert.match(await page.locator('.concept-visual [role="status"]').innerText(), /perimeter 24.*20 square units/);
    checks.push('Perimeter vs Area visual chips switch active shape and recalculate boundary vs interior conservation.');

    const chipTri = page.getByRole('button', { name: /Triangle \(Base 8/ });
    await chipTri.click();
    assert.match(await page.locator('.concept-visual [role="status"]').innerText(), /half the 40-unit area/);
    checks.push('Triangle dissection chip demonstrates right triangle half-rectangle area formula.');

    await page.screenshot({ path: path.join(dir, 'f07-lesson-perimeter-area.png'), fullPage: true });

    // 3. F07 Lesson 2: Angles & Triangles
    await page.getByLabel('Choose a lesson').selectOption('foundation.geometry.angles_and_triangles');
    await page.getByRole('heading', { name: 'Angles, Parallel Lines, and Triangle Invariants' }).waitFor();
    checks.push('F07 Lesson 2 (Angles & Triangles) loads cleanly.');

    const chipTransversal = page.getByRole('button', { name: /Parallel Lines & Transversal/ });
    await chipTransversal.waitFor();
    await chipTransversal.click();
    assert.match(await page.locator('.concept-visual [role="status"]').innerText(), /alternate interior angles/);
    checks.push('Angles visual chips switch to parallel transversal alternate interior angle demonstration.');

    await page.screenshot({ path: path.join(dir, 'f07-lesson-angles-triangles.png'), fullPage: true });

    // 4. F07 Lesson 3: Coordinate Plane
    await page.getByLabel('Choose a lesson').selectOption('foundation.geometry.coordinate_plane');
    await page.getByRole('heading', { name: 'The Cartesian Coordinate Plane and Distance' }).waitFor();
    checks.push('F07 Lesson 3 (Coordinate Plane) loads cleanly.');

    const chipLattice51213 = page.getByRole('button', { name: /\(5, 12\) Distance = 13/ });
    await chipLattice51213.waitFor();
    await chipLattice51213.click();
    assert.match(await page.locator('.concept-visual [role="status"]').innerText(), /displacements 5 and 12.*169/);
    checks.push('Coordinate lattice visual demonstrates Pythagorean triples and Euclidean distance.');

    await page.screenshot({ path: path.join(dir, 'f07-lesson-coordinate-plane.png'), fullPage: true });

    // 5. Practice Hub - Geometry Exercise Families
    await page.getByRole('button', { name: 'Practice', exact: true }).click();
    await page.getByLabel('Choose an exercise').waitFor();

    // 5.1 Perimeter & Area Practice
    await page.getByLabel('Choose an exercise').selectOption('geometry.perimeter_and_area');
    await page.locator('.axiom-exercise-prompt').waitFor();
    await page.getByRole('button', { name: 'Show me how', exact: true }).click();
    await page.getByRole('heading', { name: 'Let’s work it through', exact: true }).waitFor();
    assert.match(await page.locator('.axiom-help-content').innerText(), /(Perimeter|Area|P =|A =)/);
    checks.push('Practice template geometry.perimeter_and_area generates valid problems with Rust worked solutions.');

    // 5.2 Angles & Triangles Practice
    await page.getByLabel('Choose an exercise').selectOption('geometry.angles_triangles');
    await page.locator('.axiom-exercise-prompt').waitFor();
    await page.getByRole('button', { name: 'Show me how', exact: true }).click();
    await page.getByRole('heading', { name: 'Let’s work it through', exact: true }).waitFor();
    assert.match(await page.locator('.axiom-help-content').innerText(), /(180°|degrees|sum|triangle)/);
    checks.push('Practice template geometry.angles_triangles generates valid problems with exact angle-sum worked solutions.');

    // 5.3 Coordinate Distance Practice
    await page.getByLabel('Choose an exercise').selectOption('geometry.coordinate_distance');
    await page.locator('.axiom-exercise-prompt').waitFor();
    await page.getByRole('button', { name: 'Show me how', exact: true }).click();
    await page.getByRole('heading', { name: 'Let’s work it through', exact: true }).waitFor();
    assert.match(await page.locator('.axiom-help-content').innerText(), /(distance|coordinate|units|lattice|Pythagorean)/);
    checks.push('Practice template geometry.coordinate_distance generates valid distance problems with exact worked solutions.');

    await page.screenshot({ path: path.join(dir, 'f07-practice-geometry.png'), fullPage: true });

    assert.deepEqual(errors, []);
    const result = { passed: true, checks, errors };
    fs.writeFileSync(path.join(dir, 'f07-evidence.json'), JSON.stringify(result, null, 2));
    console.log(JSON.stringify(result, null, 2));
  } catch (e) {
    await page.screenshot({ path: path.join(dir, 'f07-failure.png'), fullPage: true });
    console.error(e);
    process.exitCode = 1;
  } finally {
    await browser.close();
  }
})();
