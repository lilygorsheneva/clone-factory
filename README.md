# Clone Factory

A factorio-like where most work gets done by recordings of yourself. 

## Instructions

### Installation
* Install the Rust toolchain for your platform
* Clone this repository 
* Cargo run

### Gameplay
* Collect ore from ore deposits by interacting with the building (U) and picking up the resulting item (T)
* Recorders can be used in the Recording Menu to create a sequence of actions that your clones will perform.
    * Only successful actions are recorded, if you try to perform an action you can't currently do, the turn will not advance and the act will not be recorded.
* Clones will either succeed or fail to do an action.
    * If an action fails, it generates a paradox field at the location (ligher screen area).
    * Paradox is lethal to both clones and yourself (though you are resistant).
    * The placeholder foes 😡 will constantly generate paradox. Watch out! 