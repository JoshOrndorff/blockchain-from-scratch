/// Braindump (revise later):
/// 
/// We have a blockchain data structure featuring:
/// 1. A built in addition accumulator state machine
/// 2. A built-in pow consensus mechanism
/// 
/// We also have abstractions over:
/// 1. State Machines
/// 2. Consensus Engines
/// 
/// Let's refactor our blockchain to take advantage of these two abstractions
/// In doing so, we create a blockchain framework