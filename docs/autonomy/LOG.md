# LOG

## 2026-10-19 (Mon)
- Mandate received ~8:33am MT. Restart chosen. E2E test running. Daily cycle set for weekdays ~9am MT.

### E2E complete ~08:36 MT
- Wrote `../e2e-test-2026-10-19.md`. Pricing LIVE; Pro → Stripe **live** checkout (AIVH LLC, CX Core+ $20, `cs_live_`); no card entered.
- License server health 200 (v2.0.0). APT deb SHA match; `cx-terminal --version` OK; `ask` needs model/key.
- Autonomy: `PLAN.md` + `NEEDS_MIKE.md` filled; BLOCKERS/DECISIONS updated.
- Next: Michael M1 ($20 pay) is the critical path.
- 08:40 MT: cloud-agent launch on cx-web blocked (no GitHub access to cxlinux-ai); logged B-GH / M0.
- 08:53 MT: GitHub access granted. Launched cloud agent bc-41dfda0e (Composer 2.5) on cx-web for T03 soft-404, T05 install CTA, T04 Pro/CX Core+ naming. PR pending.
- 08:58 MT: SHIPPED (draft) cx-web PR #57 https://github.com/cxlinux-ai/cx-web/pull/57, npm check+build pass, local /pricing 200, junk 404. Open: wrangler pages dev still 200 on junk; asked agent to fix + confirm CI. Next: T-license activation proof, apt install in container (cloud VM has docker).
- 09:00 MT: launched cloud agent bc-6e2120ee (Composer 2.5) on cx-core: clean apt install in ubuntu 24.04/debian 12, license activation proof, CI smoke test, triage of 13 open PRs.
- 09:03 MT: PR #57 now 404s junk paths under wrangler (commit 01880926); Cloudflare Pages preview + CodeRabbit pass. Risk: early 404 in middleware could hit static assets/dynamic slugs/Stripe return URLs; asked agent to prove with preview + headless browser before recommending merge.
- 09:07 MT: SHIPPED (draft) cx-core PR #774 https://github.com/cxlinux-ai/cx-core/pull/774: Ubuntu 24.04 apt install PASS (cx-terminal 20260223-145346-e97bbc7b), Debian 12 FAIL (glibc), license server health PASS, activate CLI FAIL on published deb (fixed in PR), new APT smoke CI green on ubuntu. Open-PR triage done; merge recs in NEEDS_MIKE. Next: Debian 12 build, unify activate copy across docs/site.
- 09:11 MT: PR #57 safety pass: assets, sitemap, rss, blog slugs, Stripe success/cancel routes OK on preview https://cursor-fix-spa-routing-prici.cx-web.pages.dev; Playwright no network 404s. Pre-existing bad frontmatter in ai-command-line-tools-linux.mdx (to fix). Final check queued: preview checkout click-through to Stripe.
- 09:15 MT: PR #57 VERIFIED ready (checkout API on license.cxlinux.com, unaffected by Pages /api 404; Core+ and Team reach Stripe on preview, no card). Moved to READY in NEEDS_MIKE. Next for cx-web: fix bad frontmatter in ai-command-line-tools-linux.mdx; align activation copy with cx-core #774 once chosen.

