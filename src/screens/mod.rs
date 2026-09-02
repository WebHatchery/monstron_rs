pub mod breeding;
pub mod combat;
pub mod finale;
pub mod hatchery;
pub mod help;
pub mod menu;
pub mod placeholder;
pub mod save_recovery;
pub mod shop;
pub mod stable;
pub mod tower;
pub mod town;
mod town_layout;
mod town_panels;
pub mod workshop;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AppScreen {
    MainMenu,
    ConfirmNewGame,
    ConfirmSaveReset,
    SaveRecovery,
    Settings,
    Help,
    Town,
    Hatchery,
    Stable,
    Breeding,
    Workshop,
    Shop,
    DungeonPrep,
    Tower,
    Combat,
    Finale,
    EndOfDay,
}
