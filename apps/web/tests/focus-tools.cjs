const fs=require('fs');const path=require('path');const os=require('os');const assert=require('assert/strict');
const runtime=process.env.AXIOM_RUNTIME_NODE_MODULES||path.join(os.homedir(),'.cache/codex-runtimes/codex-primary-runtime/dependencies/node/node_modules');
const {chromium}=require(require.resolve('playwright',{paths:[process.cwd(),runtime]}));
const output=path.resolve(__dirname,'../../../docs/evidence/focus-tools');fs.mkdirSync(output,{recursive:true});
const url=process.env.AXIOM_TEST_URL||'http://127.0.0.1:39018/';
(async()=>{const browser=await chromium.launch({channel:'chrome',args:['--no-proxy-server']});const page=await browser.newPage({viewport:{width:1440,height:1000},reducedMotion:'reduce'});const checks=[];const errors=[];page.on('pageerror',error=>errors.push(String(error)));
try{
 await page.goto(url);await page.getByRole('heading',{name:'Find your next idea',exact:true}).waitFor();

 // 1. The rotating pseudo-scientific wellness bar is gone.
 assert.equal(await page.locator('.axiom-wellness-bar').count(),0);
 assert.equal(await page.locator('text=vibrating at the frequency').count(),0);
 checks.push('The auto-rotating affirmation bar and its pseudo-scientific copy are removed.');

 // 2. The focus tools bar is present and static (no aria-live churn).
 const focusBar=page.locator('.axiom-focus-bar');await focusBar.waitFor();
 assert.equal(await focusBar.locator('[aria-live]').count(),0);
 checks.push('A static study focus tools bar replaces moving content, with no automatic live-region updates.');

 // 3. Focus view hides secondary chrome and persists.
 await page.getByRole('button',{name:'Focus view',exact:true}).click();
 assert.ok((await page.locator('.axiom-app').getAttribute('class')).includes('axiom-focus-view'));
 assert.equal(await page.locator('.axiom-xp-cap').isVisible(),false);
 assert.equal(await page.locator('.axiom-footer').isVisible(),false);
 assert.equal(await page.getByLabel('Main Navigation').getByRole('button',{name:'Abacus',exact:true}).isVisible(),true);
 await page.screenshot({path:path.join(output,'focus-view-atlas-1440.png'),fullPage:true});
 await page.reload();await page.getByRole('heading',{name:'Find your next idea',exact:true}).waitFor();
 assert.ok((await page.locator('.axiom-app').getAttribute('class')).includes('axiom-focus-view'));
 checks.push('Focus view hides XP details and the footer, keeps all navigation visible, and persists across reloads.');
 await page.getByRole('button',{name:'Focus view: on',exact:true}).click();
 assert.equal((await page.locator('.axiom-app').getAttribute('class')).includes('axiom-focus-view'),false);
 assert.equal(await page.locator('.axiom-footer').isVisible(),true);

 // 4. Optional timer: start, pause, resume, end. Nothing auto-starts.
 await page.getByLabel('Focus session').selectOption('10');
 await page.getByRole('button',{name:'Start session',exact:true}).click();
 const timer=page.getByRole('timer',{name:'Focus session remaining'});await timer.waitFor();
 assert.ok((await timer.innerText()).includes('10:00'));
 await page.getByRole('button',{name:'Pause',exact:true}).click();
 await page.getByRole('button',{name:'Resume',exact:true}).click();
 await page.getByRole('button',{name:'End session',exact:true}).click();
 await page.getByRole('button',{name:'Start session',exact:true}).waitFor();
 checks.push('The optional focus timer starts, pauses, resumes and ends on request without interrupting study.');

 // 5. Pause panel: honest copy, manually opened and closed.
 await page.getByRole('button',{name:'Take a pause',exact:true}).click();
 const dialog=page.getByRole('dialog');await dialog.waitFor();
 assert.ok((await dialog.innerText()).includes('not medical advice'));
 assert.ok((await dialog.innerText()).includes('Breathe in'));
 await page.screenshot({path:path.join(output,'pause-panel.png')});
 await page.getByRole('button',{name:'Close the pause panel',exact:true}).click();
 assert.equal(await dialog.count(),0);
 checks.push('The pause panel opens only on request, states honest non-medical boundaries and closes cleanly.');

 // 6. Working notes scratchpad persists with the local practice draft.
 await page.getByLabel('Main Navigation').getByRole('button',{name:'Practice',exact:true}).click();
 await page.getByLabel('Choose an exercise').waitFor();
 await page.locator('.axiom-scratchpad summary').click();
 await page.getByLabel('Working notes (kept on this device; never graded)').fill('Common denominator first: 3/4 = 6/8.');
 await page.waitForTimeout(400);
 await page.reload();await page.getByLabel('Main Navigation').getByRole('button',{name:'Practice',exact:true}).click();
 await page.getByLabel('Choose an exercise').waitFor();
 await page.locator('.axiom-scratchpad summary').click();
 assert.equal(await page.getByLabel('Working notes (kept on this device; never graded)').inputValue(),'Common denominator first: 3/4 = 6/8.');
 checks.push('Working notes are restored with the saved practice draft and stay ungraded local text.');

 // 7. Focus bar and study views fit at 320 pixels.
 await page.setViewportSize({width:320,height:844});
 for(const [label,file] of [['Explore','atlas'],['Practice','practice']]){
  await page.getByLabel('Main Navigation').getByRole('button',{name:label,exact:true}).click();await page.waitForTimeout(150);
  const dimensions=await page.evaluate(()=>({width:innerWidth,scroll:document.documentElement.scrollWidth}));
  assert.ok(dimensions.scroll<=321,`${label} 320: ${JSON.stringify(dimensions)}`);
  await page.screenshot({path:path.join(output,`${file}-320.png`),fullPage:true});
 }
 checks.push('The focus tools bar and study views fit a 320 pixel viewport without horizontal scroll.');

 // 8. Session completion (deterministic clock): one honest status message, no modal.
 const timerPage=await browser.newPage({viewport:{width:1440,height:1000},reducedMotion:'reduce'});
 timerPage.on('pageerror',error=>errors.push(String(error)));
 await timerPage.clock.install();
 await timerPage.goto(url);await timerPage.getByRole('heading',{name:'Find your next idea',exact:true}).waitFor();
 await timerPage.getByRole('button',{name:'Start session',exact:true}).click();
 await timerPage.clock.fastForward('20:01');
 await timerPage.getByText('Session finished. A short break may help, or keep going—either is fine.').waitFor();
 await timerPage.getByRole('button',{name:'Keep going',exact:true}).click();
 await timerPage.getByRole('button',{name:'Start session',exact:true}).waitFor();
 await timerPage.close();
 checks.push('A finished session shows one polite break suggestion with a keep-going action and no interrupting modal.');

 assert.deepEqual(errors,[]);
 const result={passed:true,checks,errors,limits:['Installed Chrome against a local dev server; not all browsers, devices or assistive technology.','This is not an ADHD clinical evaluation; focus tools are ordinary optional UI preferences, not treatment.','Timer completion uses Playwright clock fast-forwarding, not a real 20 minute wait.']};
 fs.writeFileSync(path.join(output,'focus-evidence.json'),JSON.stringify(result,null,2));console.log(JSON.stringify(result,null,2));
}catch(error){await page.screenshot({path:path.join(output,'focus-failure.png'),fullPage:true});console.error(error);process.exitCode=1;}finally{await browser.close()}})();
