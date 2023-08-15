# nucleotide

Roguelite autobattler in which player is drafting and rearranging the genetic code of their character and battling it against foes.

Mechanics:
- There are genes which have some sort of effect, e.g.
    - Utility: Skip the next gene, reverse the order of consumption, etc.
    - Offense: Attack, Poison
    - Defense: Armor, Evasion
    - Status
    ...
- Certain genes when used together on the same turn will combine to do extra powerful things (these are the "Side Effects")
- At the beginning of the game, the gene pool is mapped to greek letters, so you don't know which genes are mapped to which letters
- Your hero starts with several genes always and a few random random ones
- In each battle, you and the foe take turns reading off N genes and doing their action until one of you wins
- After each battle, you can add new genes, remove old genes, or reorder genes in some way

## Game Phases

State Machine that controls Game Phases:
- Loading
    - -> Menu
- Menu
    - -> InBattle
- Paused
    - -> InBattle
- Drafting
    - -> InBattle
- InBattle
    - -> Paused
    - -> GameOver
    - -> Victory
    - -> Drafting
- GameOver
- Victory


## ECS

### Entity Types
- Player (Singleton)
    - InputHandling
    - Control
    - Health
    - Genome
    - Genome Pointer
    - Energy
    - Status
- Enemy
    - AI
    - Control
    - Health
    - Genome
    - Genome Pointer
    - Energy
    - Status
- Gene
    - Symbol
    - GeneSpec
        - Name
        - GeneType
        - Text
        - Target
        - GeneCommands


### Systems
- Loading
    - gene_pool_initialiation_system
    - spec_loading_system
- InBattle
    - input_handling_system
    - ai_system
    - control_system
    - gene_expression_system - Convert GeneCommands to Events for various systems
    - HealSystem
    - DamageSystem
    - ...
- InBattle -> Drafting
    - drafting_options_generation_system
- Drafting
    - drafting_system
- Drafting -> InBattle
    - enemy_selection_system


### Components
- InputHandlingComponent
- AIComponent
- ControlComponent
- HealthComponent(u8)
- EnergyComponent(u8)
- StatusComponent(StructOfStatuses)

### Resources
- Time
- Input
- GamePhase
- Gene Pool
- Enemy Pool
- GeneCommands
    - Damage(u8)
    - Heal(u8)
    - Status(StatusEffect, u8)
    - JumpForwardNGenes(u8)
    - ReverseGeneProcessing
    - RepeatGeneNTimes(u8)
    - GainEnergy(u8)


## Genes

### Offensive
- Sting
- Tail Swipe
- Stomp
- Bite
- Claw
- Trample



### Defensive
- Block
- Camoflauge
- Thorns
- Regeneration
- 

### Gene Processing
- Stop Codon
- Repeat Codon
- Reverse Codon
- Goto Codon

### Meta

#### Drafting
After each fight, we have the option of:
- Choosing a gene from our fallen foe
    -> Show the genes from our fallen foe (with hovertips if we know them)
    -> Allow us to click one to be added to the end of our genome
    -> Allow user to click through to continue
- Moving one gene in our genome
    -> Show our genome with hovertips
    -> Allow us to click one, which gets highlighted
    -> On hovering over a different gene, show the resulting potential move
    -> On click, move it
    -> Also can cancel by pressing escape or a cancel button
    -> Allow user to click through to continue
- Swapping two genes in our genome
    -> Show our genome with hovertips
    -> Allow us to click one, which gets highlighted
    -> On hovering over a different gene, show the resulting potential swap
    -> On click, swap them
    -> Also can cancel by pressing escape or a cancel button
    -> Allow user to click through to continue
- Researching a symbol
    -> Show all genes with the ones we already know grayed out
    -> On click, show the hovertext for it
    -> Allow user to click through to continue

### Map

As the game progresses, you progress through several (3?) floors of a building, e.g.
- Penthouse Lab
- Lobby
- Basement

The goal is to escape from the building, via the basement.  Each floor has a (procedurally generated?) plan.

Between fights, you choose a new room to enter which could have:
- The exit stairs
- An event -- npcs, stores, etc.
- A fight

