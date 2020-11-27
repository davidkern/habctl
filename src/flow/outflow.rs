use super::flow::Flow;

pub struct Out {
    flow: Flow,
}

impl Out {
    /// Constructs an Out from a Flow
    pub(super) fn new(flow: Flow) -> Self {
        Self { flow }
    }

    /// Consumes the Out and returns its Flow
    pub(super) fn flow(self) -> Flow {
        self.flow
    }
}