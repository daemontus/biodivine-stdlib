/*
    This is Biodivine "standard library", which means it contains the most common
    definitions and algorithms which are used by other Biodivine modules.

    Mainly, the module contains definitions of
     - graph, directed graph, transition system, Kripke structure
     - On-the-fly variants of these structures. In this context, on-the-fly specifies
     that the structure is never stored in memory except for some exponentially smaller
     representation. An on-the-fly generator can be augmented with caching/pre-computation
     which is not necessarily exponentially smaller (for example, instead of keeping quadratically
     long list of edges, we only remember some linearly many pre-computed values for a specific
     node.
     - Symbolic variants of these structures. In this context, symbolic specifies
     that the structure is typically much smaller, but can be at the worst case linear in size.
*/

/*
Vocabulary: When talking about graphs and transition systems, one often talks about the same
objects using different names. Specifically vertices, nodes and states are in general the
same entities. Similarly, edges and transitions can refer to equivalent objects.

We try to adhere to the graph terminology as closely as possible, using vertex and edge where possible.

*/

mod graph;
mod set;

use std::collections::{HashSet, HashMap};
use std::hash::Hash;
use std::vec::IntoIter;

pub struct HashVertexSet<V: Hash + Eq> {
    set: HashSet<V>
}

impl <V: Hash + Eq> VertexSet<V> for HashVertexSet<V> {

    fn contains(&self, vertex: &V) -> bool {
        return self.set.contains(vertex);
    }

    fn is_empty(&self) -> bool {
        return self.set.is_empty();
    }

    fn insert(&mut self, vertex: V) -> bool {
        return self.set.insert(vertex);
    }
}

pub struct SimpleGraph {
    vertices: HashSet<String>,
    successors: HashMap<String, Vec<String>>,
    predecessors: HashMap<String, Vec<String>>
}

impl EvolutionOperator<String> for SimpleGraph {
    type Iterator = IntoIter<String>;

    fn next(&self, source: &String) -> Self::Iterator {
        return self.successors.get(source).unwrap().clone().into_iter()
    }
}

pub struct SimpleGraphAlgorithms;

impl GraphAlgorithms<SimpleGraph, String> for SimpleGraphAlgorithms {
    type Set = HashVertexSet<String>;

    fn new_vertex_set(graph: &SimpleGraph) -> Self::Set {
        return HashVertexSet { set: HashSet::new() }
    }
}

pub trait VertexSet<V> {
    fn contains(&self, vertex: &V) -> bool;
    fn is_empty(&self) -> bool;
    fn insert(&mut self, vertex: V) -> bool;
}

pub trait EvolutionOperator<V> {
    type Iterator : Iterator<Item=V>;

    fn next(&self, source: &V) -> Self::Iterator;
    //fn next_ref(&self, source: &V) -> &Self::Iterator;
}

pub trait GraphAlgorithms<G, V> where V: Clone, G: EvolutionOperator<V> {
    type Set : VertexSet<V>;

    fn new_vertex_set(graph: &G) -> Self::Set;

    fn reachable_states(graph: &G, source: &V) -> Self::Set {
        let mut stack: Vec<G::Iterator> = Vec::new();
        let mut result = Self::new_vertex_set(graph);
        stack.push(graph.next(source));
        result.insert(source.clone());
        while let Some(it) = stack.last_mut() {
            if let Some(t) = it.next() {
                let visited = result.contains(&t);
                if !visited {
                    stack.push(graph.next(&t));
                    result.insert(t);
                }
            } else {
                stack.pop();
            }
        }
        return result;
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut vertices: HashSet<String> = HashSet::new();
        vertices.insert("A".to_string());
        vertices.insert("B".to_string());
        vertices.insert("C".to_string());

        let succ_a = vec!["B".to_string()];
        let succ_b = vec!["C".to_string()];
        let succ_c = vec!["C".to_string()];
        let pred_a = vec![];
        let pred_b = vec!["A".to_string()];
        let pred_c = vec!["C".to_string()];

        let mut successors = HashMap::new();
        successors.insert("A".to_string(), succ_a);
        successors.insert("B".to_string(), succ_b);
        successors.insert("C".to_string(), succ_c);

        let mut predecessors = HashMap::new();
        predecessors.insert("A".to_string(), pred_a);
        predecessors.insert("B".to_string(), pred_b);
        predecessors.insert("C".to_string(), pred_c);

        let graph = SimpleGraph {
            vertices,
            successors,
            predecessors
        };

        let reach_from_a = SimpleGraphAlgorithms::reachable_states(&graph, &"A".to_string());
        let reach_from_b = SimpleGraphAlgorithms::reachable_states(&graph, &"B".to_string());
        let reach_from_c = SimpleGraphAlgorithms::reachable_states(&graph, &"C".to_string());

        assert!(reach_from_a.contains(&"A".to_string()));
        assert!(reach_from_a.contains(&"B".to_string()));
        assert!(reach_from_a.contains(&"C".to_string()));

        assert!(!reach_from_b.contains(&"A".to_string()));
        assert!(reach_from_b.contains(&"B".to_string()));
        assert!(reach_from_b.contains(&"C".to_string()));

        assert!(!reach_from_c.contains(&"A".to_string()));
        assert!(!reach_from_c.contains(&"B".to_string()));
        assert!(reach_from_c.contains(&"C".to_string()));

    }
}
