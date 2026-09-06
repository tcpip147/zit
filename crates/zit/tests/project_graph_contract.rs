use std::time::{Duration, Instant};

use zit::project::{CommitGraphMarker, CommitSummary, build_commit_graph};

fn commit(id: &str, parents: &[&str]) -> CommitSummary {
    CommitSummary {
        object_id: id.into(),
        parent_ids: parents.iter().map(|id| (*id).into()).collect(),
        summary: id.into(),
        message_body: String::new(),
        author_name: "author".into(),
        author_email: "author@example.com".into(),
        committer_name: "committer".into(),
        committer_email: "committer@example.com".into(),
        authored_at_seconds: 0,
        authored_at_offset_minutes: 0,
        commit_time_seconds: 0,
        commit_time_offset_minutes: 0,
        commit_lane: CommitGraphMarker::default(),
        input_edges: Vec::new(),
        output_edges: Vec::new(),
        is_head: false,
        branches: Vec::new(),
        remote_branches: Vec::new(),
        reachable_from_current: false,
        reachable_from_local: false,
        reachable_from_remote: false,
    }
}

#[test]
fn linear_history_stays_on_the_initial_lane_and_color() {
    let mut commits = vec![
        commit("c3", &["c2"]),
        commit("c2", &["c1"]),
        commit("c1", &[]),
    ];
    build_commit_graph(&mut commits);

    assert!(commits.iter().all(|commit| commit.commit_lane.lane == 0));
    assert!(commits.iter().all(|commit| commit.commit_lane.color == 0));
    assert_eq!(commits[0].output_edges[0].waiting_commit_id, "c2");
    assert_eq!(commits[1].input_edges[0].from_lane, 0);
    assert_eq!(commits[1].input_edges[0].to_lane, 0);
    assert!(commits[2].output_edges.is_empty());
}

#[test]
fn merge_allocates_and_then_converges_a_parent_lane() {
    let mut commits = vec![
        commit("merge", &["main", "side"]),
        commit("main", &["base"]),
        commit("side", &["base"]),
        commit("base", &[]),
    ];
    build_commit_graph(&mut commits);

    assert_eq!(commits[0].output_edges.len(), 2);
    assert_eq!(commits[0].output_edges[0].to_lane, 0);
    assert_eq!(commits[0].output_edges[1].to_lane, 1);
    assert_eq!(commits[2].commit_lane.lane, 1);
    assert!(
        commits[3]
            .input_edges
            .iter()
            .any(|edge| edge.from_lane != edge.to_lane)
    );
    assert!(commits.iter().all(|commit| {
        commit
            .output_edges
            .windows(2)
            .all(|pair| pair[0].to_lane <= pair[1].to_lane)
    }));
}

#[test]
fn graph_build_is_deterministic_and_handles_many_parents() {
    let input = vec![
        commit("octopus", &["a", "b", "c"]),
        commit("a", &[]),
        commit("b", &[]),
        commit("c", &[]),
    ];
    let mut first = input.clone();
    let mut second = input;
    build_commit_graph(&mut first);
    build_commit_graph(&mut second);
    assert_eq!(first, second);
    assert_eq!(first[0].output_edges.len(), 3);
}

#[test]
fn ten_thousand_descriptors_complete_within_acceptance_budget() {
    let mut commits = (0..10_000)
        .rev()
        .map(|index| {
            if index == 0 {
                commit("0", &[])
            } else {
                commit(&index.to_string(), &[&(index - 1).to_string()])
            }
        })
        .collect::<Vec<_>>();
    let start = Instant::now();
    build_commit_graph(&mut commits);
    assert!(start.elapsed() < Duration::from_secs(2));
}

#[test]
fn graph_domain_and_widget_boundaries_are_kept_separate() {
    let model = include_str!("../src/project/model.rs");
    let repository = include_str!("../src/project/repository.rs");
    let widget = include_str!("../../widget/src/data-grid.rs");
    assert!(!model.contains("gpui"));
    assert!(!repository.contains("DataGrid"));
    for forbidden in ["CommitGraph", "CommitSummary", "RepositoryLoader", "gix::"] {
        assert!(!widget.contains(forbidden));
    }
}
