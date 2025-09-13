use roozle as rz;
use roozle::Search;

#[test]
fn test_basic_exact() {
    let content = "the quick brown fox jumps over the lazy dog";
    let buffer = rz::Buffer::from_string(content);

    let pattern = String::from("the");
    let exact = rz::Exact::from_pattern(pattern);

    let mut config = rz::AnalysisConfig::new()
        .with::<rz::MatchIndicesReport>()
        .with::<rz::MatchFrequencyReport>();
    config.add::<rz::MatchCountReport>();

    let analysis = exact.search(&buffer, config);

    let mc_report = analysis.report::<rz::MatchCountReport>();
    let mf_report = analysis.report::<rz::MatchFrequencyReport>();
    let mi_report = analysis.report::<rz::MatchIndicesReport>();

    if let Some(r) = mi_report {
        let indices = r.indices.get(&String::from("the"));
        if let Some(indices) = indices {
            assert_eq!(*indices, vec![0, 31]);
        } else {
            assert!(false);
        }
    } else {
        assert!(false)
    }

    if let Some(r) = mf_report {
        let frequency = r.frequencies.get(&String::from("the"));
        if let Some(frequency) = frequency {
            assert_eq!(*frequency, 2);
        } else {
            assert!(false);
        }
    } else {
        assert!(false);
    }

    if let Some(r) = mc_report {
        let count = r.count;
        assert_eq!(count, 2);
    } else {
        assert!(false)
    }
}
