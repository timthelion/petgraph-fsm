use petgraph::visit::*;
use petgraph_evcxr::draw_graph_with_attr_getters;

use crate::*;

impl<'a, G, Input, Action> StateMachine<'a, G, Input, Action>
where
    G: NodeIndexable
    + IntoEdgeReferences
    + IntoEdges
    + IntoNodeReferences
    + GraphProp
    + GraphBase
    + Data,
    G::NodeId: Copy + PartialEq,
    G::EdgeId: Copy + PartialEq,
    G::EdgeWeight: std::fmt::Display,
    G::NodeWeight: std::fmt::Display,
{
    pub fn draw_evcxr(&self) {
        draw_graph_with_attr_getters(
            self.state_network,
            &[],
            &|_, _| "".to_string(),
            &|_, nr| {
                (if nr.id() == self.state {
                    "shape = circle style = filled fillcolor = red"
                } else {
                    "shape = circle"
                })
                .to_string()
            },
        );
    }
}
