# NEEDS_MIKE (ready-to-approve, short)

## M1 — First paying user ($20 Pro) — **DO THIS FIRST**
- **What:** Complete checkout at https://cxlinux.com/pricing → **Start with Pro** → fill name/email → **Subscribe Now** → pay $20 on Stripe (AIVH LLC, live; product shows as **CX Core+**).
- **Why:** E2E already reached `checkout.stripe.com` `cs_live_…` without card entry. This is the remaining gate to “first paying user.”
- **Done:** Forward receipt + license key (or success-page screenshot) to Linus via Dez notes / this folder.
- **Undo:** Stripe cancel/refund within 7-day money-back copy on page.

## ~~M2~~ DONE (PR #57 merged Oct 20 ~8:53am MT; live /pricing returns 200, verified 9:14am MT)
## M2 — Deploy SPA soft-404 fix (after cloud PR)
- **What:** Approve/merge + deploy `cx-web` PR that makes https://cxlinux.com/pricing return **HTTP 200** (SEO).
- **Why:** Page already renders; crawlers/link previews see 404.
- **URL:** https://cxlinux.com/pricing

## M3 — Send dev re-engage (draft ready)
- **What:** Approve send of `../drafts/reengage-suyash-dhruv-2026-10-19.md` to suyash@mmivip.com + dhruv@aiventureholdings.com (via Rally/Michael — not Linus).
- **Why:** Need apt smoke on Ubuntu/Debian + PR triage capacity.

## M4 — Stripe catalog hygiene (optional same week)
- **What:** In Stripe (acct connected as AIVH LLC): leave working **CX Core+ $20** path; archive/reactivate old **Cortex*** products (`active:false` today) so dashboard isn’t confusing.
- **Why:** Search found Cortex Pro/Enterprise/Managed inactive; live checkout still sold “CX Core+”.
- **No Linus writes** — Mike only.

## M5 — Merge authority for week (optional)
- **What:** Allow Linus to merge **non-main** autonomy-doc PRs and/or approve specific `cx-web` PRs for T03–T05.
- **Default:** merges stay queued here per DECISIONS.md.

## M6 — Open PR batch (after T11 list)
- **What:** Merge/close aging `cx-core` PRs per triage list (to be attached).
- **URL:** https://github.com/cxlinux-ai/cx-core/pulls

- ~~M0~~ DONE 08:52 MT: grant Cursor's GitHub app access to the cxlinux-ai org (card sent in chat Oct 19). Recommendation: approve, all repos in cxlinux-ai.

- ~~M-PR57~~ DONE: merged Oct 20 ~8:53am MT, live verified. (was: merge https://github.com/cxlinux-ai/cx-web/pull/57 (pricing pages return 200 status, free Install button goes to /getting-started, "Pro" renamed "CX Core+" to match Stripe). READY (verified 09:15 MT: assets/blog/rss/sitemap OK on preview, junk 404, Core+ and Team checkout reach checkout.stripe.com via license.cxlinux.com). Recommendation: approve. Merging likely auto-deploys cxlinux.com; rollback = revert PR. Mark ready-for-review before merge.

- **M-CORE (most important), update Oct 20 9:15am MT:** #771 and #772 merged Oct 19 ~12:50pm MT. Left: #774 now has merge conflicts; a cloud agent is rebasing and slimming it (next cycle verifies). Then merge #774 and approve a new APT release. The live site already tells buyers `cx activate <key>`, which the published package can't do yet.
- (original) **M-CORE:** a buyer who pays $20 today can't activate a license, because the published package is missing the `activate` command (proof: https://github.com/cxlinux-ai/cx-core/pull/774). Recommendation, in order: merge https://github.com/cxlinux-ai/cx-core/pull/771 (fixes CI build) → https://github.com/cxlinux-ai/cx-core/pull/772 (safety fix) → https://github.com/cxlinux-ai/cx-core/pull/774 (license CLI + `cx` command + install smoke test), then approve publishing a new APT package (production release). Until then, pause pushing paid traffic.
- **M-DEPS:** 5 Dependabot Action bumps (#762 #761 #749 #748 #747) look routine. Recommendation: approve as a batch after #776 makes main CI green (main `build` still fails on the debian11 apt 404 as of 9:15am MT Oct 20).

- ~~M-PR58~~ DONE: merged Oct 20 ~8:52am MT (with #59). (was: merge https://github.com/cxlinux-ai/cx-web/pull/58 after #57 (checkout failure now shows a banner with Retry + support link; fixes a blog post that was dropped from listings). Local check/test/build pass; Cloudflare CI pending at 09:27 MT. Recommendation: approve once CI is green (next cycle verifies). Diff is +1643 lines, mostly tests; next cycle confirms no lockfile churn.

## Added 2026-10-20 (from Michael's 10 docs, digest at inputs/2026-10-20/DIGEST.md)
- M-LIST: Don't cold-email the bought 130,242-contact list (≈80% India). CAN-SPAM + India DPDP risk. Recommendation: don't use it; build an opt-in list instead.
- M-GHPROFILE: Make cxlinux-ai/.github public so the org profile shows. Low risk, reversible.
- M-LICENSE: Fix the LICENSE file so GitHub detects BSL 1.1 (it shows "Other" today). Legal-ish, so it needs your OK.
- M-MODEL: Default AI model downloads from a personal Hugging Face account (ShreemJ/cxlinux-ai-7b). Recommendation: move it to an AIVH-owned account.
- M-TIERNAME: The $20 tier is "Pro" on the site and "CX Core+" in Stripe. Recommendation: use "CX Core+" everywhere.
- M-LICENSE (update): LICENSE prices ($20/system, $99/25, $299/100) contradict the site, and the conversion date wording conflicts with the README. Needs one legal read.
- M-EMAIL: Approve the sender and opt-in list before drafts/email-seq-1-2-DRAFT.md goes anywhere.
- M-PR775: cx-core #775 (draft) https://github.com/cxlinux-ai/cx-core/pull/775. Makes dry-run the default everywhere ([y/N] prompts, `-y` needs `--execute`), adds a Firejail/bubblewrap sandbox (warns if neither is installed), and adds demo/safety-demo.sh plus docs/safety/CLAIMS_VERIFICATION.md. Recommendation: merge after CI is green and the demo log is checked; either it or #774 can go first, then rebase the other. Note: #772 is already on main (b1090ad13).

## Order of taps (Oct 20 9:15am MT)
1. **M-PR776:** merge https://github.com/cxlinux-ai/cx-core/pull/776 (CI unblock; all checks green except CodeQL, which fails in seconds). Then #775 can be updated and merged.
2. **M-PR775:** merge https://github.com/cxlinux-ai/cx-core/pull/775 after its CI re-runs on top of #776.
3. **M-CORE:** merge #774 once the rebase lands, then approve the APT release.
4. **M1:** the $20 test buy, best done after step 3 so activation works.
