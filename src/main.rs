use eframe::{egui, Frame};
use egui::Id;
use rand::Rng;
use std::collections::HashMap;
use std::collections::HashSet;

mod dialogues;
use dialogues::{create_locations, Location, Dialogue, DialogueOption, PassiveCheck};

struct DialogueApp {
    current_text: String,
    player: Player,
    locations: HashMap<String, Location>, // All locations in the game
    current_location_id: String,          // Current location ID
    current_dialogue_id: Option<String>,  // Current dialogue ID, or None if not in a dialogue
    state: GameState,
    previous_dialogue_id: Option<String>,
}

impl Default for DialogueApp {
    fn default() -> Self {

        Self {
            current_text: "Welcome!".to_string(),
            player: Player {
                tech: 3,
                arts: 3,
                bur: 3, //short for bureaucracy
                und: 3, //short for underworld
                checkmate_mod: 0,
                rocketry_mod: 0,
                pathology_mod: 0,
                civic_engineering_mod: 0,
                apparatchik_mod: 0,
                quota_mod: 0,
                robot_mod: 0,
                dossier_mod: 0,
                delusion_mod: 0,
                arts2_mod: 0,
                arts3_mod: 0,
                arts4_mod: 0,
                high_proof_mod: 0,
                prohibition_mod: 0,
                gizmo_mod: 0,
                oldtime_religion_mod: 0,
                items: vec![],
                xp: 0,
                skill_points: 0,
                dialogues_entered: HashSet::new(),
                flags: HashSet::new(),
            },
            locations: create_locations(),
            current_location_id: "Vestibule".to_string(), // Start in the Vestibule
            current_dialogue_id: Some("Start".to_string()), // Start with the "Start" dialogue
            state: GameState::CharacterCreation, 
            previous_dialogue_id: None,
        }
    }
}


