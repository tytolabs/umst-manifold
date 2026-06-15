// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Santhosh Shyamsundar, Santosh Prabhu Shenbagamoorthy — Studio TYTO

#![cfg(feature = "solver-experimental")]

use std::fs;
use std::path::PathBuf;

use burn_ndarray::NdArray;
use umst_manifold::physics::prime_spectral_research::{
    combined_final_verdict, compute_r2_sweep_verdict, compute_r2_verdict, compute_r3_verdict,
    run_full_record, run_spacing_study, InitialCondition, ResearchMode, ResearchParams, SCHEMA,
    VerdictLabel,
};

type B = NdArray<f32>;

fn workspace_outputs(sub: &str) -> PathBuf {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest
        .parent()
        .expect("manifest parent")
        .join("outputs")
        .join("prime-spectral-research")
        .join(sub)
}

#[test]
fn prime_spectral_research_smoke_all_ics() {
    let dev = Default::default();
    for &ic in InitialCondition::all() {
        let record = run_full_record::<B>(ic, 42, 8, 8, ResearchParams::default(), &dev);
        assert_eq!(record.schema, SCHEMA);
        assert_eq!(record.modes.len(), ResearchMode::all().len());
        assert!(record.modes.iter().all(|m| m.final_l2.is_finite()));
    }
}

#[test]
fn prime_spectral_r1_baseline_emit() {
    let dev = Default::default();
    let mut records = Vec::new();
    for &ic in InitialCondition::all() {
        records.push(run_full_record::<B>(
            ic,
            42,
            16,
            16,
            ResearchParams::default(),
            &dev,
        ));
    }
    let r3 = vec![
        run_spacing_study(8, 8, false),
        run_spacing_study(16, 16, false),
    ];
    let payload = serde_json::json!({
        "schema": "prime_spectral_r1_baseline_v1",
        "records": records,
        "r3_spacing": r3,
        "verdict_r2": compute_r2_verdict(&records[0].modes),
        "verdict_r3": compute_r3_verdict(&r3),
        "final": combined_final_verdict(
            &compute_r2_verdict(&records[0].modes),
            &compute_r3_verdict(&r3),
        ),
    });
    let dir = workspace_outputs("r1");
    fs::create_dir_all(&dir).expect("create r1 output dir");
    let path = dir.join("baseline-verification.json");
    fs::write(&path, serde_json::to_string_pretty(&payload).expect("json")).expect("write r1 json");
    assert!(path.is_file(), "baseline emit failed: {}", path.display());
}

#[test]
fn prime_spectral_primary_sweep_emit() {
    let dev = Default::default();
    let grids = [(16_usize, 16_usize), (32, 32)];
    let seeds = [42_u64, 137];
    let params = ResearchParams {
        epsilon: 0.05,
        use_mangoldt: true,
        coprime_stride: None,
    };
    let mut all_records = Vec::new();
    for &(nx, ny) in &grids {
        for &seed in &seeds {
            for &ic in InitialCondition::all() {
                all_records.push(run_full_record::<B>(ic, seed, nx, ny, params.clone(), &dev));
            }
        }
    }

    let r3 = vec![
        run_spacing_study(8, 8, false),
        run_spacing_study(16, 16, false),
        run_spacing_study(8, 8, true),
        run_spacing_study(16, 16, true),
    ];
    let r2 = compute_r2_sweep_verdict(&all_records);
    let r3v = compute_r3_verdict(&r3);
    let final_v = combined_final_verdict(&r2, &r3v);

    let payload = serde_json::json!({
        "schema": "prime_spectral_primary_sweep_v1",
        "run_count": all_records.len(),
        "records": all_records,
        "r3_spacing": r3,
        "verdict_r2": r2,
        "verdict_r3": r3v,
        "final_verdict": final_v,
    });

    let dir = workspace_outputs("r2");
    fs::create_dir_all(&dir).expect("create r2 output dir");
    let path = dir.join("primary-sweep.json");
    fs::write(&path, serde_json::to_string_pretty(&payload).expect("json")).expect("write sweep");
    let prose = format!(
        "# Prime-Spectral primary sweep (computed)\n\n\
         **R2:** {} — {}\n\n\
         **R3:** {} — {}\n\n\
         **FINAL:** {} — {}\n",
        r2.label.as_str(),
        r2.summary,
        r3v.label.as_str(),
        r3v.summary,
        final_v.label.as_str(),
        final_v.summary,
    );
    fs::write(dir.join("primary-sweep-verdict.md"), prose).expect("write verdict md");
    assert!(path.is_file());
}

#[test]
fn prime_spectral_full_sweep_emit_when_env_set() {
    if std::env::var("UMST_PRIME_SPECTRAL_RESEARCH").ok().as_deref() != Some("1") {
        eprintln!("skip full sweep (set UMST_PRIME_SPECTRAL_RESEARCH=1 to enable)");
        return;
    }

    let dev = Default::default();
    let grids = [(16_usize, 16_usize), (32, 32)];
    let seeds = [42_u64, 137];
    let epsilons = [0.01_f32, 0.05, 0.1];
    let mangoldt_flags = [true, false];
    let strides: [Option<u32>; 5] = [None, Some(3), Some(5), Some(7), Some(11)];
    let mut all_records = Vec::new();

    for &(nx, ny) in &grids {
        for &seed in &seeds {
            for &ic in InitialCondition::all() {
                for &epsilon in &epsilons {
                    for &use_mangoldt in &mangoldt_flags {
                        for &coprime_stride in &strides {
                            let params = ResearchParams {
                                epsilon,
                                use_mangoldt,
                                coprime_stride,
                            };
                            all_records.push(run_full_record::<B>(ic, seed, nx, ny, params, &dev));
                        }
                    }
                }
            }
        }
    }

    let r3 = vec![
        run_spacing_study(8, 8, false),
        run_spacing_study(16, 16, false),
        run_spacing_study(8, 8, true),
        run_spacing_study(16, 16, true),
    ];
    let r2 = compute_r2_sweep_verdict(&all_records);
    let r3v = compute_r3_verdict(&r3);
    let final_v = combined_final_verdict(&r2, &r3v);

    let payload = serde_json::json!({
        "schema": SCHEMA,
        "run_count": all_records.len(),
        "records": all_records,
        "r3_spacing": r3,
        "verdict_r2": r2,
        "verdict_r3": r3v,
        "final_verdict": final_v,
    });

    let dir = workspace_outputs("r2");
    fs::create_dir_all(&dir).expect("create r2 output dir");
    let run_id = format!("sweep_{}", chrono_like_stamp());
    let path = dir.join(format!("{run_id}.json"));
    fs::write(&path, serde_json::to_string_pretty(&payload).expect("json")).expect("write sweep");

    let prose = format!(
        "# Prime-Spectral R2/R3 verdict (computed)\n\n\
         **run_id:** {run_id}\n\n\
         **R2:** {} — {}\n\n\
         **R3:** {} — {}\n\n\
         **FINAL:** {} — {}\n",
        r2.label.as_str(),
        r2.summary,
        r3v.label.as_str(),
        r3v.summary,
        final_v.label.as_str(),
        final_v.summary,
    );
    fs::write(dir.join(format!("{run_id}-verdict.md")), prose).expect("write verdict md");

    if final_v.label == VerdictLabel::KillZetaTrack {
        eprintln!("full sweep verdict: kill_zeta_track (expected for null path)");
    }
    assert!(path.is_file());
}

fn chrono_like_stamp() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0);
    format!("{ms}")
}
