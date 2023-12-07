pub trait Edge {
    fn get_ends(&self) -> (usize, usize);

    fn is_directed() -> bool;
}

pub trait Vertex {
    fn get_name(&self) -> &str;

    fn set_name(&mut self);
}

pub trait GraphStorage<V: Vertex, E: Edge> {
    // add Result and Option enums to these
    fn add_vertex(&mut self, vertex: V) -> usize;

    fn get_vertex(&self, vertex_id: usize) -> &V;

    fn get_vertex_mut(&mut self, vertex_id: usize) -> &mut V;

    fn remove_vertex(&mut self);

    fn get_vertices(&self, vertex_ids: &[usize]) -> &[V];

    fn num_vertices(&self) -> usize;

    fn add_edge(&mut self, edge: E) -> usize;

    fn get_edge(&self, edge_id: usize) -> &E;

    fn get_edge_mut(&mut self, edge_id: usize) -> &mut E;

    fn remove_edge(&mut self, edge_id: usize);

    fn get_edges(&self, edge_ids: &[usize]) -> &[E];

    fn get_edges_mut(&mut self, edge_ids: &[usize]) -> &mut [E];

    fn num_edges(&self) -> usize;

    fn get_edges_between(&self, src_vertex_id: usize, dst_vertex_id: usize) -> &[E];

    fn get_edges_between_mut(&mut self, src_vertex_id: usize, dst_vertex_id: usize) -> &mut [E];

    fn get_edges_from(&self, vertex_id: usize) -> &[E];

    fn get_edges_from_mut(&mut self, vertex_id: usize) -> &mut [E];

    fn get_edges_to(&self, vertex_id: usize) -> &[E];

    fn get_edges_to_mut(&mut self, vertex_id: usize) -> &mut [E];

    fn is_directed() -> bool {
        E::is_directed()
    }
}