impl eframe::App for DialogueApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        match self.state {
            GameState::CharacterCreation => {
                egui::SidePanel::right(Id::new("character_creation_right_panel")).show(ctx, |ui| {
                    ui.heading("Character Creation");
                    ui.label("Distribute 12 points among your four stats. Each stat must have between 1 and 6 points.");

                    // Sliders for each stat
                    ui.add(egui::Slider::new(&mut self.player.tech, 1..=6).text("Tech"));
                    ui.add(egui::Slider::new(&mut self.player.arts, 1..=6).text("Arts"));
                    ui.add(egui::Slider::new(&mut self.player.bur, 1..=6).text("Bureaucracy"));
                    ui.add(egui::Slider::new(&mut self.player.und, 1..=6).text("Underworld"));

                    // Display remaining points
                    let remaining_points = self.player.remaining_points();
                    ui.label(format!("Remaining points: {}", remaining_points));

                    // Disable the "Start Game" button if the allocation is invalid
                    if self.player.is_valid() {
                        if ui.button("Start Game").clicked() {
                            self.state = GameState::InGame;
                        }
                    } else {
                        ui.label("Ensure all stats are between 1 and 6 points, and the total is exactly 12.");
                    }
                });
            }
            GameState::InGame => {

                let mut style = (*ctx.style()).clone();
                style.spacing.button_padding = egui::Vec2::new(15.0, 5.0);
                ctx.set_style(style);

                // In-game logic here
                egui::SidePanel::right(Id::new("in_game_right_panel")).min_width(400.0).max_width(400.0).show(ctx, |ui| {



                    egui::Frame::default()
                        .inner_margin(egui::style::Margin::same(50.0))  // Apply margin inside the panel
                        .show(ui, |ui| {
                            ui.add_space(50.0);
                            ui.heading("In-Game Dialogue");
                            ui.add_space(20.0);
                    
                            let current_dialogue_id_clone = self.current_dialogue_id.clone();
                            let mut new_dialogue_id = None;
                            let mut new_location_id = None;
                            let mut options_to_remove = vec![];    // Store which options to remove
                            let mut items_to_add = vec![];         // Store items to add to inventory after borrow ends
                            let mut xp_reward = None;              // Store XP reward for later processing
                            let mut passive_checks = vec![];       // Store passive checks to process later

                            let mut flags_to_add = vec![]; // Store flags to add after immutable borrow ends
                    
                            if let Some(current_dialogue_id) = &current_dialogue_id_clone {
                                // First, get the current dialogue immutably
                                if let Some(current_dialogue) = self.get_current_dialogue_from_id(current_dialogue_id) {
                            
                                    // Extract the XP reward
                                    xp_reward = current_dialogue.xp_reward;
                            
                                    // Extract passive checks for later use
                                    passive_checks = current_dialogue.passive_check.clone();

                                    ui.label(egui::RichText::new(format!("{}", current_dialogue.speaker)).strong().size(24.0));

                                    ui.add_space(20.0);

                                    // Display the dialogue
                                    // ui.heading(&current_dialogue.intro);
                                    ui.label(egui::RichText::new(format!("{}", &current_dialogue.intro)).size(20.0));

                                    ui.add_space(20.0);
                            
                                    // Iterate through the dialogue options
                                    for (i, option) in current_dialogue.options.iter().enumerate() {

                                        let is_visible = match &option.visible_when {
                                            Some(flag) => self.player.flags.contains(flag),  // Only visible if the flag is set
                                            None => true,  // Always visible if no flag is required
                                        };

                                        if is_visible {
                                            if ui.button(&option.description).clicked() {
                                                // Clone the item to be picked up to avoid immutable borrow conflicts
                                                if let Some(item) = &option.item_to_pickup {
                                                    items_to_add.push(item.clone());  // Add the item for later processing
                                                    options_to_remove.push(i);        // Mark this option for removal
                                                }

                                                if let Some(flags) = &option.flags {
                                                    for flag in flags {
                                                        flags_to_add.push(flag.clone());  // Collect flags to add later
                                                    }
                                                }
            
                                                // Handle challenges and dialogue transitions
                                                if option.challenge_number.is_some() {
                                                    let success = handle_challenge(&self.player, option);
                                                    if success {
                                                        new_dialogue_id = option.success_dialogue.clone();
                                                    } else {
                                                        new_dialogue_id = option.failure_dialogue.clone();
                                                    }
                                                } else if let Some(success_dialogue) = &option.success_dialogue {
                                                    // Handle transition to a new location or dialogue
                                                    if self.locations.contains_key(success_dialogue) {
                                                        new_location_id = Some(success_dialogue.clone());
                                                        new_dialogue_id = None;
                                                    } else {
                                                        new_dialogue_id = Some(success_dialogue.clone());
                                                    }
                                                }
                                            }
                                        }
                                    } 

                                    // Now that the immutable borrow has ended, we can safely add the flags to the player's flags
                                    for flag in flags_to_add {
                                        self.player.flags.insert(flag);  // Add each flag to the player's flags
                                    }

                                    // Now that the immutable borrow of `current_dialogue` has ended, we can safely mutate `self.player`
                            
                                    // Add items to inventory
                                    for item in items_to_add {
                                        self.player.items.push(item);
                                    }
                            
                                    // Remove options that have been interacted with
                                    if !options_to_remove.is_empty() {
                                        let dialogue = self.get_current_dialogue_from_id_mut(current_dialogue_id).unwrap();
                                        for &index in options_to_remove.iter().rev() {
                                            dialogue.options.remove(index);  // Remove the option in reverse order to avoid shifting indices
                                        }
                                    }
                            
                                    // Award XP if this is the first time entering the dialogue
                                    if !self.player.dialogues_entered.contains(current_dialogue_id) {
                                        if let Some(xp_amount) = xp_reward {
                                            self.player.add_xp(xp_amount);
                                            println!("You gained {} XP!", xp_amount);
                                        }
                                        // Mark the dialogue as entered
                                        self.player.dialogues_entered.insert(current_dialogue_id.clone());
                                    }
            
                                    // Handle passive checks
                                    for passive_check in passive_checks {
                                        let player_skill_value = self.get_player_skill(&passive_check.skill) + 6;  // Assume some modifier for skill checks
                                        let success = player_skill_value >= passive_check.target;
                                
                                        if success {
                                            if let Some(success_text) = &passive_check.success_text {
                                                ui.heading(&format!("{} says:", passive_check.speaker.clone().unwrap_or("Narrator".to_string())));
                                                ui.label(success_text);
                                            }
                                        } else {
                                            if let Some(failure_text) = &passive_check.failure_text {
                                                ui.heading(&format!("{} says:", passive_check.speaker.clone().unwrap_or("Narrator".to_string())));
                                                ui.label(failure_text);
                                            }
                                        }
                                    }
                                }
                            }

                            // Handle dialogue or location transition
                    if let Some(new_id) = new_dialogue_id {
                        self.current_dialogue_id = Some(new_id);
                    }
                    if let Some(new_location) = new_location_id {
                        self.current_location_id = new_location;
                    }

                    ui.add_space(20.0);
            
                    // Add the "View Inventory" button
                    if ui.button("View Inventory").clicked() {
                        self.previous_dialogue_id = self.current_dialogue_id.clone();
                        self.state = GameState::InventoryView;
                    }

                    ui.add_space(20.0);

                    // Add the "Manage Skills" button
                    if ui.button("Manage Skills").clicked() {
                        self.previous_dialogue_id = self.current_dialogue_id.clone();
                        self.state = GameState::SkillManagement;  // Switch to skill management state
                    }

                        });

                });
            }
            
            
            