After N rooms traversed, the swat team arrives and recaptures you if you haven't exited yet.

#### Modelling

- Will need a graph (network) representation to know which rooms are adjacent to other ones
    - # of rooms total
    - # of each type of room
        - Entrance
        - Exit
        - Battles
        - Events
        - Stores
        - Leftover rooms are empty
    - Minimum distances?  Probably not
- Will need a cartesian representation to know how to render
    - Where each wall, door, etc. is
    - Which rooms are which
    - Should look like a floor plan
        - It should have walls, doors
        - Walls should typically be straight or at right angles
        - Doors should be in reasonable locations and there shouldn't be redundancies
        - Graph should bias towards completeness rather than sparseness

Which to start with to generate the other one?
Which has more constraints?  Probably the cartesian representation.

Key question: Given an adjacency graph, can I subdivide a rectangle to achieve that graph?
- In general, no.  It takes a 3d space to embed a graph.

There's a global vs. local tradeoff here:
- Globally, we want to satisfy some adjacency graph
- Locally, we want things to look like a building plan and not like a random cave

Algorithm:
1. Construct floor plan
    1. Do N iterations of pick a point and an orientation (up/down or left/right) and build a wall until it hits something
    2. Derive the rectangles from the walls
        1. Observation: As each line segment is created, it turns exactly one rectangle into two rectangles
    3. Mark adjacent rooms as adjacent in the AdjacencyGraph
        1. Observation: At the intersection points of the line segment with walls are up to two other adjacencies (usually 1)
        2. Observation: If we check the intersection points in (1) in reverse order that they're applied, we're guaranteed to visit each possible adjacency at least once
    4. Remove some adjacencies for interest
2. Define rooms (or regenerate if it's not possible)
3. (Potentially, generate several instances and evaluate which one is best based on some criteria)
4. Render

#### Rendering

- Blueprint
- Grayed out where you haven't been
- Glowing where you can go
- Indicator where you are
- Number of rooms left before the swat arrives


As a v0, implement via colored rectangles

## Enemies
[ ] An enemy that bluffs you
[ ] 

## Implementation Plan

### V0
[x] Bevy functional
[x] IyesLoopless functional
[x] Fonts loaded and displaying on screen
[x] Data driven approach for specifying genes and enemies sketched out

### V1
[x] Basic versions of systems implemented for Battle and Components
[x] Basic drafting system implemented
[x] Game Over Screen
[ ] Models loaded and displaying on screen
[ ] 10 basic genes and 3 basic enemies
    [ ] Status effects need to wear out at the appropriate time (e.g. block at the beginning of turn)
    [ ] Status effects need to stack (right now you just get multiple status effects with stacks of whatever they were added with)

### V2
[ ] Bosses


## Content

### Status Effects
- Dodge - % to negate damage
- Poison - Deal n damage and decrement by 1
- Critical - % to double damage
- 

### Genes

#### Attacks
[ ] Sting
[ ] Stomp -- Massive Damage
[ ] Slash
[ ] Claw
[ ] Rake
[ ] Echolocate
[ ] Swarm
[ ] Constrict
[ ] Shock
[ ] Gore
[ ] Roll -- Damage that ignores block
[ ] Suicidal Sting

#### Defense
[x] Block
[ ] Camoflauge -- +50% Temporary Dodge
[ ] Fly
[ ] Take Off
[ ] Leap
[ ] Hop
[ ] Run -- 100% Chance to escape the fight, but gain no rewards
[ ] Bioluminescence
[ ] Regenerate
[ ] Mimic -- Perform the last ability that the target performed


#### Transcription
[x] Reverse
[ ] Skip
[x] Repeat
[ ] ATP50 -- Get more energy

#### Potions
- Oxytocin
- Dopamine
- Seratonin
- Cortisol
- Norepinephrine - Fight or Flight -- Choose between bonus damage and bonus evasion
- GABA - Reduce damage
- Acetylcholine - Learning / Memory - Research an undiscovered gene
- Adrenelin - Double your energy

### Bosses


### Enemies


## Graphics / Animation

Hero's monster is caged or otherwise occluded from the player's view but each attack has an animation, with another extra cool one for a critical hit.



