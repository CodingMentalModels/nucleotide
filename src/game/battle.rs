use bevy::prelude::*;

use crate::game::constants::*;
use crate::game::resources::*;

use super::events::BattleActionEvent;
use super::specs::ActivationTiming;
use super::specs::EnemyName;
use super::specs::StatusEffect;
use super::specs::{GeneCommand, TargetType};
use super::ui_state::CharacterUIState;
use super::ui_state::GeneUIState;
use super::ui_state::GenomeUIState;
use super::ui_state::InBattleUIState;
use super::ui_state::SelectBattleRewardUIState;

pub type TargetEntity = Entity;

pub struct BattlePlugin;

impl Plugin for BattlePlugin {
    fn build(&self, app: &mut App) {
        let get_event_handling_system_condition = || {
            in_state(NucleotideState::GeneCommandHandling)
                .or_else(in_state(NucleotideState::StartOfTurn))
                .or_else(in_state(NucleotideState::EndOfTurn))
        };

        app.add_event::<RanAwayEvent>()
            .add_event::<DamageEvent>()
            .add_event::<BlockEvent>()
            .add_event::<GeneProcessingEvent>()
            .add_event::<StatusEffectEvent>()
            .add_event::<BattleActionEvent>()
            .add_systems(
                OnEnter(NucleotideState::InitializingBattle),
                initialize_battle_system,
            )
            .add_systems(
                OnEnter(NucleotideState::CharacterActing),
                character_acting_system,
            )
            .add_systems(
                OnEnter(NucleotideState::AwaitingBattleInput),
                fetch_battle_actions_system,
            )
            .add_systems(
                Update,
                handle_battle_actions_system.run_if(in_state(NucleotideState::AwaitingBattleInput)),
            )
            .add_systems(
                OnEnter(NucleotideState::StartOfTurn),
                handle_start_of_turn_statuses_system,
            )
            .add_systems(OnEnter(NucleotideState::GeneLoading), gene_loading_system)
            .add_systems(
                OnEnter(NucleotideState::GeneCommandHandling),
                handle_gene_commands_system,
            )
            .add_systems(
                Update,
                handle_ran_away_system.run_if(get_event_handling_system_condition()),
            )
            .add_systems(
                Update,
                handle_damage_system.run_if(get_event_handling_system_condition()),
            )
            .add_systems(
                Update,
                update_block_system.run_if(get_event_handling_system_condition()),
            )
            .add_systems(
                Update,
                update_gene_processing_system.run_if(get_event_handling_system_condition()),
            )
            .add_systems(
                Update,
                apply_status_effect_system.run_if(in_state(NucleotideState::GeneCommandHandling)),
            )
            .add_systems(
                OnEnter(NucleotideState::EndOfTurn),
                handle_end_of_turn_statuses_system,
            )
            .add_systems(
                Update,
                finished_handling_gene_system.run_if(in_state(NucleotideState::EndOfTurn)),
            )
            .add_systems(
                OnEnter(NucleotideState::CharacterActing),
                render_character_display_system,
            )
            .add_systems(
                OnEnter(NucleotideState::GeneAnimating),
                render_character_display_system,
            )
            .add_systems(
                OnEnter(NucleotideState::GeneAnimating),
                render_genome_system,
            )
            .add_systems(
                Update,
                finished_animating_gene_system.run_if(in_state(NucleotideState::GeneAnimating)),
            )
            .add_systems(
                OnEnter(NucleotideState::SelectBattleReward),
                clean_up_after_battle,
            );
    }
}

// Run Conditions

// End Run Conditions

// Resources
#[derive(Resource, Clone, Default)]
pub struct LogState(Vec<egui::RichText>);

impl LogState {
    pub fn log_characters_turn(&mut self, character_type: CharacterType) {
        let character_color = character_type.to_color();
        self.log(
            egui::RichText::new(format!("{}'s Turn", character_type.to_string()))
                .size(LOG_TEXT_SIZE)
                .color(character_color)
                .underline(),
        );
    }
    pub fn get_messages(&self) -> Vec<egui::RichText> {
        self.0.clone()
    }

    pub fn log_string(&mut self, message: String) {
        self.0.push(
            egui::RichText::new(message)
                .size(LOG_TEXT_SIZE)
                .color(egui::Color32::WHITE),
        );
    }

