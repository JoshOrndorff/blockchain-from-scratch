//! At this point we abandon our simple integer adding logic. Instead of replacing
//! it with a new specific thing, we will instead abstract the execution out to an
//! arbitrary state machine. Specifically we will allow the blockchain to reach
//! consensus over any arbitrary state machine as long as it meets the interface
//! designed in p1_state_machine
//! 
//

// When I do this part, make the `child` function (and maybe others) have a type parameter which is the state machine and the consensus engine
// That way those methods are still useful when we are writing the client. And we don't have to take generic or associated types on the Block type.
// Discussion point: What things would be possible or impossible if we made type parameters on the block type instead of on the functions.

