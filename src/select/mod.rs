use crate::*;
use petgraph_overlay::*;
use std::hash::*;

pub struct WalkingSelector<'a, G, Input, Action>
where
    G: GraphBase + Data + IntoEdgeReferences,
{
    selection: Selection<G>,
    state_machine: StateMachine<'a, G, Input, Action>,
}


impl<'a, G, Input, Action> WalkingSelector<'a, G, Input, Action>
where
    G: Data + NodeIndexable + IntoNodeReferences + IntoEdges + DataMap + GraphBase,
<G as GraphBase>::EdgeId: Copy + PartialEq + Eq + Hash,
<G as GraphBase>::NodeId: Copy + PartialEq + Eq + Hash,
<G as Data>::EdgeWeight: PartialEq + Clone,
<G as Data>::NodeWeight: PartialEq + Clone,
    Input: Clone,
{
    pub fn new(
        network: G,
        start: <G as Data>::NodeWeight,
        match_inputs: &'a dyn Fn(Input, G::EdgeWeight) -> Option<Action>,
    ) -> Option<WalkingSelector<'a, G, Input, Action>>{
        StateMachine::new(network, start, match_inputs)
            .map(|sm| {
                let state = sm.get_state_id();
                let mut ws = WalkingSelector{
                    state_machine: sm,
                    selection: Selection::new(network),
                };
                ws.selection.select_node(state);
                ws
            })
    }

    pub fn next<'c>(&'c mut self, input: Input) -> Option<(Action, G::NodeWeight)> {
        let refs = self.state_machine.next_refs(input);
        match refs {
            Some((matched_transition, edge_ref, state)) => {
                self.selection.select_edge(edge_ref);
                self.selection.select_node(state);
                match self.state_machine.state_network.node_weight(state) {
                    Some(weight) => Some((matched_transition, weight.clone())),
                    None => None,
                }
            },
            None => None,
        }
    }

    pub fn get_selection<'c>(&'c self) -> Selection<G> {
        self.selection.clone()
    }
}
