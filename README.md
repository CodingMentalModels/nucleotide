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


## Implementation Plan

### V0
[x] Bevy functional
[x] IyesLoopless functional
[x] Fonts loaded and displaying on screen
[x] Data driven approach for specifying genes and enemies sketched out

### V1
[x] Basic versions of systems implemented for Battle and Components
[ ] Basic drafting system implemented
[ ] Game Over Screen
[ ] Models loaded and displaying on screen
[ ] 10 basic genes and 3 basic enemies

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

#### Defense
[x] Block
[ ] Camoflauge -- +50% Temporary Dodge
[ ] Run -- 100% Chance to excape the fight, but gain no rewards
[ ] Bioluminescence
[ ] Regenerate
[ ] Mimic -- Perform the last ability that the target performed
[ ] 


#### Transcription
[x] Reverse
[ ] Skip
[ ] Repeat
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