            GameState::InventoryView => {
                // Display the player's inventory
                egui::CentralPanel::default().show(ctx, |ui| {
                    self.display_inventory(ui);

                    // Add a button to return to the previous dialogue/location
                    if ui.button("Return to Game").clicked() {
                        // Return to the previous dialogue and switch back to InGame state
                        self.current_dialogue_id = self.previous_dialogue_id.clone();
                        self.state = GameState::InGame;
                    }
                });
            }

            GameState::SkillManagement => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.heading("Skill Management");
                    
                    // Display available skill points
                    ui.label(format!("Available Skill Points: {}", self.player.skill_points));
                    
                    // Display current skills and add buttons to increase skills
                    ui.horizontal(|ui| {
                        ui.label(format!("TECH: Checkmate: {}", self.player.checkmate()));
                        if self.player.skill_points > 0 && ui.button("Increase").clicked() {
                            self.player.checkmate_mod += 1;
                            self.player.skill_points -= 1;
                        }
                    });
            
                    ui.horizontal(|ui| {
                        ui.label(format!("TECH: Rocketry: {}", self.player.rocketry()));
                        if self.player.skill_points > 0 && ui.button("Increase").clicked() {
                            self.player.rocketry_mod += 1;
                            self.player.skill_points -= 1;
                        }
                    });
            
                    ui.horizontal(|ui| {
                        ui.label(format!("TECH: Pathology: {}", self.player.pathology()));
                        if self.player.skill_points > 0 && ui.button("Increase").clicked() {
                            self.player.pathology_mod += 1;
                            self.player.skill_points -= 1;
                        }
                    });
            
                    ui.horizontal(|ui| {
                        ui.label(format!("TECH: Civic Engineering: {}", self.player.civic_engineering()));
                        if self.player.skill_points > 0 && ui.button("Increase").clicked() {
                            self.player.civic_engineering_mod += 1;
                            self.player.skill_points -= 1;
                        }
                    });

                    ui.horizontal(|ui| {
                        ui.label(format!("ARTS: Delusion: {}", self.player.delusion()));
                        if self.player.skill_points > 0 && ui.button("Increase").clicked() {
                            self.player.delusion_mod += 1;
                            self.player.skill_points -= 1;
                        }
                    });

                    ui.horizontal(|ui| {
                        ui.label(format!("ARTS: Arts2: {}", self.player.arts2()));
                        if self.player.skill_points > 0 && ui.button("Increase").clicked() {
                            self.player.arts2_mod += 1;
                            self.player.skill_points -= 1;
                        }
                    });

