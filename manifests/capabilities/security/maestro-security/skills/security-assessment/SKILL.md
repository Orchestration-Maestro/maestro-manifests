---
name: security-assessment
description: Run one autonomous security assessment from doctor to disclosure.
---

Run one assessment for one target. Check the box first, route the target,
run the pipeline, prove every finding, keep the evidence, then report and
disclose.

1. Doctor: probe the box (runner reachability, model reachability). An
   unhealthy box stops the work before a phase runs.
2. Route: name the target's domains (web, network, binary, source) so the
   pipeline tests the right hypotheses.
3. Assess: run the phases in order — recon, target-model, hypotheses,
   verify, deepen, threat, report. Steering shapes priorities and guidance
   only; the target, the evidence and the verdict rules stay fixed.
4. Validate: run the gate before any report is shown.
5. Evidence: read the ledger the run wrote, never a summary from memory.
6. Report: render the report page from the evidence.
7. Disclose: hand over the disclosure package and the SARIF export.

A run starts only from the user's request. Evidence is immutable; a finding
without evidence is unknown, and one its counterevidence kills is refuted.
