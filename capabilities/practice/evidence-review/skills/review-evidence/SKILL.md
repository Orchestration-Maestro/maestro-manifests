---
name: review-evidence
description: Review a proposed answer against its cited source passages and report the evidence truthfully.
---

# Review evidence

Use this procedure only when you review an answer against its sources.
It holds the organization's truthful-evidence rule (SEC-008) as rules a reviewer can check.

1. Read the proposed answer and each cited passage.
2. For every factual claim, locate the passage that supports it.
3. Mark each unsupported claim and say what evidence is missing.
4. Compare each result the answer reports with the output you observed, and mark each mismatch.
5. Return the report below, with a corrected answer that cites its sources.

## Report

Return four sections: `Corrected answer`, `Findings`, `Checks run` and `Not run`.
A review passes a rule when its report shows the evidence in the last column.

| Rule | Required | Checked by |
| --- | --- | --- |
| Cite | Each finding opens with `path:line`, or `no source` when no passage supports the claim. Each claim in `Corrected answer` ends with the `path:line` of its supporting passage. | Every entry in `Findings` opens with one of the two, and every claim in `Corrected answer` ends with a `path:line`. |
| Classify | Each finding is a `Verified failure` or a `Hypothesis`. A `Verified failure` quotes the passage or output that shows it. | Every entry carries one label, and each `Verified failure` carries a quote. |
| Report runs | `Checks run` lists each file you read and each other check, with its result. | The section exists and every entry names a result. |
| Report gaps | `Not run` names each check you could not run, with the reason. It reads `None` when every check ran. | Every entry in `Not run` names a check and gives a reason. `None` appears only when every check the answer claims appears in `Checks run`. |
| Source honesty | You cite only sources you read, and claim no result you did not observe. You compare each result the answer claims with the output you observed. | Every `path:line` in `Findings` and `Corrected answer` names a file listed in `Checks run`. Every result the answer claims appears in `Checks run` or `Findings` beside the output you observed. |
| Private evidence | Credentials and private evidence stay out of the report. | The reviewer reads the report for credentials and private evidence before sharing it. |
