use roozle as rz;
use roozle::Search;

#[test]
fn test_exact() {
    let content = "the quick brown fox jumps over the lazy dog";
    let buffer = rz::Buffer::from_string(content);

    let pattern = String::from("the");
    let exact = rz::Exact::from_pattern(pattern);

    let mut options = rz::AnalysisOptions::new().with(rz::AnalysisOption::MatchCount);
    options.add(rz::AnalysisOption::MatchIndices);

    let analysis = exact.search(&buffer, options);

    let mc_report = analysis.report::<rz::MatchCountReport>();
    let mi_report = analysis.report::<rz::MatchIndicesReport>();

    if let Some(r) = mc_report {
        let count = r.count;
        assert_eq!(count, 2);
    } else {
        assert!(false)
    }

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
}