    pub fn log_string_color(&mut self, message: String, color: egui::Color32) {
        self.0.push(
            egui::RichText::new(message)
                .color(color)
                .size(LOG_TEXT_SIZE),
        );
    }

    pub fn log(&mut self, message: egui::RichText) {
        self.0.push(message);
    }
}

// End Resources

// Events
#[derive(Debug, Clone, Copy, PartialEq, Eq, Event)]
struct RanAwayEvent(Entity);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Event)]
struct DamageEvent(TargetEntity, u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Event)]
struct BlockEvent(TargetEntity, u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Event)]
struct GeneProcessingEvent(TargetEntity, GeneProcessingEventType);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Event)]
struct StatusEffectEvent(TargetEntity, StatusEffect, u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Event)]
enum GeneProcessingEventType {
    Reverse,
}

// End Events

// Systems

fn initialize_battle_system(
    mut commands: Commands,
    enemy_specs: Res<EnemySpecs>,
    gene_specs: Res<GeneSpecs>,
    player: Res<Player>,
    mut enemy_queue: ResMut<EnemyPool>,
    current_state: Res<State<NucleotideState>>,
    mut next_state: ResMut<NextState<NucleotideState>>,
) {
    let enemy = match enemy_queue.pop() {
        Some(enemy) => enemy,
        None => {
            commands.insert_resource(NextState(Some(NucleotideState::Victory)));
            return;
        }
    };

    let mut log = LogState::default();

    let player_entity = instantiate_player(&mut commands, player);
    let enemy_entity = instantiate_enemy(&mut commands, enemy.clone(), gene_specs, enemy_specs);

    commands.insert_resource(CharacterActing(player_entity));
    log.log_characters_turn(CharacterType::Player);
    let character_type_to_entity: Vec<_> = vec![
        (CharacterType::Player, player_entity),
        (CharacterType::Enemy(enemy), enemy_entity),
    ]
    .into_iter()
    .collect();

    commands.insert_resource(log);
    commands.insert_resource(BattleActions(Vec::new()));
    commands.insert_resource(GeneCommandQueue::default());
    commands.insert_resource(CharacterTypeToEntity(character_type_to_entity));

    queue_next_state_if_not_already_queued(
        *current_state.get(),
        &mut next_state,
        NucleotideState::CharacterActing,
    );
}

fn character_acting_system(
    mut character_acting: ResMut<CharacterActing>,
    character_type_to_entity_map: Res<CharacterTypeToEntity>,
    mut query: Query<(Entity, &mut EnergyComponent)>,
    mut remove_statuses_query: Query<(Entity, &mut BlockComponent, &mut StatusEffectComponent)>,
    current_state: Res<State<NucleotideState>>,
    mut next_state: ResMut<NextState<NucleotideState>>,
    mut log: ResMut<LogState>,
) {
    let (acting_entity, mut energy) = query.get_mut(character_acting.0).unwrap();

    if energy.energy_remaining == 0 {
        energy.energy_remaining = energy.starting_energy;
        character_acting.0 = character_type_to_entity_map.get_next(acting_entity);

        let character_type = character_type_to_entity_map.get_character_type(character_acting.0);
        log.log_characters_turn(character_type);
        remove_statuses_query
            .iter_mut()
            .filter(|(entity, _block, _statuses)| entity == &character_acting.0)
            .for_each(|(entity, mut block, mut statuses)| {
                if entity == acting_entity {
                    statuses.end_of_turn_clear();
                } else {
                    block.0 = 0; // TODO: Doesn't handle multiple enemies
                }
            });
    }
    energy.energy_remaining -= 1;

    queue_next_state_if_not_already_queued(
        *current_state.get(),
        &mut next_state,
        NucleotideState::AwaitingBattleInput,
    );
}

fn fetch_battle_actions_system(
    character_acting: Res<CharacterActing>,
    character_type_to_entity_map: Res<CharacterTypeToEntity>,
    status_query: Query<(&StatusEffectComponent, With<PlayerComponent>)>,
    mut battle_actions: ResMut<BattleActions>,
) {
    let not_players_turn = !character_type_to_entity_map.is_player(character_acting.0);
    let is_running = status_query.single().0.contains(&StatusEffect::RunningAway);

    if not_players_turn || is_running {
        battle_actions.0 = vec![BattleActionEvent::Continue];
    } else {
        battle_actions.0 = vec![BattleActionEvent::Continue, BattleActionEvent::RunAway];
    }
}

