// Opuscule State module
//

// extern crate finny;
// use finny::{
//     decl::{BuiltFsm, FsmBuilder},
//     finny_fsm, FsmFactory, FsmResult,
// };

// Tracks the state of the audio player, returns new comprehensive status data structure

// The context is shared between all guards, actions and transitions. Generics are supported here!
// #[derive(Default)]
// pub struct AudioStateContext {
//     stack: Vec<ValidStates>,
//     // include indicators here?
// }

// pub enum ValidStates {
//     StandardQueuePaused,
//     StandardQueuePlaying,
//     Stopped,
// }
// // The states are plain structs.
// #[derive(Default)]
// pub struct StandardQueuePlaying {
//     // n: usize,
// }
// #[derive(Default)]
// pub struct StandardQueuePaused {}

// #[derive(Default)]
// pub struct Stopped {}
// // The events are also plain structs. They can have fields.
// #[derive(Clone)]
// pub struct RequestPlay;
// #[derive(Clone)]
// pub struct RequestPause;

// #[derive(Clone)]
// pub struct RequestStop;

// #[finny_fsm]
// pub fn my_fsm(mut fsm: FsmBuilder<AudioStateFsm, AudioStateContext>) -> BuiltFsm {
//     // The FSM is described using a builder-style API
//     fsm.state::<StandardQueuePlaying>()
//         .on_event::<RequestPause>()
//         .transition_to::<StandardQueuePaused>()
//         .guard(|_ev, ctx| ctx.context.stack.is_empty())
//         .action(|_ev, ctx, state_a, state_b| println!("PLAY to PAUSE"));
//     fsm.state::<StandardQueuePlaying>()
//         .on_event::<RequestStop>()
//         .transition_to::<Stopped>()
//         .guard(|_ev, ctx| ctx.context.stack.is_empty())
//         .action(|_ev, ctx, state_a, state_b| println!("PLAY to STOP"));
//     fsm.state::<StandardQueuePaused>()
//         .on_event::<RequestPlay>()
//         .transition_to::<StandardQueuePlaying>()
//         .guard(|_ev, ctx| ctx.context.stack.is_empty())
//         .action(|_ev, ctx, state_a, state_b| println!("PAUSE to PLAY"));
//     fsm.state::<StandardQueuePaused>()
//         .on_event::<RequestStop>()
//         .transition_to::<Stopped>()
//         .guard(|_ev, ctx| ctx.context.stack.is_empty())
//         .action(|_ev, ctx, state_a, state_b| println!("PAUSE to STOP"));
//     fsm.state::<Stopped>()
//         .on_event::<RequestPlay>()
//         .transition_to::<StandardQueuePlaying>()
//         .guard(|_ev, ctx| ctx.context.stack.is_empty())
//         .action(|_ev, ctx, state_a, state_b| println!("STOP to PLAY"));
//     fsm.initial_state::<Stopped>();
//     fsm.build()
// }

// // pub fn get_fsm() {
// //     let mut as_fsm = AudioStateFsm::new(AudioStateContext::default()).unwrap();

// //     as_fsm.start();
// // }

// // fn main() -> FsmResult<()> {
// //     let mut fsm = AudioStateFsm::new(AudioStateContext::default())?;
// //     assert_eq!(0, fsm.val);
// //     fsm.start()?;
// //     let state_a: &MyStateA = fsm.get_state();
// //     assert_eq!(1, state_a.n);
// //     assert_eq!(1, fsm.val);
// //     fsm.dispatch(MyEvent)?;
// //     assert_eq!(2, fsm.val);
// //     Ok(())
// // }
