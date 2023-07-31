// Filepaths
pub const ENEMY_SPEC_DIRECTORY: &str = "assets/specs/enemies/";
pub const GENE_SPEC_DIRECTORY: &str = "assets/specs/genes/";

// Game Parameters
pub const STARTING_PLAYER_ENERGY: u8 = 3;
pub const STARTING_PLAYER_HEALTH: u8 = 100;
pub const ENERGY_COST_TO_RUN_AWAY: u8 = 5;

// UI
pub const PLAYER_WINDOW_SIZE: (f32, f32) = (300.0, 200.0);
pub const ENEMY_WINDOW_SIZE: (f32, f32) = (150.0, 100.0);
pub const OPTION_CARD_SIZE: (f32, f32) = (100., 200.);
pub const DEFAULT_FONT_SIZE: f32 = 28.;
pub const LOG_WINDOW_SIZE: (f32, f32) = (300.0, 300.0);
pub const LOG_TEXT_SIZE: f32 = 20.;
pub const MAX_ACTION_BUTTONS: usize = 4;

// Map
pub const MAP_FLOOR_SIZE: (f32, f32) = (700., 400.);
pub const N_ROOMS_PER_FLOOR: usize = 12;
pub const MIN_ROOM_SIZE: f32 = 20.;
pub const ROOM_TYPE_RECT_SIZE: f32 = 10.;
pub const PLAYER_RECT_ON_MAP_SIZE: f32 = 20.;
pub const MAP_WALLS_VERTICAL_PROPORTION: f64 = 0.5;
pub const MAX_MAP_GENERATION_ITERATIONS: usize = 100;
pub const WALL_WIDTH: f32 = 1.;
pub const DOOR_SIZE: (f32, f32) = (5., 5.);
pub const EMPTY_ROOM_WEIGHT: f32 = 1.0;
pub const COMBAT_ROOM_WEIGHT: f32 = 1.0;

// Colors
pub const BLUEPRINT_BLUE: (f32, f32, f32) = (0.25, 0.25, 0.75);

// Greek Letters
pub const ALPHA_LOWER: char = 'α';
pub const ALPHA_UPPER: char = 'Α';
pub const BETA_LOWER: char = 'β';
pub const BETA_UPPER: char = 'Β';
pub const GAMMA_LOWER: char = 'γ';
pub const GAMMA_UPPER: char = 'Γ';
pub const DELTA_LOWER: char = 'δ';
pub const DELTA_UPPER: char = 'Δ';
pub const EPSILON_LOWER: char = 'ε';
pub const EPSILON_UPPER: char = 'Ε';
pub const ZETA_LOWER: char = 'ζ';
pub const ZETA_UPPER: char = 'Ζ';
pub const ETA_LOWER: char = 'η';
pub const ETA_UPPER: char = 'Η';
pub const THETA_LOWER: char = 'θ';
pub const THETA_UPPER: char = 'Θ';
pub const IOTA_LOWER: char = 'ι';
pub const IOTA_UPPER: char = 'Ι';
pub const KAPPA_LOWER: char = 'κ';
pub const KAPPA_UPPER: char = 'Κ';
pub const LAMBDA_LOWER: char = 'λ';
pub const LAMBDA_UPPER: char = 'Λ';
pub const MU_LOWER: char = 'μ';
pub const MU_UPPER: char = 'Μ';
pub const NU_LOWER: char = 'ν';
pub const NU_UPPER: char = 'Ν';
pub const XI_LOWER: char = 'ξ';
pub const XI_UPPER: char = 'Ξ';
pub const OMICRON_LOWER: char = 'ο';
pub const OMICRON_UPPER: char = 'Ο';
pub const PI_LOWER: char = 'π';
pub const PI_UPPER: char = 'Π';
pub const RHO_LOWER: char = 'ρ';
pub const RHO_UPPER: char = 'Ρ';
pub const SIGMA_LOWER: char = 'σ';
pub const SIGMA_UPPER: char = 'Σ';
pub const TAU_LOWER: char = 'τ';
pub const TAU_UPPER: char = 'Τ';
pub const UPSILON_LOWER: char = 'υ';
pub const UPSILON_UPPER: char = 'Υ';
pub const PHI_LOWER: char = 'φ';
pub const PHI_UPPER: char = 'Φ';
pub const CHI_LOWER: char = 'χ';
pub const CHI_UPPER: char = 'Χ';
pub const PSI_LOWER: char = 'ψ';
pub const PSI_UPPER: char = 'Ψ';
pub const OMEGA_LOWER: char = 'ω';
pub const OMEGA_UPPER: char = 'Ω';

pub const GREEK_ALPHABET: &'static [char] = &[
    ALPHA_LOWER,
    BETA_LOWER,
    GAMMA_LOWER,
    DELTA_LOWER,
    EPSILON_LOWER,
    ZETA_LOWER,
    ETA_LOWER,
    THETA_LOWER,
    IOTA_LOWER,
    KAPPA_LOWER,
    LAMBDA_LOWER,
    MU_LOWER,
    NU_LOWER,
    XI_LOWER,
    OMICRON_LOWER,
    PI_LOWER,
    RHO_LOWER,
    SIGMA_LOWER,
    TAU_LOWER,
    UPSILON_LOWER,
    PHI_LOWER,
    CHI_LOWER,
    PSI_LOWER,
    OMEGA_LOWER,
    ALPHA_UPPER,
    BETA_UPPER,
    GAMMA_UPPER,
    DELTA_UPPER,
    EPSILON_UPPER,
    ZETA_UPPER,
    ETA_UPPER,
    THETA_UPPER,
    IOTA_UPPER,
    KAPPA_UPPER,
    LAMBDA_UPPER,
    MU_UPPER,
    NU_UPPER,
    XI_UPPER,
    OMICRON_UPPER,
    PI_UPPER,
    RHO_UPPER,
    SIGMA_UPPER,
    TAU_UPPER,
    UPSILON_UPPER,
    PHI_UPPER,
    CHI_UPPER,
    PSI_UPPER,
    OMEGA_UPPER,
];
