# Senior Engineering Audit Checklist

**Purpose**: The lead engineer (human or AI) reads this every review cycle. It contains every question a senior engineering team would ask about a production codebase. The lead brings these up proactively -- never waits for someone to ask.

**Read this BEFORE starting work. Check each section. Flag anything that has fallen behind.**

---

## 1. Product Management

- [ ] Is the product vision documented and current?
- [ ] Are user personas defined? Who are the target users?
- [ ] Is there a prioritized feature backlog or roadmap?
- [ ] Are we building what users need or what is easy to build?
- [ ] When did we last validate the vision against real user needs?
- [ ] Are success metrics defined? (Not vanity metrics -- real user satisfaction.)
- [ ] Is there a v0.1 / MVP definition? What is the minimum viable experience?

---

## 2. Software Architecture

- [ ] Is the architecture documented? (ADRs, diagrams, layer descriptions)
- [ ] Are module boundaries clean? (No circular dependencies, no God objects)
- [ ] Is the largest file acceptable? (Any file > 500 lines that is not data?)
- [ ] Are there circular dependencies?
- [ ] Is the dependency graph documented and reviewed?
- [ ] Are there God Objects or God Functions? (> 100 lines in a single function)
- [ ] Is error handling consistent across the codebase?
- [ ] Is state management clean? (Centralized, predictable, debuggable)

---

## 3. Code Quality

- [ ] Is static analysis running? (Linters, type checkers in CI)
- [ ] Are code formatters enforced? (Pre-commit hooks or CI checks)
- [ ] Is code coverage measured and tracked? (Target: project-appropriate %)
- [ ] Are coding standards documented and enforced automatically?
- [ ] Is there a pre-commit hook catching issues?
- [ ] Is dead code being removed? (Regular cleanup sweeps)
- [ ] Are refactor candidates tracked and addressed?

---

## 4. Testing Strategy (The Pyramid)

- [ ] **Unit tests**: Sufficient coverage of pure logic? (Target: 80%+)
- [ ] **Integration tests**: Critical boundaries covered? (DB, APIs, services)
- [ ] **Contract tests**: API contracts validated between services?
- [ ] **Property-based tests**: Mathematical invariants verified?
- [ ] **Smoke tests**: Does the app start and serve basic requests?
- [ ] **End-to-end tests**: Can a user complete a critical journey?
- [ ] **Performance tests**: Are response time budgets enforced?
- [ ] **Visual regression**: Do screenshots match baseline? (If UI exists)
- [ ] **Load tests**: How many concurrent users can the system handle?
- [ ] **Accessibility tests**: WCAG compliance automated? (If UI exists)
- [ ] **Security tests**: Injection, XSS, CORS, auth bypass checked?

---

## 5. DevOps & Deployment

- [ ] Is CI/CD running on every push?
- [ ] Are there separate environments? (dev / staging / production)
- [ ] Is deployment automated? (One command or one merge to deploy)
- [ ] Is the build artifact size tracked? (Binary, bundle, container image)
- [ ] Are there performance budgets enforced in CI?
- [ ] Is there a rollback strategy? (Can we undo a bad deploy in < 5 minutes?)
- [ ] Is there monitoring? (Error reporting, uptime, key metrics)
- [ ] Is there alerting? (PagerDuty, Slack, email for critical failures)
- [ ] Is there a status page? (Public or internal)

---

## 6. Documentation

- [ ] Is the API documented? (OpenAPI, JSDoc, docstrings, header comments)
- [ ] Is the architecture documented? (ADRs, diagrams)
- [ ] Is there a contributor guide? (For humans and/or AI agents)
- [ ] Is the build process documented? (README, Makefile, scripts)
- [ ] Are decisions logged? (DECISIONS.md, ADR directory)
- [ ] Is there a changelog? (CHANGELOG.md or release notes)

---

## 7. Performance

- [ ] Are response times / frame times measured?
- [ ] Is startup time measured?
- [ ] Is memory usage profiled?
- [ ] Are there performance regressions tracked in CI?
- [ ] Is the critical path profiled?
- [ ] Are there performance budgets? (e.g., < 200ms p95, < 100MB RAM)

---

## 8. Accessibility

<!-- CUSTOMIZE: Delete this section if your project has no UI. -->

- [ ] Screen reader support? (ARIA labels, semantic HTML)
- [ ] Keyboard navigation? (Tab order, focus management)
- [ ] Color contrast verified? (WCAG AA minimum)
- [ ] Reduced motion mode? (Respects `prefers-reduced-motion`)
- [ ] High contrast mode?
- [ ] Touch targets 44px+? (Mobile accessibility)

---

## 9. Internationalization

<!-- CUSTOMIZE: Delete this section if your project does not need i18n. -->

- [ ] i18n infrastructure in place? (String extraction, translation files)
- [ ] RTL (right-to-left) support?
- [ ] Actual translations shipped? (How many languages?)
- [ ] Cultural sensitivity considered? (Date formats, number formats, symbols)
- [ ] Unicode handling correct? (Emoji, multi-byte characters, combining marks)

---

## 10. Security & Privacy

- [ ] No tracking without consent?
- [ ] User data minimized? (Collect only what is needed)
- [ ] Data encrypted at rest and in transit?
- [ ] Authentication and authorization correct?
- [ ] HTTPS enforced?
- [ ] CSP headers configured? (Content Security Policy)
- [ ] No secrets in codebase? (Env vars, secret managers)
- [ ] Dependency vulnerabilities scanned? (`npm audit`, `cargo audit`, etc.)
- [ ] OWASP Top 10 addressed?

---

## 11. Knowledge Management

- [ ] Are external sources/algorithms attributed? (Contributors, licenses)
- [ ] Is domain knowledge documented? (Not just in people's heads)
- [ ] Are learning logs maintained? (Mistakes, discoveries, decisions)
- [ ] Is there a research pipeline? (Papers, books, references tracked)
- [ ] Are knowledge gaps identified and prioritized?

---

## 12. Project-Specific

<!-- CUSTOMIZE: Replace this section entirely with checks specific to your project. -->

```
{CUSTOM_SECTION}
```

**Examples of project-specific checks**:

- **E-commerce**: Are payment flows tested end-to-end? Is PCI compliance verified?
- **Healthcare**: Is HIPAA compliance verified? Are audit logs complete?
- **Real-time app**: Are WebSocket reconnection strategies tested? Is data consistency guaranteed?
- **Mobile app**: Is offline mode tested? Are push notifications working?
- **Data pipeline**: Is data quality monitored? Are schema migrations tested?
- **Game/Interactive**: Is frame rate stable? Are input latency budgets met?

---

## How to Use This Checklist

Read this BEFORE every review cycle. For each unchecked item:

1. **Known gap with a plan?** Note it, no immediate action needed.
2. **NEW gap nobody flagged?** Flag it to the team lead or project owner.
3. **Something a team member or agent should address?** Create a task or ticket.
4. **Architectural decision needed?** Draft an ADR or update the roadmap.

**The point of this checklist is that the team lead should NEVER be surprised by an industry-standard concern.** If someone else brings up something on this list that was not already tracked, the lead missed it.