                    ui.horizontal(|ui| {
                        ui.label(format!("ARTS: Arts3: {}", self.player.arts3()));
                        if self.player.skill_points > 0 && ui.button("Increase").clicked() {
                            self.player.arts3_mod += 1;
                            self.player.skill_points -= 1;
                        }
                    });

                    ui.horizontal(|ui| {
                        ui.label(format!("ARTS: Arts4: {}", self.player.arts4()));
                        if self.player.skill_points > 0 && ui.button("Increase").clicked() {
                            self.player.arts4_mod += 1;
                            self.player.skill_points -= 1;
                        }
                    });

                    ui.horizontal(|ui| {
                        ui.label(format!("BUR: Quota: {}", self.player.quota()));
                        if self.player.skill_points > 0 && ui.button("Increase").clicked() {
                            self.player.quota_mod += 1;
                            self.player.skill_points -= 1;
                        }
                    });

                    ui.horizontal(|ui| {
                        ui.label(format!("BUR: Apparatchik: {}", self.player.apparatchik()));
                        if self.player.skill_points > 0 && ui.button("Increase").clicked() {
                            self.player.apparatchik_mod += 1;
                            self.player.skill_points -= 1;
                        }
                    });

                    ui.horizontal(|ui| {
                        ui.label(format!("BUR: Robot: {}", self.player.robot()));
                        if self.player.skill_points > 0 && ui.button("Increase").clicked() {
                            self.player.robot_mod += 1;
                            self.player.skill_points -= 1;
                        }
                    });

                    ui.horizontal(|ui| {
                        ui.label(format!("BUR: Dossier: {}", self.player.dossier()));
                        if self.player.skill_points > 0 && ui.button("Increase").clicked() {
                            self.player.dossier_mod += 1;
                            self.player.skill_points -= 1;
                        }
                    });

                    ui.horizontal(|ui| {
                        ui.label(format!("UND: High Proof: {}", self.player.high_proof()));
                        if self.player.skill_points > 0 && ui.button("Increase").clicked() {
                            self.player.high_proof_mod += 1;
                            self.player.skill_points -= 1;
                        }
                    });

                    ui.horizontal(|ui| {
                        ui.label(format!("UND: Prohibition: {}", self.player.prohibition()));
                        if self.player.skill_points > 0 && ui.button("Increase").clicked() {
                            self.player.prohibition_mod += 1;
                            self.player.skill_points -= 1;
                        }
                    });

                    ui.horizontal(|ui| {
                        ui.label(format!("UND: Gizmo: {}", self.player.gizmo()));
                        if self.player.skill_points > 0 && ui.button("Increase").clicked() {
                            self.player.gizmo_mod += 1;
                            self.player.skill_points -= 1;
                        }
                    });

                    ui.horizontal(|ui| {
                        ui.label(format!("UND: Oldtime Religion: {}", self.player.oldtime_religion()));
                        if self.player.skill_points > 0 && ui.button("Increase").clicked() {
                            self.player.oldtime_religion_mod += 1;
                            self.player.skill_points -= 1;
                        }
                    });
            
                    // Add a button to return to the game
                    if ui.button("Return to Game").clicked() {
                        self.state = GameState::InGame;
                    }
                });
            }
            
        }
    }
}



// Add a helper function to get the dialogue based on the cloned dialogue ID
impl DialogueApp {
    fn get_current_dialogue_from_id(&self, dialogue_id: &String) -> Option<&Dialogue> {
        if let Some(location) = self.locations.get(&self.current_location_id) {
            return location.dialogues.get(dialogue_id);
        }
        None
    }

    fn get_current_dialogue_from_id_mut(&mut self, dialogue_id: &String) -> Option<&mut Dialogue> {
        // Access the current location and retrieve the dialogue mutably
        if let Some(location) = self.locations.get_mut(&self.current_location_id) {
            return location.dialogues.get_mut(dialogue_id);
        }
        None
    }

    // commented because it was never being used, may use it in the future to refactor
    
    // fn get_current_location(&self) -> &Location {
    //     self.locations.get(&self.current_location_id).unwrap()
    // }

