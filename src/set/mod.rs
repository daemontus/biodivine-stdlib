
/// Since Rust does not have a Set trait yet (API is not stabilized yet), we use our own
/// trait with as small API as possible. Sets are used in many places, mostly to represents
/// iterable collections of graph vertices or model parametrisations.
pub trait Set<V> where V: Eq {

    /// Returns true if the set is empty.
    fn is_empty(&self) -> bool;

    /// Returns true if the set contains an element which is equal to the given item.
    fn contains(&self, item: &V) -> bool;

    /// Inserts the given item into the set, returns true if item was inserted and
    /// false if not.
    fn insert(&mut self, item: V) -> bool;

}