fn handle_battle_actions_system(
    mut player_query: Query<&mut StatusEffectComponent, With<PlayerComponent>>,
    mut battle_action_event_reader: EventReader<BattleActionEvent>,
    current_state: Res<State<NucleotideState>>,
    mut next_state: ResMut<NextState<NucleotideState>>,
    mut log: ResMut<LogState>,
) {
    for event in battle_action_event_reader.iter() {
        match event {
            BattleActionEvent::Continue => {}
            BattleActionEvent::RunAway => {
                let mut status_effects = player_query.single_mut();
                status_effects.add(StatusEffect::RunningAway, ENERGY_COST_TO_RUN_AWAY);
                log.log_string("Running away!".to_string());
            }
        }
        queue_next_state_if_not_already_queued(
            *current_state.get(),
            &mut next_state,
            NucleotideState::StartOfTurn,
        );
    }
}

fn handle_start_of_turn_statuses_system(
    character_acting: Res<CharacterActing>,
    query: Query<(Entity, &mut StatusEffectComponent, &mut GenomeComponent)>,
    ran_away_event_writer: EventWriter<RanAwayEvent>,
    damage_event_writer: EventWriter<DamageEvent>,
    current_state: Res<State<NucleotideState>>,
    mut next_state: ResMut<NextState<NucleotideState>>,
    mut log_state: ResMut<LogState>,
) {
    handle_statuses(
        character_acting,
        query,
        ran_away_event_writer,
        damage_event_writer,
        ActivationTiming::StartOfTurn,
        &mut log_state,
    );

    queue_next_state_if_not_already_queued(
        *current_state.get(),
        &mut next_state,
        NucleotideState::GeneLoading,
    );
}

fn gene_loading_system(
    character_acting: ResMut<CharacterActing>,
    character_type_to_entity_map: Res<CharacterTypeToEntity>,
    mut gene_command_queue: ResMut<GeneCommandQueue>,
    gene_specs: Res<GeneSpecs>,
    mut query: Query<(Entity, &mut GenomeComponent, &StatusEffectComponent)>,
    current_state: Res<State<NucleotideState>>,
    mut next_state: ResMut<NextState<NucleotideState>>,
    mut log: ResMut<LogState>,
) {
    let (acting_entity, genome, status_effects) = query.get_mut(character_acting.0).unwrap();

    if status_effects.contains(&StatusEffect::RunningAway) {
        log.log_string("Trying to run away.".to_string());
    } else {
        let gene = genome.get_active_gene();

        log.log_string(format!("Expressing {}.", gene));
        let gene_spec = gene_specs
            .0
            .get_spec_from_name(&gene)
            .expect("Gene should exist as a gene spec.");
        let targets = get_targets(
            acting_entity,
            character_type_to_entity_map,
            gene_spec.get_target(),
        );

        gene_command_queue.0.append(
            &mut gene_spec
                .get_gene_commands()
                .iter()
                .filter(|gene_command| match gene_command {
                    GeneCommand::Damage(_) => !status_effects.contains(&StatusEffect::Constricted),
                    _ => true,
                })
                .map(|gene_command| {
                    targets
                        .iter()
                        .map(|target| (gene_command.clone(), target.clone()))
                })
                .flatten()
                .collect(),
        );
    }

    queue_next_state_if_not_already_queued(
        *current_state.get(),
        &mut next_state,
        NucleotideState::GeneCommandHandling,
    );
}