    fn display_inventory(&self, ui: &mut egui::Ui) {
        ui.heading("Inventory:");
        if self.player.items.is_empty() {
            ui.label("You have no items.");
        } else {
            for item in &self.player.items {
                ui.label(item);
            }
        }
    }

    // commented because it was never being called. May use in the future to refactor
    // fn display_current_dialogue(&mut self, ui: &mut egui::Ui) {
    //     if let Some(current_dialogue_id) = &self.current_dialogue_id {
    //         if let Some(current_dialogue) = self.get_current_dialogue_from_id(current_dialogue_id) {
    //             ui.heading(&format!("{} says:", current_dialogue.speaker));
    //             ui.heading(&current_dialogue.intro);

    //             let mut new_dialogue_id = None;
    //             for option in current_dialogue.options.iter() {
    //                 if ui.button(&option.description).clicked() {
    //                     new_dialogue_id = option.success_dialogue.clone();
    //                 }
    //             }

    //             if let Some(new_id) = new_dialogue_id {
    //                 self.current_dialogue_id = Some(new_id);
    //             }
    //         }
    //     } else {
    //         ui.label("No dialogue available.");
    //     }
    // }

    fn get_player_skill(&self, skill: &str) -> i32 {
        match skill {
            "checkmate" => self.player.checkmate(),
            "rocketry" => self.player.rocketry(),
            "pathology" => self.player.pathology(),
            "civic engineering" => self.player.civic_engineering(),
            "apparatchik" => self.player.apparatchik(),
            "quota" => self.player.quota(),
            "robot" => self.player.robot(),
            "dossier" => self.player.dossier(),
            "delusion" => self.player.delusion(),
            "arts2" => self.player.arts2(),
            "arts3" => self.player.arts3(),
            "arts4" => self.player.arts4(),
            "high proof" => self.player.high_proof(),
            "prohibition" => self.player.prohibition(),
            "gizmo" => self.player.gizmo(),
            "oldtime religion" => self.player.oldtime_religion(),
            _ => 0, // Default to 0 if the skill doesn't exist
        }
    }
}

// Challenge logic
fn handle_challenge(player: &Player, option: &DialogueOption) -> bool {
    if let Some(challenge_attribute) = &option.challenge_attribute {
        if let Some(challenge_number) = option.challenge_number {
            let attribute_value = match challenge_attribute.as_str() {
                "checkmate" => player.checkmate(),
                "rocketry" => player.rocketry(),
                "pathology" => player.pathology(),
                "civic engineering" => player.civic_engineering(),
                "apparatchik" => player.apparatchik(),
                "quota" => player.quota(),
                "robot" => player.robot(),
                "dossier" => player.dossier(),
                "delusion" => player.delusion(),
                "arts2" => player.arts2(),
                "arts3" => player.arts3(),
                "arts4" => player.arts4(),
                "high proof" => player.high_proof(),
                "prohibition" => player.prohibition(),
                "gizmo" => player.gizmo(),
                "oldtime religion" => player.oldtime_religion(),
                _ => 0,
            };

            let (die1, die2) = roll_dice();
            let roll_sum = die1 + die2;

            println!("You rolled: {} + {} = {}", die1, die2, roll_sum);

            if die1 == 6 && die2 == 6 {
                println!("Double sixes! Automatic success.");
                return true;
            } else if die1 == 1 && die2 == 1 {
                println!("Double ones! Automatic failure.");
                return false;
            }

            let total = roll_sum + attribute_value;
            if total >= challenge_number {
                println!("Success! You needed {}, and you got {}.", challenge_number, total);
                return true;
            } else {
                println!("Failure. You needed {}, but you got {}.", challenge_number, total);
                return false;
            }
        }
    }
    false
}

fn roll_dice() -> (i32, i32) {
    let mut rng = rand::thread_rng();
    (rng.gen_range(1..=6), rng.gen_range(1..=6))
}

