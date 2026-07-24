# Architecture Decision Records (ADRs)

This directory tracks significant architectural decisions made across the project. For the process of proposing or updating records, refer to the guidelines below.

## Quick Links
- [ADR Template](TEMPLATE.md)
- [Project Architecture notes](../notes/)

---

## Log of Architecture Decisions

| ID | Title | Status | Scope | Date |
| :--- | :--- | :--- | :--- | :--- |
<!-- | [ADR-0001](0001-short-title.md) | Short Title of Decision | Accepted | Compiler | 2026-06-15 | -->
<!-- | [ADR-0002](0002-short-title.md) | Short Title of Decision | Proposed | Syntax | 2026-07-01 | -->

---

## How to Create a New ADR

1. **Copy the Template:** Copy [`TEMPLATE.md`](TEMPLATE.md) to a new file in this directory named `XXXX-short-title.md` (e.g., `0003-async-runtime.md`).
2. **Draft the Decision:** Fill out all applicable sections. Pay specific attention to **Scope**, **Breaking Changes**, and clear **Before/After** examples.
3. **Submit a Pull Request:** Change status to `Proposed` and open a PR for team review and discussion.
4. **Finalize:** Once consensus is reached, update the status to `Accepted` (or `Rejected`) and add the entry to the index table above.

---

## ADR Lifecycle & Statuses

* **Proposed:** Under active discussion and review.
* **Accepted:** Approved and set as the project standard.
* **Rejected**: Reviewed and decided against. Kept for historical context to avoid re-evaluating the same idea later.
* **Deprecated:** Previously accepted, but no longer recommended practice.
* **Superseded:** Replaced by a newer record (always reference the replacing ADR, e.g., `Superseded by ADR-0012`).