fn handle_gene_commands_system(
    mut gene_command_queue: ResMut<GeneCommandQueue>,
    mut damage_event_writer: EventWriter<DamageEvent>,
    mut block_event_writer: EventWriter<BlockEvent>,
    mut gene_processing_event_writer: EventWriter<GeneProcessingEvent>,
    mut status_effect_event_writer: EventWriter<StatusEffectEvent>,
    mut log_state: ResMut<LogState>,
    current_state: Res<State<NucleotideState>>,
    mut next_state: ResMut<NextState<NucleotideState>>,
) {
    for (gene_command, target_entity) in gene_command_queue.0.iter() {
        match gene_command {
            GeneCommand::Damage(damage) => log_and_send(
                &mut log_state,
                format!("{} damage dealt.", damage),
                &mut damage_event_writer,
                DamageEvent(*target_entity, *damage),
            ),
            GeneCommand::Block(amount) => {
                log_and_send(
                    &mut log_state,
                    format!("{} damage blocked.", amount),
                    &mut block_event_writer,
                    BlockEvent(*target_entity, *amount),
                );
            }
            GeneCommand::ReverseGeneProcessing => log_and_send(
                &mut log_state,
                "Gene processing reversed.".to_string(),
                &mut gene_processing_event_writer,
                GeneProcessingEvent(*target_entity, GeneProcessingEventType::Reverse),
            ),
            GeneCommand::Status(effect, n_stacks) => log_and_send(
                &mut log_state,
                format!("{} stacks of {} applied.", n_stacks, effect.to_string()),
                &mut status_effect_event_writer,
                StatusEffectEvent(*target_entity, *effect, *n_stacks),
            ),
            _ => panic!("Unimplemented Gene Command!"),
        }
    }

    gene_command_queue.0.clear();

    queue_next_state_if_not_already_queued(
        *current_state.get(),
        &mut next_state,
        NucleotideState::EndOfTurn,
    );
}

fn handle_ran_away_system(
    mut ran_away_event_reader: EventReader<RanAwayEvent>,
    character_type_to_entity_map: Res<CharacterTypeToEntity>,
    current_state: Res<State<NucleotideState>>,
    mut next_state: ResMut<NextState<NucleotideState>>,
    mut battle_reward_ui_state: ResMut<SelectBattleRewardUIState>,
) {
    for entity in ran_away_event_reader.iter() {
        if character_type_to_entity_map.is_player(entity.0) {
            *battle_reward_ui_state = SelectBattleRewardUIState::after_running_away();
            force_next_state(
                *current_state.get(),
                &mut next_state,
                NucleotideState::SelectBattleReward,
            );
        }
    }
}

fn handle_damage_system(
    mut query: Query<(Entity, &mut HealthComponent, &mut BlockComponent)>,
    mut damage_event_reader: EventReader<DamageEvent>,
    character_type_to_entity: Res<CharacterTypeToEntity>,
    current_state: Res<State<NucleotideState>>,
    mut next_state: ResMut<NextState<NucleotideState>>,
    mut battle_reward_ui_state: ResMut<SelectBattleRewardUIState>,
) {
    for damage_event in damage_event_reader.iter() {
        if let Ok((entity, mut health, mut block)) = query.get_mut(damage_event.0) {
            let damage = damage_event.1.saturating_sub(block.0);
            block.0 = block.0.saturating_sub(damage_event.1);
            health.0 = health.0.saturating_sub(damage);

            if health.0 == 0 {
                match character_type_to_entity.get_character_type(entity) {
                    CharacterType::Player => force_next_state(
                        *current_state.get(),
                        &mut next_state,
                        NucleotideState::GameOver,
                    ),
                    CharacterType::Enemy(_) => {
                        *battle_reward_ui_state =
                            SelectBattleRewardUIState::after_defeating_enemy();
                        force_next_state(
                            // TODO: This doesn't handle multiple enemies at all -- if one dies, battle
                            // over
                            *current_state.get(),
                            &mut next_state,
                            NucleotideState::SelectBattleReward,
                        );
                    }
                }
            }
        }
    }
}

fn update_block_system(
    mut query: Query<(Entity, &mut BlockComponent)>,
    mut block_event_reader: EventReader<BlockEvent>,
) {
    for block_event in block_event_reader.iter() {
        if let Ok((_, mut block)) = query.get_mut(block_event.0) {
            block.0 = block.0.saturating_add(block_event.1);
        }
    }
}

fn update_gene_processing_system(
    mut query: Query<(Entity, &mut GenomeComponent)>,
    mut gene_processing_event_reader: EventReader<GeneProcessingEvent>,
) {
    for gene_processing_event in gene_processing_event_reader.iter() {
        if let Ok((_, mut genome)) = query.get_mut(gene_processing_event.0) {
            match gene_processing_event.1 {
                GeneProcessingEventType::Reverse => {
                    genome.reverse_processing_order();
                }
            }
        }
    }
}

fn apply_status_effect_system(
    mut query: Query<(Entity, &mut StatusEffectComponent)>,
    mut status_effect_event_reader: EventReader<StatusEffectEvent>,
) {
    for status_effect_event in status_effect_event_reader.iter() {
        if let Ok((entity, mut status_effect)) = query.get_mut(status_effect_event.0) {
            if entity == status_effect_event.0 {
                status_effect.add(status_effect_event.1, status_effect_event.2);
            }
        }
    }
}

