use serde_json::Value;
use std::fs;
use std::path::Path;

#[test]
fn release_documents_keep_technical_evidence_distinct_from_human_approval() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let definition = fs::read_to_string(root.join("docs/pc_demo_definition.md"))
        .expect("demo definition should be readable");
    let human = fs::read_to_string(root.join("HUMAN_PC_DEMO_RELEASE_CHECKLIST.md"))
        .expect("human release checklist should be readable");

    assert!(definition.contains("all 80 scene/resolution pairs"));
    assert!(definition.contains("dedicated/shared GPU-memory distributions"));
    assert!(definition.contains("Portable VRAM thresholds, GPU presentation timing"));
    assert!(!definition.contains("diagnostics; GPU memory, stable"));
    assert!(human.contains("have not received human visual approval"));
    assert!(!human.contains("current verification capture is not release-ready"));
}

#[test]
fn catalog_metadata_is_explicitly_internal_and_has_no_inherited_repository_claim() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let raw =
        fs::read_to_string(root.join("game_page.json")).expect("game_page.json should be readable");
    let page: Value = serde_json::from_str(&raw).expect("game_page.json should be valid JSON");

    assert_eq!(page["status"]["text"], "Internal Preview");
    assert!(page.get("repository").is_none());
    assert!(!raw.contains("monstron_rs"));

    let details = page["details"]
        .as_array()
        .expect("catalog details should be an array");
    let platform = details
        .iter()
        .find(|detail| detail["label"] == "Platform")
        .expect("catalog details should name the platform");
    assert!(platform["value"]
        .as_str()
        .is_some_and(|value| value.contains("Windows 10/11 x64 target")));
    assert!(platform["value"]
        .as_str()
        .is_some_and(|value| value.contains("development-only")));

    let controls = page["controls"]
        .as_array()
        .expect("catalog controls should be an array");
    assert_eq!(controls[0]["key"], "Mouse / touch");
    assert!(controls
        .iter()
        .any(|control| control["key"] == "Tower actions"));
    assert!(controls.iter().any(|control| control["key"] == "Combat"));
}

#[test]
fn packaged_player_documents_match_current_local_data_and_known_issue_facts() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let read = |name: &str| {
        fs::read_to_string(root.join(name)).unwrap_or_else(|error| panic!("{name}: {error}"))
    };
    let player_readme = read("PLAYER_README.md");
    let support = read("SUPPORT.md");
    let known_issues = read("KNOWN_ISSUES.md");
    let privacy = read("PRIVACY.md");
    let packager = read("scripts/package_windows_preview.ps1");
    let normalized = |text: &str| text.split_whitespace().collect::<Vec<_>>().join(" ");

    for required in [
        "PLAYER_README.md",
        "SUPPORT.md",
        "KNOWN_ISSUES.md",
        "PRIVACY.md",
        "CREDITS.md",
        "THIRD_PARTY_NOTICES.md",
    ] {
        assert!(packager.contains(required), "packager omits {required}");
    }

    assert!(normalized(&player_readme).contains("EXPORT LOCAL SUMMARY"));
    assert!(normalized(&player_readme)
        .contains("`SUPPORT.md`, `KNOWN_ISSUES.md`, and `PRIVACY.md` beside this README"));
    assert!(!player_readme.contains("docs/KNOWN_ISSUES.md"));
    assert!(!normalized(&player_readme).contains("custom executable icon, rights approval"));
    assert!(support.contains("tester_summary.txt"));
    let privacy = normalized(&privacy);
    assert!(privacy.contains("saved local pacing/balance counters"));
    assert!(privacy.contains("only when the player taps"));
    assert!(privacy.contains("never uploaded automatically"));
    assert!(!known_issues.contains("chroma/magenta art defects"));
    assert!(!known_issues.contains("There is no first-time tutorial"));
    assert!(known_issues.contains("first-expedition and first-egg guides are implemented"));
    assert!(known_issues.contains("final human visual review is pending"));
    assert!(normalized(&known_issues).contains("a custom icon derived from the title art"));
    assert!(normalized(&known_issues).contains("a code signature remain absent"));

    let notices = normalized(&read("THIRD_PARTY_NOTICES.md"));
    assert!(notices.contains("hash-verified copies from each exact upstream tag"));
    assert!(notices.contains("quad-rand 0.2.3"));
    assert!(packager.contains("gilrs 0.10.10"));
    assert!(packager.contains("gilrs-core 0.5.15"));
    assert!(packager.contains("supplementalLicenseHashes"));
}

