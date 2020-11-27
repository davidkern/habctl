use super::flow::Flow;
use super::outflow::Out;

pub struct In {
    flow: Flow,
}

impl In {
    // Construct an In From a Flow
    pub(super) fn new(flow: Flow) -> Self {
        Self { flow }
    }

    /// Consumes the In and returns its Flow
    pub(super) fn flow(self) -> Flow {
        self.flow
    }

    /// Defines a Transfer from In to Out
    pub fn transfer(self) -> Out {
        self.flow.transfer()
    }

    /// Define a Source
    pub fn source(self) -> Out {
        self.flow.source()
    }

    /// Define a Sink
    pub fn sink(self) -> Out {
        self.flow.sink()
    }
}