fn handle_end_of_turn_statuses_system(
    character_acting: Res<CharacterActing>,
    query: Query<(Entity, &mut StatusEffectComponent, &mut GenomeComponent)>,
    ran_away_event_writer: EventWriter<RanAwayEvent>,
    damage_event_writer: EventWriter<DamageEvent>,
    mut log: ResMut<LogState>,
) {
    handle_statuses(
        character_acting,
        query,
        ran_away_event_writer,
        damage_event_writer,
        ActivationTiming::EndOfTurn,
        &mut log,
    );
}

fn finished_handling_gene_system(
    character_acting: ResMut<CharacterActing>,
    mut query: Query<(&mut GenomeComponent, &StatusEffectComponent)>,
    mut next_state: ResMut<NextState<NucleotideState>>,
    current_state: Res<State<NucleotideState>>,
) {
    let (mut genome, status_effects) = query.get_mut(character_acting.0).unwrap();

    if !status_effects.contains(&StatusEffect::RunningAway) {
        genome.advance_pointer();
    }

    queue_next_state_if_not_already_queued(
        *current_state.get(),
        &mut next_state,
        NucleotideState::GeneAnimating,
    );
}

fn render_character_display_system(
    character_display_query: Query<(
        Entity,
        &HealthComponent,
        &BlockComponent,
        &EnergyComponent,
        &StatusEffectComponent,
        &GenomeComponent,
    )>,
    mut ui_state: ResMut<InBattleUIState>,
    character_type_to_entity_map: Res<CharacterTypeToEntity>,
    gene_specs: Res<GeneSpecs>,
) {
    for (entity, health, block, energy, status_effects, genome) in character_display_query.iter() {
        if character_type_to_entity_map.is_player(entity) {
            ui_state.player_character_state = CharacterUIState::new(
                "Player".to_string(),
                energy.energy_remaining,
                energy.starting_energy,
                health.0,
                block.0,
                status_effects.0.clone(),
                GenomeUIState::from_genome(genome, &gene_specs.0),
            )
        } else if character_type_to_entity_map.is_enemy(entity) {
            ui_state.enemy_character_state = CharacterUIState::new(
                "Enemy".to_string(),
                energy.energy_remaining,
                energy.starting_energy,
                health.0,
                block.0,
                status_effects.0.clone(),
                GenomeUIState::from_genome(genome, &gene_specs.0),
            )
        }
    }
}

fn render_genome_system(
    character_query: Query<(Entity, &GenomeComponent)>,
    mut ui_state: ResMut<InBattleUIState>,
    gene_specs: Res<GeneSpecs>,
    character_type_to_entity_map: Res<CharacterTypeToEntity>,
) {
    for (entity, genome) in character_query.iter() {
        let character_type = character_type_to_entity_map.get_character_type(entity);
        ui_state.update_genome(character_type, genome, &gene_specs.0);
    }
}

fn finished_animating_gene_system(
    current_state: Res<State<NucleotideState>>,
    mut next_state: ResMut<NextState<NucleotideState>>,
) {
    queue_next_state_if_not_already_queued(
        *current_state.get(),
        &mut next_state,
        NucleotideState::CharacterActing,
    );
}

fn clean_up_after_battle(
    mut commands: Commands,
    character_type_to_entity: Res<CharacterTypeToEntity>,
) {
    for entity in character_type_to_entity.get_all() {
        commands.entity(entity).despawn_recursive();
    }
    commands.remove_resource::<CharacterActing>();
    commands.remove_resource::<GeneCommandQueue>();
}
// End Systems

// Components

#[derive(Component, Clone, Copy)]
pub struct PlayerComponent;

#[derive(Component, Clone, Copy)]
pub struct EnemyComponent;

#[derive(Component, Clone)]
pub struct GenomeComponent {
    pub genes: Vec<String>,
    pub pointer: usize,
    pub processing_order: GeneProcessingOrder,
    pub repeat_gene: u8,
}

impl GenomeComponent {
    pub fn new(
        genes: Vec<String>,
        pointer: usize,
        processing_order: GeneProcessingOrder,
        repeat_gene: u8,
    ) -> Self {
        GenomeComponent {
            genes,
            pointer,
            processing_order,
            repeat_gene,
        }
    }

