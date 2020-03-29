extern crate petgraph;

#[cfg(feature = "evcxr")]
pub mod evcxr;

#[cfg(feature = "select")]
pub mod select;

use petgraph::data::*;
use petgraph::visit::*;

pub struct StateMachine<'a, G, Input, Action>
where
    G: GraphBase + Data,
{
    state_network: G,
    state: G::NodeId,
    match_inputs: &'a dyn Fn(Input, G::EdgeWeight) -> Option<Action>,
}

fn get_id_for_state<'a, G>(
    network: &'a G,
    state: <G as Data>::NodeWeight,
) -> Option<<G as GraphBase>::NodeId>
where
    G: IntoNodeReferences + GraphBase + DataMap,
    <G as Data>::NodeWeight: PartialEq,
{
    for nr in network.node_references() {
        if *(network.node_weight(nr.id())).unwrap() == state {
            return Option::Some(nr.id());
        }
    }
    return None;
}

impl<'a, G, Input, Action> StateMachine<'a, G, Input, Action>
where
    G: Data + NodeIndexable + IntoNodeReferences + IntoEdges + DataMap + GraphBase,
    <G as GraphBase>::EdgeId: Copy + PartialEq,
    <G as GraphBase>::NodeId: Copy + PartialEq,
    <G as Data>::EdgeWeight: PartialEq + Clone,
    <G as Data>::NodeWeight: PartialEq + Clone,
    Input: Clone,
{
    pub fn next<'c>(&'c mut self, input: Input) -> Option<(Action, G::NodeWeight)> {
        let refs = self.next_refs(input);
        match refs {
            Some((matched_transition, _, state)) => {
                return match self.state_network.node_weight(state) {
                    Some(weight) => Some((matched_transition, weight.clone())),
                    None => None,
                }
            },
            None => None,
        }
    }

    pub fn next_refs<'c>(&'c mut self, input: Input) -> Option<(Action, G::EdgeRef, G::NodeId)> {
        for edge in (&self.state_network).edges(self.state) {
            match (self.match_inputs)(input.clone(), edge.weight().clone()) {
                Some(matched_transition) => {
                    self.state = edge.target();
                    return Some((matched_transition, edge, self.state));
                }
                None => (),
            }
        }
        return None;
    }

    pub fn set_state<'c>(&'c mut self, state: G::NodeWeight) {
        get_id_for_state(&self.state_network, state).map(|id| self.state = id);
    }

    pub fn set_state_id<'c>(&'c mut self, state: G::NodeId) {
        self.state = state;
    }

    pub fn get_state_id<'c>(&'c self) -> G::NodeId {
        self.state
    }

    pub fn new(
        network: G,
        start: <G as Data>::NodeWeight,
        match_inputs: &'a dyn Fn(Input, G::EdgeWeight) -> Option<Action>,
    ) -> Option<StateMachine<'a, G, Input, Action>> {
        get_id_for_state(&network, start).map(|id| StateMachine {
            state_network: network,
            state: id,
            match_inputs: match_inputs,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use petgraph::*;

    #[test]
    fn test_transitions() {
        let mut sn: Graph<&str, u32, petgraph::Directed> = Graph::new();
        let sn_item1 = sn.add_node("a");
        let sn_item2 = sn.add_node("b");
        let sn_item3 = sn.add_node("c");
        let sn_item4 = sn.add_node("d");
        let sn_item5 = sn.add_node("e");
        sn.add_edge(sn_item1, sn_item2, 1);
        sn.add_edge(sn_item1, sn_item3, 2);
        sn.add_edge(sn_item2, sn_item4, 1);
        sn.add_edge(sn_item2, sn_item5, 2);
        sn.add_edge(sn_item5, sn_item1, 2);
        sn.add_edge(sn_item5, sn_item3, 1);
        let mut sm = StateMachine::new(&sn, "a", &|ew1, ew2| {
            if ew1 == ew2 {
                Some(())
            } else {
                None
            }
        })
        .unwrap();
        assert_eq!(sm.next(1), Some(((), "b")));
        assert_eq!(sm.next(1), Some(((), "d")));
        sm.set_state("b");
        assert_eq!(sm.next(2), Some(((), "e")));
        assert_eq!(sm.next(2), Some(((), "a")));
        assert_eq!(sm.next(2), Some(((), "c")));
        assert_eq!(sm.next(2), None);
    }
}