struct Player {
    tech: i32,
    arts: i32,
    bur: i32, //short for bureaucracy
    und: i32, //short for underworld
    checkmate_mod: i32,
    rocketry_mod: i32,
    pathology_mod: i32,
    civic_engineering_mod: i32,
    apparatchik_mod: i32,
    quota_mod: i32,
    robot_mod: i32,
    dossier_mod: i32,
    delusion_mod: i32,
    arts2_mod: i32,
    arts3_mod: i32,
    arts4_mod: i32,
    high_proof_mod: i32,
    prohibition_mod: i32,
    gizmo_mod: i32,
    oldtime_religion_mod: i32,
    items: Vec<String>,
    xp: i32,
    skill_points: i32,
    dialogues_entered: HashSet<String>,
    flags: HashSet<String>,
}

impl Player {
    fn checkmate(&self) -> i32 {
        self.tech + self.checkmate_mod
    }

    fn rocketry(&self) -> i32 {
        self.tech + self.rocketry_mod
    }

    fn pathology(&self) -> i32 {
        self.tech + self.pathology_mod
    }

    fn civic_engineering(&self) -> i32 {
        self.tech + self.civic_engineering_mod
    }

    fn apparatchik(&self) -> i32 {
        self.bur + self.apparatchik_mod
    }

    fn quota(&self) -> i32 {
        self.bur + self.quota_mod
    }

    fn robot(&self) -> i32 {
        self.bur + self.robot_mod
    }

    fn dossier(&self) -> i32 {
        self.bur + self.dossier_mod
    }

    fn delusion(&self) -> i32 {
        self.arts + self.delusion_mod
    }

    fn arts2(&self) -> i32 {
        self.arts + self.arts2_mod
    }

    fn arts3(&self) -> i32 {
        self.arts + self.arts3_mod
    }

    fn arts4(&self) -> i32 {
        self.arts + self.arts4_mod
    }

    fn high_proof(&self) -> i32 {
        self.und + self.high_proof_mod
    }

    fn prohibition(&self) -> i32 {
        self.und + self.prohibition_mod
    }

    fn gizmo(&self) -> i32 {
        self.und + self.gizmo_mod
    }

    fn oldtime_religion(&self) -> i32 {
        self.und + self.oldtime_religion_mod
    }

    fn total_points(&self) -> i32 {
        self.tech + self.arts + self.bur + self.und
    }

    fn remaining_points(&self) -> i32 {
        12 - self.total_points()
    }

    fn is_valid(&self) -> bool {
        self.tech >= 1 && self.arts >= 1 && self.bur >= 1 && self.und >= 1
            && self.tech <= 6 && self.arts <= 6 && self.bur <= 6 && self.und <= 6
            && self.total_points() == 12
    }

    fn add_xp(&mut self, amount: i32) {
        self.xp += amount;

        // Handle leveling up
        while self.xp >= 100 {
            self.xp -= 100; // Reset XP and preserve the overflow
            self.skill_points += 1; // Award skill points
            println!("You gained a skill point! You now have {} skill points.", self.skill_points);
        }
    }
}

impl Default for Player {
    fn default() -> Self {
        Self {
        tech: 1,
        arts: 1,
        bur: 1, //short for bureaucracy
        und: 1, //short for underworld
        checkmate_mod: 0,
        rocketry_mod: 0,
        pathology_mod: 0,
        civic_engineering_mod: 0,
        apparatchik_mod: 0,
        quota_mod: 0,
        robot_mod: 0,
        dossier_mod: 0,
        delusion_mod: 0,
        arts2_mod: 0,
        arts3_mod: 0,
        arts4_mod: 0,
        high_proof_mod: 0,
        prohibition_mod: 0,
        gizmo_mod: 0,
        oldtime_religion_mod: 0,
        items: vec![],
        xp: 0,
        skill_points: 0,
        dialogues_entered: HashSet::new(),
        flags: HashSet::new(),
        }
    }
}

enum GameState {
    CharacterCreation,
    InGame,
    InventoryView,
    SkillManagement,
}

fn main() {
    
    let app = DialogueApp::default();
    let mut native_options = eframe::NativeOptions::default();
    native_options.fullscreen = true; 

    eframe::run_native(
        "Dialogue System",  // App name
        native_options,     // Window options
        Box::new(|_cc| Box::new(app)),  // Closure to create the app
    );
}

