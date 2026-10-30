//! Checks the opt-in security content without running an assessment.

use std::collections::BTreeSet;

use maestro_manifests_tests::reading::collapse_whitespace;
use serde_json::json;

use crate::support::{Check, Failures, Role, files, owned, read, read_json};

/// The security capability folder.
const CAPABILITY: &str = "manifests/capabilities/security/maestro-security";
/// The security skill file.
const SKILL: &str =
    "manifests/capabilities/security/maestro-security/skills/security-assessment/SKILL.md";
/// The security instruction file.
const INSTRUCTION: &str = "manifests/capabilities/security/maestro-security/instructions/security-assessment.instructions.md";
/// Each role with the tools its package lets it hold.
const ROLE_TOOLS: [(&str, &[&str]); 5] = [
    (
        "security",
        &[
            "security_doctor",
            "security_route",
            "security_assess",
            "security_workspace",
        ],
    ),
    ("security-probe", &["security_route", "security_assess"]),
    (
        "security-proof",
        &[
            "security_validate",
            "security_evidence",
            "security_disclose",
        ],
    ),
    (
        "security-rebuild",
        &["security_workspace", "security_disclose"],
    ),
    (
        "security-cyber",
        &["security_assess", "security_report_page"],
    ),
];
/// Each role with the terms its prompt keeps.
const CONTRACTS: [(&str, &[&str]); 5] = [
    (
        "security",
        &["one target", "/assess", "doctor", "route", "user"],
    ),
    (
        "security-probe",
        &[
            "technologies",
            "entry points",
            "trust boundaries",
            "Do not test hypotheses",
            "router",
        ],
    ),
    (
        "security-proof",
        &[
            "counterevidence",
            "gate",
            "ledger",
            "NeedsValidation",
            "Rejected",
        ],
    ),
    (
        "security-rebuild",
        &["fix", "verification", "disclosure", "Do not weaken"],
    ),
    (
        "security-cyber",
        &[
            "threat",
            "NeedsValidation",
            "Rejected",
            "coverage",
            "ledger",
        ],
    ),
];

/// Records every way the role `name` departs from its name, description, tools and body.
fn check_role(failures: &mut Failures, name: &str, tools: &[&str]) -> Check {
    let path = format!("{CAPABILITY}/agents/{name}.md");
    let role = Role::parse(&path, &read(&path)?)?;
    let held: Vec<String> = tools
        .iter()
        .map(|tool| format!("maestro-security/{tool}"))
        .collect();

    failures.require_eq(&path, &role.get("name"), &Some(name));
    failures.require(
        role.get("description").is_some_and(|text| !text.is_empty()),
        &path,
        "has no description",
    );
    failures.require_eq(&path, &role.get("tools"), &Some(held.join(", ").as_str()));
    failures.require(!role.body.trim().is_empty(), &path, "has no body");
    Ok(())
}

/// Discovers five native roles, an offered instruction and one skill.
#[test]
fn native_layout_and_role_tool_ceilings() -> Check {
    let mut failures = Failures::default();
    let mut expected: BTreeSet<String> = ROLE_TOOLS
        .iter()
        .map(|(name, _)| format!("agents/{name}.md"))
        .collect();
    expected.extend(owned(&[
        "package.json",
        "instructions/security-assessment.instructions.md",
        "skills/security-assessment/SKILL.md",
    ]));
    let found: BTreeSet<String> = files(CAPABILITY, "")?
        .iter()
        .filter_map(|path| {
            path.strip_prefix(&format!("{CAPABILITY}/"))
                .map(str::to_owned)
        })
        .collect();
    let package = read_json(&format!("{CAPABILITY}/package.json"))?;
    let settings = read_json("manifests/settings/settings.json")?;

    failures.require_eq("capability files", &found, &expected);
    failures.require_eq(
        "package name",
        &package.get("name"),
        &Some(&json!("maestro-security")),
    );
    failures.require_eq(
        "package maestro",
        &package.get("maestro"),
        &Some(&json!({"skills": ["./skills/security-assessment"]})),
    );
    failures.require(
        package.get("version").is_none(),
        "package.json",
        "has a version",
    );
    for (name, tools) in ROLE_TOOLS {
        check_role(&mut failures, name, tools)?;
    }
    failures.require(
        read(INSTRUCTION)?.starts_with("---\napplyTo: \"**\"\n---\n"),
        INSTRUCTION,
        "does not open with applyTo frontmatter",
    );
    failures.matches(
        SKILL,
        &read(SKILL)?,
        r"\A---\nname: security-assessment\ndescription: [^\n]+\n---\n",
    )?;
    failures.require_eq(
        "manifestAreas",
        &settings.get("manifestAreas"),
        &Some(&json!([])),
    );

    failures.finish()
}

/// Keeps explicit initiation, all seven phases and evidence-led delivery.
#[test]
fn assessment_procedure_keeps_lifecycle_and_evidence_gates() -> Check {
    let mut failures = Failures::default();
    let skill = read(SKILL)?;
    let instruction = read(INSTRUCTION)?;
    let requirements = [
        "/assess",
        "wasm extension",
        "MCP stdio",
        "one target",
        "recon, target-model, hypotheses, verify, deepen, threat, report",
        "Confirmed",
        "NeedsValidation",
        "Rejected",
        "SARIF",
        "immutable",
        "counterevidence",
        "blockers",
        "validation plan",
        "SEC-002",
        "SEC-005",
        "SEC-008",
        "SEC-009",
        "unhealthy",
        "priorities and guidance only",
        "target, evidence and verdict rules stay fixed",
    ];
    let tools = [
        "security_doctor",
        "security_route",
        "security_assess",
        "security_validate",
        "security_evidence",
        "security_report_page",
        "security_disclose",
    ];

    for requirement in requirements {
        failures.contains(SKILL, &skill, requirement);
    }
    for tool in tools {
        failures.contains(SKILL, &skill, &format!("`{tool}`"));
    }
    for term in [
        "SEC-002",
        "SEC-005",
        "SEC-008",
        "Check:",
        "/assess",
        "Confirmed",
        "NeedsValidation",
        "Rejected",
    ] {
        failures.contains(INSTRUCTION, &instruction, term);
    }
    for path in files(CAPABILITY, ".md")? {
        let text = read(&path)?;
        failures.lacks(&path, &text, "/maestro-security");
        failures.contains(&path, &text, "Check:");
        failures.contains(&path, &text, "SEC-008");
        failures.lacks(&path, &text, "| SEC-");
    }

    failures.finish()
}

/// Records each role whose prompt lacks a term of its contract, once `reshape` has changed the text.
fn check_responsibilities(reshape: impl Fn(&str) -> String) -> Check {
    let mut failures = Failures::default();
    for (name, terms) in CONTRACTS {
        let path = format!("{CAPABILITY}/agents/{name}.md");
        let role = Role::parse(&path, &reshape(&read(&path)?))?;
        let collapsed = collapse_whitespace(&role.body);
        for term in terms {
            failures.contains(name, &collapsed, term);
        }
    }
    failures.finish()
}

/// Accepts equivalent responsibilities split across Markdown lines.
#[test]
fn specialist_roles_allow_wrapped_responsibilities() -> Check {
    check_responsibilities(|text| text.replace("entry points", "entry\npoints"))
}

/// Keeps reconnaissance separate from proof, repair and reporting.
#[test]
fn specialist_roles_keep_their_responsibilities() -> Check {
    check_responsibilities(str::to_owned)
}
