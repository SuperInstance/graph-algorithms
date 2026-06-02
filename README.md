# graph-algorithms

Graph algorithms in Rust. BFS to max-flow, Dijkstra to spectral methods.

A complete toolkit for working with graphs: constructing directed and undirected weighted graphs, running classical algorithms (Dijkstra, Bellman-Ford, Floyd-Warshall, Kruskal, Prim, Ford-Fulkerson, Hopcroft-Karp), analyzing structure (connected components, bridges, articulation points, biconnected components), computing spectral properties (Laplacian, algebraic connectivity, Fiedler vector, Cheeger constant), and performing network analysis (betweenness centrality, influence propagation, clustering coefficients).

## Install

```toml
[dependencies]
graph-algorithms = "0.1"
```

## Quick Start

```rust
use graph_algorithms::*;

let mut g = Graph::new(5);
g.add_edge(0, 1, 4.0);
g.add_edge(0, 2, 1.0);
g.add_edge(1, 2, 2.0);
g.add_edge(1, 3, 1.0);
g.add_edge(2, 3, 5.0);
g.add_edge(3, 4, 3.0);

let (dist, _) = shortest_path::dijkstra(&g, 0);
let mst_edges = mst::kruskal(&g);
let ac = spectral::algebraic_connectivity(&g);
```

## License

MIT OR Apache-2.0
