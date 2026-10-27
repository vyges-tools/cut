// SPDX-License-Identifier: Apache-2.0
//! The timer's answers the cut is built from, gathered before it is built.

/// The timer's view of the design that `get_blob` reads: one entry per graph vertex.
#[derive(Clone, Debug, Default)]
pub struct Facts {
    /// The vertex's pin: `inst/term`, or a port's name.
    pub pin: Vec<String>,
    /// The instance a pin belongs to, `None` for a port.
    pub inst: Vec<Option<String>>,
    /// `Search::isEndpoint`.
    pub endpoint: Vec<bool>,
    /// `isTopLevelPort`.
    pub port: Vec<bool>,
    /// The resizer's `isRegOutput`: a register clock-to-Q arc ends here.
    pub reg_output: Vec<bool>,
    /// The resizer's `isRegister`: the pin's cell is sequential.
    pub register: Vec<bool>,
    /// `SearchPred0::searchFrom` / `searchTo` (not a constant).
    pub from_ok: Vec<bool>,
    pub to_ok: Vec<bool>,
    /// Edges `(from, to, SearchPredCombLogic::searchThru)`.
    pub edges: Vec<(usize, usize, bool)>,
    pub in_edges: Vec<Vec<usize>>,
    pub out_edges: Vec<Vec<usize>>,
    /// `check_timing -unconstrained_endpoints`: the end points no constraint reaches.
    pub unconstrained: Vec<usize>,
    /// `check_timing` reg_no_clks: register clock pins no clock reaches.
    pub unclocked: Vec<usize>,
    /// Timing mode: `PathExpanded(vertexWorstSlackPath(v, max)).size() / 2`; `None` without a
    /// worst-slack path (unconstrained).
    pub depth: Vec<Option<usize>>,
    /// The BLIF writer's `getArrivalTime` / `getRequiredTime` (rise, fall), in seconds: 0 where
    /// there is no path.
    pub arrival: Vec<[f32; 2]>,
    pub required: Vec<[f32; 2]>,
    /// `Sta::slack(vertex, max)`: the least max slack over the scenes (`INF` without a path).
    pub slack: Vec<f32>,
    /// Per scene (in order), each vertex's `Search::wnsSlack` for the scene's max analysis.
    pub scene_wns: Vec<Vec<f32>>,
    /// `Sta::required(vertex, riseFall, every scene, max)`: the fuzzily least required time of the
    /// vertex's max paths over the scenes (`INF` without a path).
    pub required_all: Vec<f32>,
}

impl Facts {
    /// `graph_->pinLoadVertex(pin)`: a pin's first vertex (a bidirect pin's driver follows it).
    pub fn load_vertex(&self, pin: &str) -> Option<usize> {
        self.pin.iter().position(|p| p == pin)
    }
}