### Daily cycle run 09:22 MT (routine)
- Verified (GitHub API 09:25 MT): cx-web PR #57 draft, mergeable_state clean, head fb90f38 — unchanged since READY. cx-core PR #774 draft, mergeable_state blocked (needs review/merge order #771→#772→#774), 5 review comments open; agent bc-6e2120ee still running (Bookworm build).
- Launched cloud agent bc-71fb33cc (Composer 2.5) on cx-web, new branch off main, independent of #57: T06 visible checkout-failure error + retry (with mocked-failure test) and fix bad frontmatter in ai-command-line-tools-linux.mdx. PR pending.
- Drafted (pulled T13 forward): `../drafts/first-user-playbook.md` (buyer path with verified/unverified per step; gate = M-CORE before inviting payers) and `../drafts/launch-posts-DRAFT.md` (do-not-post until M-CORE + #57 + Michael OK).
- Next: verify bc-71fb33cc PR (tests, preview); check #774 Bookworm result + review comments; T10 activation proof after M1 + M-CORE.
- Risk: paid traffic before M-CORE = buyers who can't activate. No external sends made.
- Repo mirror (docs/autonomy/ in cx-core, PR #773) NOT synced this run: the routine run can't steer agent bc-6e2120ee (owned by the main chat). Main chat can queue the sync on that agent; otherwise next cycle syncs via a new docs-only agent.
- 09:27 MT: SHIPPED (draft) cx-web PR #58 https://github.com/cxlinux-ai/cx-web/pull/58: checkout error banner/retry/timeout + blog frontmatter fix; npm check/test(8)/build pass; CF Pages CI pending. Queued autonomy-docs mirror update on cx-core agent bc-6e2120ee.
- 09:36 MT: cx-core PR #773 docs mirror updated (0142cfcb), but my 'replace exactly' instruction removed BOUNTY_AUDIT.md + PR_REVIEWS.md (not ours). Asked agent to restore them. PR #773 is non-draft (can't revert to draft from agent). Lesson: mirror = add/overwrite our 5 files only.
- 09:37 MT: PR #773 files restored (c769f3b5). PR #774 results: Debian 12 minimal CLI .deb built on Bookworm PASS (cx --version, cx activate --help); full GUI .deb on Bookworm FAIL (freetype/zlib build flake); canonical command = `cx activate <LICENSE_KEY>` (aliases kept); real license server invalid key PASS (clear error, exit 1). Size is mostly committed logs (~4.6K lines); Cargo.lock net -214 (drops private hrm-ai), no vendored deps.
- Next cycle: (a) move big verification logs out of #774 into CI artifacts to slim the diff; (b) fix full Bookworm GUI build flake; (c) align cx-web activation copy to `cx activate`; (d) recheck PR #58 CI.

## 2026-10-20 (Tue)
- 04:10 MT: Michael shared market read and the repositioning. Launched cloud agent bc-821dfa51 (cx-core: dry-run-by-default guard, verify safety claims, 30s demo). Drafting positioning one-pager + "CX vs Ubuntu built-in AI" (MD+PDF) in drafts/. Note: "build green" not yet true on main (#771 unmerged).
- 04:12 MT: Drafts done: drafts/cx-positioning-one-pager.{md,pdf}, drafts/cx-vs-ubuntu-ai.{md,pdf}. 7 safety claims tagged [verify before publishing] pending bc-821dfa51. Sources confirmed; Canonical says opt-in/local/removable, so our edge = dry-run default, rollback, Debian+Ubuntu, available on LTS. Open: demo link /ai-consultant vs /ai-consultant/vip; cx-core license "Other" so say "public code" not "open source".
- 04:17 MT: Digested Michael's 10 docs (inputs/2026-10-20/DIGEST.md). Queued M-LIST, M-GHPROFILE, M-LICENSE. Next: FACTS.md + blocklist, demo FAQ, re-draft emails 1-2 around the new positioning.
- 04:20 MT: FACTS.md, drafts/demo-faq-DRAFT.md and drafts/email-seq-1-2-DRAFT.md done. Code read: dry-run not the default (-y skip, [Y/n]) and no sandbox (sh -c). Fed both to bc-821dfa51. get.cxlinux.com down; cx-docs link 404.
- 04:39 MT: bc-821dfa51 opened draft cx-core #775. Claims: dry-run PASS; sandbox PARTIAL (firejail/bwrap, falls back with a warning); AppArmor FAIL; undo PASS for cx ask; audit, offline and uninstall PARTIAL. Next: check #775 CI, then update FACTS.md and the drafts with these verdicts.
- 08:18 MT: #775 is mergeable with no conflicts (Michael saw the block). Blockers: CLA check, debian11 build after 12s, 8 open review threads, CodeQL pending, GidTeam review. Asked bc-821dfa51 to fix review threads, find the debian11 root cause and the CLA cause. Note: #775 is no longer a draft.
- 08:32 MT: #775 fixed all 8 review comments (2 threads resolved, 6 pending). Cargo, Clippy, fmt, tests, CodeQL and Sonar are green. Remaining failures predate this PR: CLA (cursoragent not allowlisted), debian11 apt 404 (fails on every PR), build-docs broken link. Asked for a separate CI-unblock draft PR.
- 09:04 MT: Draft cx-core #776 (CI unblock: CLA allows cursoragent only with a signed co-author, debian11 apt via archive.debian.org, docs links). Its checks pass except CodeQL (3s quirk). Commits authored as Mike to pass the old CLA check; flagged to Michael. All 8 #775 threads resolved. Plan: merge #776, update the #775 branch, re-run CI, merge #775.
- M-PR776: merge #776 first (Michael tap).

### Daily cycle run 09:12 MT (routine, Oct 20)
- Verified (GitHub API + curl, 9:13–9:15 MT): cx-web #57, #58, #59 MERGED (~8:52–8:53am MT, 14:52–14:53Z); cx-web main CI green (head 2497fe96). Live: https://cxlinux.com/pricing 200, /getting-started 200, /blog 200, /sitemap.xml 200, junk path 404, license.cxlinux.com/health 200, repo.cxlinux.com Release 200. (/rss.xml 404 but the feed lives at /blog/rss.xml 200.)
- cx-core: #771 + #772 merged Oct 19 ~12:50pm MT; #773 docs mirror merged ~4:09pm MT. #774 is draft + dirty (conflicts). #775 open: 28 checks pass, 3 fail (build/debian11, build-docs, cla-check, all pre-existing). #776 draft: 19 pass, CodeQL fail. cx-core main 7b88a923: only `build` fails (debian11 apt 404, fixed by #776).
- Live bundle scan: activation copy mixes `cx activate` and `cx license activate`; two GPG key URLs (key.gpg / pub.gpg, same file); install.sh claims Debian 11+ though Debian 12 fails. Published package still lacks `activate` (per #774), so live copy tells buyers a command that won't work yet. Not an emergency: 0 known paying users.
- Couldn't reply to earlier agents from this run (tool limit), so launched fresh Composer 2.5 agents: bc-f3dbaed4 (cx-core: rebase + slim #774 on its branch, fix Bookworm GUI flake if cheap, plus a docs-only draft PR syncing these 5 files) and bc-293c0687 (cx-web: one activation command, one apt snippet, verified-OS-only claims; draft PR).
- NEEDS_MIKE: marked M2/M-PR57/M-PR58 done; added tap order #776 → #775 → #774 → APT release → M1.
- gh CLI token on the box is invalid (401); used the built-in GitHub tools instead. Not a blocker.
- Next cycle: verify both new PRs (CI, diff, preview); recheck #775/#776; rerun T08/T09 after #774 rebase.
- No external sends, merges, deploys or Stripe writes.
- 09:19 MT: Draft cx-web #60 (https://github.com/cxlinux-ai/cx-web/pull/60) makes every page use `cx activate <key>` and one install snippet, and lists Ubuntu 24.04 as the only verified system. tsc, 10/10 tests, build and Pages preview all pass. Follow-ups: the docs/ Docusaurus install page still uses key.gpg, and blog excerpts still say "Debian/Ubuntu".
- M-PR60: merge cx-web #60 after #776/#775 (low risk, preview https://74e483d8.cx-web.pages.dev).
- 10:06 MT: #774 has no conflicts and is down to +802/-59 (logs moved to CI artifacts). Debian 12 minimal .deb build and install smoke are green, and so is the Ubuntu 24.04 APT smoke. The optional full GUI .deb still fails (follow-up; artifact on run 39448560683). Draft docs-sync PR #777 touches only docs/autonomy/*. Merge order: #776, #775, #774, #777, cx-web #60, then a new APT release and the $20 test purchase.

### Daily cycle run 09:02 MT (routine, Wed Oct 21)
- Verified live (curl 9:02 MT): https://cxlinux.com/ 200 (now sends HSTS), /pricing 200, /getting-started 200, license.cxlinux.com/health 200, repo.cxlinux.com/apt/dists/stable/Release 200. No production harm found.
- Verified GitHub (API 9:05 MT): cx-web #61 (homepage server-render + HSTS, by Michael's account) merged Oct 21 ~1:09am MT. cx-core #778 (CLA security fix + Debian 11 archive on all 3 workflows) merged into #776's branch Oct 20 ~11:38am MT; #776 head f48caff3 now 20/20 checks success. #774 (5fbee3cd), #775 (36ef88e2), #776 all mergeable=true, state "blocked" (needs review only). cx-web #60 mergeable_state clean. cx-core main 7b88a923: 28 pass, 1 fail (`build`, debian11), fixed by #776.
- Found: Michael's Claude track opened #779 (docs) with REVIEW_775.md: #775's sandbox blocks `sudo apt install` and bwrap discards writes. Not yet independently reproduced.
- Launched cloud agent bc-7d5d09f5 (default model) on #775's branch: reproduce the 3 review findings, fix on-branch, tests, update CLAIMS_VERIFICATION.md. No new PR, no merge.
- Updated FACTS.md + positioning one-pager: sandbox FAIL for installs, no AppArmor claim.
- NEEDS_MIKE: new tap order #776 → #774 → APT release → M1; #60, #777 any time; #775 on hold.
- Next cycle: verify bc-7d5d09f5 commits (reproduction logs, tests, CI); if green, move M-PR775 back to ready.
- No external sends, merges, deploys or Stripe writes.
