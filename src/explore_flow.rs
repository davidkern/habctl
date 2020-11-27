// What actually runs:
// 
// fn Flow::run() {
//      x1 = read()
//      x2 = chunk(x1)
//      x3 = parse(x2)
//      display(x3)
// }
//
// How we define it:
//
// flow = read().chunk().parse().display()
// flow.run()
//
struct Sausage { }

pub struct Flow {
    _sausage: Sausage,   // unknown private implementation state and functionaltity
}

impl Flow {
    // Implement a Source
    fn source(self) -> Out {
        Out::new(self)
    }

    // Implement a Sink
    fn sink(self) -> Out {
        Out::new(self)
    }

    // Implement a Transfer
    fn transfer(self) -> Out {
        Out::new(self)
    }
}

pub struct In {
    flow: Flow,
}

impl In {
    /// Constructs an In from a Flow
    pub fn new(flow: Flow) -> Self {
        Self { flow }
    }

    /// Consumes the In and returns its Flow
    pub fn flow(self) -> Flow {
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

pub struct Out {
    flow: Flow,
}

impl Out {
    /// Constructs an Out from a Flow
    pub fn new(flow: Flow) -> Out {
        Out { flow }
    }

    /// Consumes the Out and returns its Flow
    pub fn flow(self) -> Flow {
        self.flow
    }
}

pub fn source(flow: In) -> Out {
    flow.source()
}

pub fn alpha(flow: In) -> Out {
    flow.transfer()
}

pub fn beta(flow: In) -> Out {
    flow.transfer()
}

pub fn sink(flow: In) -> Out {
    flow.sink()
}

pub fn example() {

}

//
//------------------------------ Previous Attempt 3
//


// use std::fmt::Debug;
// use std::marker::PhantomData;

// pub trait Output<O: ?Sized> {
// }

// pub trait Input<I: ?Sized> {
// }

// pub trait Stage<I: ?Sized, O: ?Sized>: Input<I> + Output<O> { }

// // impl<O> Output<O> for fn(&O) -> () {
// //     fn send(&mut self, data: &O) {
// //         self(data);
// //     }
// // }

// fn fake_read_file<O: Output<[u8]>>(output: &mut O) {
//     let data = [1u8, 2, 3, 4];
//     //output.push(&data);
// }

// pub fn example() {

// }

//
//------------------------ Previous Attempt 2
// 

// use std::marker::PhantomData;

// pub struct NoInput { }
// pub struct NoOutput { }

// pub fn read_file(_input: &NoInput, _output: &mut [u8]) { }
// pub fn chunk(_input: &[u8], _output: &mut [u8]) { }
// pub fn parse(_input: &[u8], _output: &mut str) { }
// pub fn display(_input: &[u8], _output: &mut NoOutput) { }

// // pub struct ReadFile { }
// // impl Stage for ReadFile {
// //     type Input = NoInput;
// //     type Output = [u8];
// // }

// pub fn example() {
//     let mut foo: Box<dyn Stage<Input=[u8], Output=[u8]>> = Box::new(chunk);
// //    let f = Flow::from(read_file);
//         // .to(chunk)
//         // .to(parse);
// }

// pub trait Source {
//     type Output;
// }

// // Implement Source trait for any stateless function
// impl<TOutput> Source for fn(&mut TOutput) {
//     type Output = TOutput;
// }

// pub trait Stage {
//     type Input;
//     type Output;
// }

// // Implement Stage trait for any stateless function
// impl<'r, 's, TInput, TOutput> Stage for fn(&'r TInput, &'s mut TOutput) {
//     type Input = TInput;
//     type Output = TOutput;
// }

// pub struct Flow<TInput, TOutput> {
//     input: PhantomData<TInput>,
//     output: PhantomData<TOutput>,
// }

// impl<TInput, TOutput> Flow<TInput, TOutput> {
//     /// A new flow with stage added to the end
//     pub fn to<TStage, TNewOutput>(&self, stage: TStage) -> Flow<TInput, TNewOutput>
//     where
//         TStage: Stage<Input=TOutput, Output=TNewOutput>
//     {
//         Flow {
//             input: PhantomData,
//             output: PhantomData,
//         }
//     }
// }

// impl<TInput, TOutput, TStage> From<TStage> for Flow<TInput, TOutput>
// where
//     TStage: Stage<Input=TInput, Output=TOutput>
// {
//     fn from(stage: TStage) -> Self {
//         Flow {
//             input: PhantomData,
//             output: PhantomData,
//         }
//     }
// }

// impl Flow {
//     pub fn pull_from(_stage: Stage) -> Self {
//         Self { }
//     }

//     pub fn push_from(_stage: Stage) -> Self {
//         Self { }
//     }

//     pub fn and_then(stage: Stage) -> Self {
//         self
//     }

//     pub fn run() {

//     }
// }

// pub struct FileReader {}

// impl FileReader {
//     pub fn new() -> Self {
//         Self { }
//     }
// }

// pub struct Chunk {}

// impl Chunk {
//     pub fn new() -> Self {
//         Self { }
//     }
// }

// pub struct Parse {}

// impl Parse {
//     pub fn new() -> Self {
//         Self { }
//     }
// }

// pub struct Display {}

// impl Display {
//     pub fn new() -> Self {
//         Self { }
//     }
// }

// pub fn example() {
//     let flow = Flow::pull_from(FileReader::new())
//         .and_then(Chunk::new())
//         .and_then(Parse::new())
//         .and_then(Display::new());
    
//     flow.run();
// }

//
//------------------------ Previous Attempt 1
// 

// use std::fmt::Debug;

// pub struct Flow<TData, TSource, TSink>
// where
//     TSource: Source<TData>,
//     TSink: Sink<TData>
// {
//     source: TSource,
//     sink: TSink,
//     buffer: Vec<TData>,
// }

// // pub struct Flow<TData> {
// //     source: Box<dyn Source<TData>>,
// //     sink: Box<dyn Sink<TData>>,
// //     buffer: Vec<TData>,
// // }

// // impl<TData> Flow<TData> {
// //     pub fn new(source: Box<dyn Source<TData>>, sink: Box<dyn Sink<TData>>) -> Self {
// //         Self {
// //             source,
// //             sink,
// //             buffer: Vec::new(),    
// //         }
// //     }

// //     pub fn run_once(&self) {
// //         self.source.pull(self);
// //     }
// // }

// /// A source of data
// pub trait Source<TData> {
//     /// Pulls data from the source
//     fn pull(&self, flow: &Flow<TData>);
// }

// impl<TData> Source<TData> for &[TData]  {
//     fn pull(&self, flow: &Flow<TData>) {
//         // TODO: keep state somewhere
//         //sink.push(&self[0]);
//     }
// }

// impl Source<()> for () {
//     fn pull(&self, flow: &Flow<()>) {
//         //sink.push(&())
//     }
// }

// /// A sink of data
// pub trait Sink<T> {
//     /// Pushes data to a sink
//     fn push(&self, data: &T);
// }

// pub struct DebugPrint;

// impl<T> Sink<T> for DebugPrint
// where
//     T: Debug
// {
//     fn push(&self, data: &T) {
//         println!("{:?}", data);
//     }
// }

// pub fn run() {
//     let empty_source = ();
//     let empty_flow = Flow::new(
//         Box::new(empty_source),
//         Box::new(DebugPrint { }));
//     empty_flow.run_once();

//     //empty_source.pull(&DebugPrint { });

//     // let source_data = [1u8, 2, 3];
//     // source_data.as_ref().pull(|x| println!("{:?}", x));

//     // let empty_flow = Flow::new(empty_source, DebugPrint { });
// }
