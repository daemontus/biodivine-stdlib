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
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