    pub fn instantiate(genes: Vec<String>) -> Self {
        Self::new(genes, 0, GeneProcessingOrder::Forward, 0)
    }

    pub fn add_gene(&mut self, gene: String) {
        self.genes.push(gene);
    }

    pub fn get_gene_ui_states(&self, gene_spec_lookup: &GeneSpecLookup) -> Vec<GeneUIState> {
        self.genes
            .iter()
            .enumerate()
            .map(|(i, gene)| {
                let symbol = gene_spec_lookup
                    .get_symbol_from_name(gene)
                    .expect("All genes should have a valid symbol.");
                GeneUIState::new(
                    symbol,
                    (i == self.get_pointer()),
                    gene_spec_lookup
                        .get_card_from_symbol(symbol)
                        .expect("All genes should have a valid symbol."),
                )
            })
            .collect()
    }

    pub fn get_genes(&self) -> Vec<String> {
        self.genes.clone()
    }

    pub fn get_gene(&self, index: usize) -> Option<String> {
        self.genes.get(index).cloned()
    }

    pub fn get_active_gene(&self) -> String {
        self.genes
            .get(self.pointer)
            .expect(&format!(
                "Genome pointer should always be valid but was {}",
                self.pointer
            ))
            .clone()
    }

    pub fn advance_pointer(&mut self) {
        if self.repeat_gene > 0 {
            self.repeat_gene -= 1;
            return;
        }
        let pointer_delta = match self.processing_order {
            GeneProcessingOrder::Forward => 1,
            GeneProcessingOrder::Reverse => -1,
        };
        self.pointer =
            ((self.pointer as i32 + pointer_delta).rem_euclid(self.genes.len() as i32)) as usize;
    }

    pub fn reverse_processing_order(&mut self) {
        self.processing_order = match self.processing_order {
            GeneProcessingOrder::Forward => GeneProcessingOrder::Reverse,
            GeneProcessingOrder::Reverse => GeneProcessingOrder::Forward,
        }
    }

    pub fn increment_repeat_gene(&mut self) {
        self.repeat_gene += 1;
    }

    pub fn get_pointer(&self) -> usize {
        self.pointer
    }
}

#[derive(Component, Clone, Copy)]
pub struct EnergyComponent {
    pub energy_remaining: u8,
    pub starting_energy: u8,
}

impl EnergyComponent {
    pub fn new(energy_remaining: u8, starting_energy: u8) -> Self {
        EnergyComponent {
            energy_remaining,
            starting_energy,
        }
    }
}

#[derive(Component, Clone, Copy)]
pub struct HealthComponent(pub u8);

#[derive(Component, Clone, Copy)]
pub struct BlockComponent(pub u8);

#[derive(Component, Clone)]
pub struct StatusEffectComponent(pub Vec<(StatusEffect, u8)>);

impl StatusEffectComponent {
    pub fn contains(&self, status_effect: &StatusEffect) -> bool {
        self.0.iter().filter(|(e, _)| e == status_effect).count() > 0
    }

    pub fn add(&mut self, status_effect: StatusEffect, n_stacks: u8) {
        if self.contains(&status_effect) {
            self.0 = self
                .0
                .clone()
                .into_iter()
                .map(|(e, n)| {
                    if e == status_effect {
                        (e, n + n_stacks)
                    } else {
                        (e, n)
                    }
                })
                .collect();
        } else {
            self.0.push((status_effect, n_stacks));
        }
    }

    pub fn clear(&mut self) {
        self.0 = Vec::new();
    }

    pub fn end_of_turn_clear(&mut self) {
        self.0 = self
            .0
            .clone()
            .into_iter()
            .filter(|(s, _)| s.clears_after_turn())
            .collect()
    }
}

// End Components

// Helper Types

#[derive(Clone)]
pub enum GeneProcessingOrder {
    Forward,
    Reverse,
}

// End Helper Types

// Helper Functions

fn instantiate_player(mut commands: &mut Commands, player: Res<Player>) -> Entity {
    commands
        .spawn_empty()
        .insert(PlayerComponent)
        .insert(GenomeComponent::instantiate(player.get_genome()))
        .insert(HealthComponent(player.get_health()))
        .insert(BlockComponent(0))
        .insert(EnergyComponent::new(
            player.get_energy(),
            player.get_energy(),
        ))
        .insert(StatusEffectComponent(vec![]))
        .id()
}