#[test]
fn sustained_render_soak_keeps_short_or_headless_runs_out_of_release_evidence() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let soak = fs::read_to_string(root.join("scripts/realtime_render_soak.ps1"))
        .expect("sustained render-soak script should be readable");

    assert!(soak.contains("[int]$DurationSeconds = 7200"));
    assert!(soak.contains("$DurationSeconds -le 14400"));
    assert!(soak.contains("$releaseEvidence = $releaseDurationMet -and -not $Headless"));
    assert!(soak.contains("validation_only"));
    assert!(soak.contains("ExecutablePath = $executable"));
    assert!(soak.contains("$manifest.included_files"));
    assert!(soak.contains("$process.elapsed_wall_milliseconds"));
    assert!(soak.contains("$timing.frames -ne $framesPerScene"));
    assert!(soak.contains("$manifest.toolkit_git_commit"));
    assert!(soak.contains("SampleWindowsGpuCounters = $true"));
    assert!(soak.contains("[switch]$RequireGpuCounters"));
    assert!(soak.contains("gpu_counter_status"));
    assert!(soak.contains("p95_gpu_dedicated_bytes"));
    assert!(soak.contains("p95_gpu_shared_bytes"));

    let packager = fs::read_to_string(root.join("scripts/package_windows_preview.ps1"))
        .expect("Windows preview packager should be readable");
    let smoke = fs::read_to_string(root.join("scripts/release_smoke.ps1"))
        .expect("release smoke script should be readable");
    for field in [
        "toolkit_build_id",
        "toolkit_git_commit",
        "toolkit_working_tree_dirty",
    ] {
        assert!(packager.contains(field), "packager omits {field}");
        assert!(smoke.contains(field), "release smoke omits {field}");
    }
    assert!(smoke.contains("$versionInfo.Comments"));
    assert!(smoke.contains("Invoke-Icacls"));
    assert!(smoke.contains("/inheritance:r"));
    assert!(smoke.contains("The relocated launch directory did not deny file creation."));
    assert!(smoke.contains("-SampleWindowsGpuCounters"));
    assert!(smoke.contains("[switch]$RequireGpuCounters"));
    assert!(smoke.contains("gpu_counter_reports_sampled"));
    assert!(smoke.contains("performance_sample_count = $performanceSamples.Count"));
    assert!(smoke.contains("performance_{0}.jsonl"));
    assert!(smoke.contains("worst_p95_cpu_resolution"));

    let defender = fs::read_to_string(root.join("scripts/defender_scan.ps1"))
        .expect("Defender exact-package scanner should be readable");
    assert!(defender.contains("-DisableRemediation"));
    assert!(defender.contains("$manifest.toolkit_git_commit"));
    assert!(defender.contains("$manifest.included_files"));
    assert!(defender.contains("MaxSignatureAgeDays"));
    assert!(defender.contains("host_diagnostic_only"));

    let window_events = fs::read_to_string(root.join("scripts/window_event_smoke.ps1"))
        .expect("Windows event smoke script should be readable");
    assert!(window_events.contains("MoveWindow"));
    assert!(window_events.contains("IsIconic"));
    assert!(window_events.contains("HATCHSPIRE_HEADLESS \"0\""));
    assert!(window_events.contains("resized_client_width"));
    assert!(window_events.contains("host_diagnostic_only"));

    assert!(smoke.contains("technical_capture_passed_human_review_pending"));
    assert!(smoke.contains("human_visual_review_recorded = $false"));
    assert!(smoke.contains("capture_summary.json"));

    let physical_packet = fs::read_to_string(root.join("scripts/create_physical_test_packet.ps1"))
        .expect("physical Windows packet generator should be readable");
    assert!(physical_packet.contains("awaiting_human_evidence"));
    assert!(physical_packet.contains("Get-AuthenticodeSignature"));
    assert!(physical_packet.contains("$manifest.included_files"));
    assert!(physical_packet.contains("$manifest.toolkit_git_commit"));
    assert!(physical_packet.contains("release_approval_granted = $false"));

    let physical_record = fs::read_to_string(root.join("docs/physical_windows_test_record.md"))
        .expect("physical Windows test instructions should be readable");
    assert!(
        physical_record.contains("downloaded from the restricted/unlisted distribution channel")
    );
    assert!(physical_record.contains("at least two clean Windows machines"));
    assert!(physical_record.contains("does not count as a clean-PC result"));

    let human_checklist = fs::read_to_string(root.join("HUMAN_PC_DEMO_RELEASE_CHECKLIST.md"))
        .expect("human release checklist should be readable");
    assert!(!human_checklist.contains("currently points to `monstron_rs`"));
    assert!(human_checklist.contains("stale `monstron_rs` catalog link has been removed"));

    let visual_packet = fs::read_to_string(root.join("scripts/create_visual_review_packet.ps1"))
        .expect("visual-review packet generator should be readable");
    assert!(visual_packet.contains("awaiting_human_visual_review"));
    assert!(visual_packet.contains("captures.Count -ne 80"));
    assert!(visual_packet.contains("Get-FileHash -LiteralPath $source"));
    assert!(visual_packet.contains("refusing to overwrite possible human evidence"));
    assert!(visual_packet.contains("release_approval_granted = $false"));

    let visual_record = fs::read_to_string(root.join("docs/visual_review_packet.md"))
        .expect("visual-review packet instructions should be readable");
    assert!(visual_record.contains("all 76 source PNGs"));
    assert!(visual_record
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .contains("agent judgment do not pass the visual gate"));

    let storefront_packet =
        fs::read_to_string(root.join("scripts/create_storefront_review_packet.ps1"))
            .expect("storefront packet generator should be readable");
    assert!(storefront_packet.contains("awaiting_human_storefront_approval"));
    assert!(storefront_packet.contains("status --porcelain"));
    assert!(storefront_packet.contains("copy HEAD does not match the sealed package commit"));
    assert!(storefront_packet.contains("refusing to overwrite possible human evidence"));
    assert!(storefront_packet.contains("human_storefront_review_recorded = $false"));
    assert!(storefront_packet.contains("upload_authorized = $false"));
    assert!(storefront_packet.contains("release_approval_granted = $false"));

    let store_draft = fs::read_to_string(root.join("docs/itch_store_page_draft.md"))
        .expect("itch store-page draft should be readable");
    assert!(store_draft.contains("{{ZIP_SHA256}}"));
    assert!(store_draft.contains("[[HUMAN APPROVAL REQUIRED:"));
    assert!(store_draft
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .contains("Do not invent CPU, RAM, GPU, storage"));
    assert!(store_draft.contains("do not select HTML5"));

    let release_messages = fs::read_to_string(root.join("docs/demo_release_messages_draft.md"))
        .expect("release-message drafts should be readable");
    assert!(release_messages.contains("Restricted test invitation"));
    assert!(release_messages.contains("Public demo launch post"));
    assert!(release_messages.contains("First-patch update post"));

    let playtest_packet = fs::read_to_string(root.join("scripts/create_playtest_packet.ps1"))
        .expect("independent-playtest packet generator should be readable");
    assert!(playtest_packet.contains("awaiting_independent_playtests"));
    assert!(playtest_packet.contains("distribution_instructions_approved = $false"));
    assert!(playtest_packet.contains("refusing to overwrite possible human evidence"));
    assert!(playtest_packet.contains("Storefront instructions no longer match"));

    let cohort = fs::read_to_string(root.join("scripts/summarize_playtest_cohort.ps1"))
        .expect("playtest cohort summarizer should be readable");
    assert!(cohort.contains("insufficient_evidence"));
    assert!(cohort.contains("targets_not_met"));
    assert!(cohort.contains("targets_met"));
    assert!(cohort.contains("^T[0-9]{2,4}$"));
    assert!(cohort.contains("record fields differ from the anonymous session schema"));
    assert!(cohort.contains("release_approval_granted = $false"));

    let playtest_docs = fs::read_to_string(root.join("docs/independent_playtest_packet.md"))
        .expect("independent-playtest packet instructions should be readable");
    assert!(playtest_docs.contains("at least five eligible independent testers"));
    assert!(playtest_docs.contains("Would play more"));
    assert!(playtest_docs.contains("no forced pass threshold"));
}
