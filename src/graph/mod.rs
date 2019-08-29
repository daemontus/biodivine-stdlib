use std::hash::Hash;
use crate::set::Set;

/// Vertex is a marker trait for a struct that can be used as a graph vertex. Currently,
/// such struct only needs to be cloneable and hashable. In the future, maybe extra
/// restrictions can be added...
pub trait Vertex : Eq + Clone + Hash {}

/// Evolution operator trait represents part of the di-graph structure -- specifically
/// the forward edges of the graph. The reason evolution operator exists is the fact
/// that not all di-graphs have to be finite or have a known state space. Using
/// evolution operator, you can define a graph that you can explore, but you can't
/// simply iterate over its structure (vertices edges).
pub trait EvolutionOperator<V> where V: Vertex {
    type SuccessorIterator : Iterator<Item=V>;

    fn next_step(&self, source: &V) -> Self::SuccessorIterator;
}

/// Inverse evolution operator is exactly the opposite of evolution operator -- instead
/// of describing forward edges of the graph, inverse evolution operator allows inverse
/// exploration.
pub trait InverseEvolutionOperator<V> where V: Vertex {
    type PredecessorIterator : Iterator<Item=V>;

    fn prev_step(&self, source: &V) -> Self::PredecessorIterator;
}

pub trait VertexSet<V> : Set<V> where V: Vertex {}