use crate::wires;

pub(crate) fn solve(path: &str) -> usize {
    let mut graph = wires::scan(path);
    for _ in 0..3 {
        let bridge = wires::find_bridge(&graph);
        graph.get_mut(&bridge.0).unwrap().remove(&bridge.1);
        graph.get_mut(&bridge.1).unwrap().remove(&bridge.0);
    }
    let gl = wires::bfs_reach(&graph, &graph.keys().next().unwrap());
    gl * (graph.len() - gl)
}