fn instantiate_enemy(
    commands: &mut Commands,
    enemy_name: EnemyName,
    gene_specs: Res<GeneSpecs>,
    enemy_specs: Res<EnemySpecs>,
) -> Entity {
    let enemy_spec = enemy_specs.get(enemy_name);
    let genome = enemy_spec
        .get_genome()
        .iter()
        .map(|s| {
            gene_specs
                .0
                .get_spec_from_name(s)
                .expect(&format!("Gene spec not found: {}", s))
                .get_name()
                .clone()
        })
        .collect();
    commands
        .spawn_empty()
        .insert(EnemyComponent)
        .insert(GenomeComponent::instantiate(genome))
        .insert(HealthComponent(enemy_spec.get_health()))
        .insert(BlockComponent(0))
        .insert(EnergyComponent::new(
            enemy_spec.get_energy(),
            enemy_spec.get_energy(),
        ))
        .insert(StatusEffectComponent(vec![]))
        .id()
}

fn get_targets(
    acting_entity: Entity,
    character_type_to_entity_map: Res<CharacterTypeToEntity>,
    target_type: TargetType,
) -> Vec<Entity> {
    match target_type {
        TargetType::Us => vec![acting_entity],
        TargetType::RandomEnemy | TargetType::AllEnemies => {
            vec![character_type_to_entity_map.get_next(acting_entity)]
        }
        TargetType::Everyone => character_type_to_entity_map.get_all(),
    }
}

fn handle_statuses(
    character_acting: Res<CharacterActing>,
    mut query: Query<(Entity, &mut StatusEffectComponent, &mut GenomeComponent)>,
    mut ran_away_event_writer: EventWriter<RanAwayEvent>,
    mut damage_event_writer: EventWriter<DamageEvent>,
    activation_timing: ActivationTiming,
    mut log: &mut ResMut<LogState>,
) {
    for (entity, mut status_effect, mut genome) in query.iter_mut() {
        status_effect
            .0
            .retain_mut(|(status_effect_type, n_stacks)| {
                if status_effect_type.applies_only_on_turn() && (character_acting.0 != entity) {
                    return true;
                }
                if status_effect_type.get_activation_timing() != activation_timing {
                    return true;
                }
                match status_effect_type {
                    StatusEffect::RunningAway => {
                        *n_stacks -= 1;
                        if n_stacks == &mut 0 {
                            ran_away_event_writer.send(RanAwayEvent(entity))
                        }
                    }
                    StatusEffect::Poison => {
                        log_and_send(
                            &mut log,
                            format!("Poison deals {} damage.", n_stacks),
                            &mut damage_event_writer,
                            DamageEvent(entity, *n_stacks),
                        );
                        *n_stacks -= 1;
                    }
                    StatusEffect::RepeatGene => {
                        genome.increment_repeat_gene();
                        *n_stacks -= 1;
                    }
                    StatusEffect::Weak | StatusEffect::Constricted => {
                        *n_stacks -= 1;
                    }
                    _ => panic!("Unimplemented Status Effect! {:?}", status_effect_type),
                };
                *n_stacks > 0
            });
    }
}

fn queue_next_state_if_not_already_queued(
    current_state: NucleotideState,
    next_state: &mut ResMut<NextState<NucleotideState>>,
    next_state_to_queue: NucleotideState,
) {
    if next_state.0.is_none() {
        println!(
            "Setting state to {:?} (Current state: {:?})",
            next_state_to_queue, current_state
        );
        next_state.0 = Some(next_state_to_queue);
    } else {
        println!(
            "State already queued to {:?}, not setting to {:?} (Current state: {:?})",
            next_state.0.unwrap(),
            next_state_to_queue,
            current_state
        );
    }
}

fn force_next_state(
    current_state: NucleotideState,
    next_state: &mut ResMut<NextState<NucleotideState>>,
    next_state_to_queue: NucleotideState,
) {
    println!(
        "Forcing state to {:?} (Current state: {:?}, Next state was: {:?})",
        next_state_to_queue, current_state, next_state.0
    );
    next_state.0 = Some(next_state_to_queue);
}

fn log_and_send<E: Send + Sync + Event>(
    log_state: &mut LogState,
    message: String,
    writer: &mut EventWriter<E>,
    event: E,
) {
    log_state.log_string(message);
    writer.send(event);
}

// End Helper Functions
