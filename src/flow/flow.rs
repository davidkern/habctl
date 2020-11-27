use super::inflow::In;
use super::outflow::Out;

// Construction Stage
//  Flow                - flow contains neither source nor sink
//  SourceFlow          - flow containing a Source
//  SinkFlow            - flow containing a Sink
//  SourceSinkFlow      - flow containing a Source and a Sink

// Running Stage
//  In                  - flow into stage
//  Out                 - flow out of stage

pub type FlowStage = fn(In) -> Out;

pub struct Flow {
    state: FlowState,
}

impl Flow {
    // Constructs a Flow
    pub fn new() -> Self {
        Self {
            state: FlowState::new(),
        }
    }

    // Adds a stage to the flow
    pub fn to(self, stage: FlowStage) -> Self {
        self
    }

    // Implement a Source
    pub(super) fn source(self) -> Out {
        Out::new(self)
    }

    // Implement a Sink
    pub(super) fn sink(self) -> Out {
        Out::new(self)
    }

    // Implement a Transfer
    pub(super) fn transfer(self) -> Out {
        Out::new(self)
    }
}

struct FlowState {
}

impl FlowState {
    fn new() -> Self {
        Self { }
    }
}

