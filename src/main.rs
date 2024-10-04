use eframe::{egui, Frame};
use rand::Rng;
use std::collections::HashMap;

// Define Location structure
struct Location {
    name: String,
    dialogues: HashMap<String, Dialogue>,
    exits: Vec<String>, // Names of other locations you can move to
}

struct DialogueApp {
    current_text: String,
    player: Player,
    locations: HashMap<String, Location>, // All locations in the game
    current_location_id: String,          // Current location ID
    current_dialogue_id: Option<String>,  // Current dialogue ID, or None if not in a dialogue
}

impl Default for DialogueApp {
    fn default() -> Self {
        let mut locations = HashMap::new();

        // Define sample dialogues for the Vestibule
        let mut vestibule_dialogues = HashMap::new();
        vestibule_dialogues.insert(
            "Start".to_string(),
            Dialogue {
            speaker: "".to_string(),
            intro: r"The front door swings shut, cutting off the bitter wind like a scythe.\
                You stand in the harsh light of a public apartment vestibule.\ 
                A grandfather clock stands stout against the wall, like an elderly servant \
                whose crooked back can't quite stand up to attention.".to_string(),
                options: vec![
                    DialogueOption {
                        description: "Inspect the grandfather clock.".to_string(),
                        challenge_attribute: None,
                        challenge_number: None,
                        success_dialogue: Some("InspectClock".to_string()),
                        failure_dialogue: None,
                    },
                    DialogueOption {
                        description: "Go to the first floor.".to_string(),
                        challenge_attribute: None,
                        challenge_number: None,
                        success_dialogue: Some("FirstFloor".to_string()),
                        failure_dialogue: None,
                    },
                ],
                is_hidden: false,
            },
        );

        vestibule_dialogues.insert(
            "InspectClock".to_string(),
            Dialogue {
                speaker: "Grandfather Clock".to_string(),
                intro: "A round, pale face crossed by dark lines stares down at you. It has seen much, and forgotten more.".to_string(),
                options: vec![
                    DialogueOption {
                        description: "What stories could you tell me, old man? (Delusion 12)".to_string(), 
                        challenge_attribute: Some("delusion".to_string()),
                        challenge_number: Some(12),
                        success_dialogue: Some("HungryClock".to_string()), 
                        failure_dialogue: Some("MockingClock".to_string()),

                        // hungry for the gear? it wants to eat its beating heart. Can give you a hint of where to find it
                    },

                    //to go back to the room's origin, the code is 'Start'

                    DialogueOption {
                        description: "Check the time.".to_string(), 
                        challenge_attribute: None,
                        challenge_number: None,
                        success_dialogue: Some("BrokenClock".to_string()), 
                        failure_dialogue: None,
                    },

                    DialogueOption {
                        description: "Open it up.".to_string(), 
                        challenge_attribute: None,
                        challenge_number: None,
                        success_dialogue: Some("ClockInterior".to_string()), 
                        failure_dialogue: None,
                    },

                    DialogueOption {
                        description: "Goodbye, fair clock. (End conversation).".to_string(), 
                        challenge_attribute: None,
                        challenge_number: None,
                        success_dialogue: Some("Start".to_string()), 
                        failure_dialogue: None,
                    },
                ],
                is_hidden: true,
            },
        );

        vestibule_dialogues.insert(
            "HungryClock".to_string(),
            Dialogue {
                speaker: "Grandfather Clock".to_string(),
                intro: "".to_string(),
                options: vec![
                    DialogueOption {
                        description: "".to_string(),
                        challenge_attribute: None,
                        challenge_number: None,
                        success_dialogue: Some("".to_string()),
                        failure_dialogue: None,
                    },
                ],
                is_hidden: true,
            }
        );

        vestibule_dialogues.insert(
            "BrokenClock".to_string(),
            Dialogue {
                speaker: "Grandfather Clock".to_string(),
                intro: r"Its two hands, the shorter ending in a stylized sun and the longer in a crescent moon, \
                are stuck at 5 hours and 37 minutes".to_string(), // this is a CLUE
                options: vec![
                    DialogueOption {
                        description: "What a waste. Surely I could fix it up?".to_string(),
                        challenge_attribute: None,
                        challenge_number: None,
                        success_dialogue: Some("ClockRecommends".to_string()), //gizmo to fix it, or telling you to open it up to do so
                        failure_dialogue: None,
                    },

                    DialogueOption {
                        description: "Five-thirty-seven in the morning or night?".to_string(),
                        challenge_attribute: None,
                        challenge_number: None,
                        success_dialogue: Some("ClockFutile".to_string()), //futilely ask if this is morning or night
                        failure_dialogue: None,
                    },

                    DialogueOption {
                        description: "Stand proud, fallen soldier of the Republic! (Salute the clock)".to_string(),
                        challenge_attribute: None,
                        challenge_number: None,
                        success_dialogue: Some("ClockSalute".to_string()), //salute it
                        failure_dialogue: None,
                    },

                    DialogueOption {
                        description: "Goodbye, mysterious clock.".to_string(),
                        challenge_attribute: None,
                        challenge_number: None,
                        success_dialogue: Some("Start".to_string()), //exit
                        failure_dialogue: None,
                    },
                ],
                is_hidden: true,
            }
        );


        // have a passive apparatchik check to know about fictional soviet-era clock and time policies

        // this check to notice that it's missing a crucial gear, and what to look for elsewhere
        // can be retried later if you happen to find the gear with a bonus, if you succeed here you can
        //fix it automatically with the gear

        vestibule_dialogues.insert(
            "ClockInterior".to_string(),
            Dialogue {
                speaker: "Grandfather Clock".to_string(),
                intro: "".to_string(),
                options: vec![
                    DialogueOption {
                        description: "".to_string(),
                        challenge_attribute: None,
                        challenge_number: None,
                        success_dialogue: Some("".to_string()),
                        failure_dialogue: None,
                    },
                ],
                is_hidden: true,
            }
        );

        vestibule_dialogues.insert(
            "MockingClock".to_string(),
            Dialogue {
                speaker: "Grandfather Clock".to_string(),
                intro: r"It seems to talk to you. It says 'Why are you talking to a clock, you absolute dolt? You twat.\
                Has my non-sentient existence come to the point that madmen come in off the street and try to engage me in conversation?'".to_string(),
                options: vec![
                    DialogueOption {
                        description: "Okay, that's just, like, your opinion, man. (End conversation)".to_string(),
                        challenge_attribute: None,
                        challenge_number: None,
                        success_dialogue: Some("Start".to_string()), // Go back to the room's main dialogue
                        failure_dialogue: None,
                    },

                    DialogueOption {
                        description: "Tell me your secrets!".to_string(),
                        challenge_attribute: None,
                        challenge_number: None,
                        success_dialogue: Some("ClockDecline".to_string()), // Go back to the room's main dialogue
                        failure_dialogue: None,
                    },
                ],
                is_hidden: true,
            }
        );

        vestibule_dialogues.insert(
            "ClockDecline".to_string(),
            Dialogue {
                speaker: "Grandfather Clock".to_string(),
                intro: "No.".to_string(),
                options: vec![
                    DialogueOption {
                        description: "Well, alright then.".to_string(),
                        challenge_attribute: None,
                        challenge_number: None,
                        success_dialogue: Some("Start".to_string()), // Go back to the room's main dialogue
                        failure_dialogue: None,
                    },

                    DialogueOption {
                        description: "Fuck you, and your little cuckoo too.".to_string(),
                        challenge_attribute: None,
                        challenge_number: None,
                        success_dialogue: Some("Start".to_string()), // Go back to the room's main dialogue
                        failure_dialogue: None,
                    },
                ],
                is_hidden: true,
            }
        );

        vestibule_dialogues.insert(
            "ClockRecommends".to_string(),
            Dialogue {
                speaker: "Grandfather Clock".to_string(),
                intro: r"You'd have to get into this thing's guts. Are you mentally and *spiritually* prepared to mess around \
                with this poor, fallen soldier's insides? On the off chance you can restore life to the dead?".to_string(),
                options: vec![
                    DialogueOption {
                        description: "On second thought, let's not do that.".to_string(),
                        challenge_attribute: None,
                        challenge_number: None,
                        success_dialogue: Some("BrokenClock".to_string()), 
                        failure_dialogue: None,
                    },

                    DialogueOption {
                        description: "Come on, why did you have to phrase it that way?".to_string(),
                        challenge_attribute: None,
                        challenge_number: None,
                        success_dialogue: Some("ClockPhrasing".to_string()), 
                        failure_dialogue: None,
                    },

                    DialogueOption {
                        description: "Let's do this.".to_string(),
                        challenge_attribute: None,
                        challenge_number: None,
                        success_dialogue: Some("ClockInterior".to_string()), // Go back to the room's main dialogue
                        failure_dialogue: None,
                    },
                ],
                is_hidden: true,
            }
        );

        vestibule_dialogues.insert(
            "ClockPhrasing".to_string(),
            Dialogue {
                speaker: "Grandfather Clock".to_string(),
                intro: "Like what?".to_string(),
                options: vec![
                    DialogueOption {
                        description: "Like I'm doing something creepy here! It's just a broken clock.".to_string(),
                        challenge_attribute: None,
                        challenge_number: None,
                        success_dialogue: Some("JustABrokenClock".to_string()), 
                        failure_dialogue: None,
                    },

                    DialogueOption {
                        description: "Like this is some *religious* thing. That's not what I'm about.".to_string(),
                        challenge_attribute: None,
                        challenge_number: None,
                        success_dialogue: Some("ReligiousClock".to_string()), 
                        failure_dialogue: None,
                    },

                    DialogueOption {
                        description: "Like... you know.".to_string(),
                        challenge_attribute: None,
                        challenge_number: None,
                        success_dialogue: Some("ClockSuggestive".to_string()), 
                        failure_dialogue: None,
                    },

                    DialogueOption {
                        description: "You know what, never mind.".to_string(),
                        challenge_attribute: None,
                        challenge_number: None,
                        success_dialogue: Some("BrokenClock".to_string()), 
                        failure_dialogue: None,
                    },

                    DialogueOption {
                        description: "Fine, I'll get *into its guts*. What's in there?".to_string(),
                        challenge_attribute: None,
                        challenge_number: None,
                        success_dialogue: Some("ClockInterior".to_string()), 
                        failure_dialogue: None,
                    },
                ],
                is_hidden: true,
            }
        );

        vestibule_dialogues.insert(
            "JustABrokenClock".to_string(),
            Dialogue {
                speaker: "Grandfather Clock".to_string(),
                intro: "".to_string(),
                options: vec![
                    DialogueOption {
                        description: "".to_string(),
                        challenge_attribute: None,
                        challenge_number: None,
                        success_dialogue: Some("Start".to_string()), // Go back to the room's main dialogue
                        failure_dialogue: None,
                    },
                ],
                is_hidden: true,
            }
        );

        vestibule_dialogues.insert(
            "ReligiousClock".to_string(),
            Dialogue {
                speaker: "Grandfather Clock".to_string(),
                intro: "".to_string(),
                options: vec![
                    DialogueOption {
                        description: "".to_string(),
                        challenge_attribute: None,
                        challenge_number: None,
                        success_dialogue: Some("Start".to_string()), // Go back to the room's main dialogue
                        failure_dialogue: None,
                    },
                ],
                is_hidden: true,
            }
        );

        vestibule_dialogues.insert(
            "ClockSuggestive".to_string(),
            Dialogue {
                speaker: "Grandfather Clock".to_string(),
                intro: "I'm very, *very* sure I don't.".to_string(),
                options: vec![
                    DialogueOption {
                        description: "Like it's something... sexual.".to_string(),
                        challenge_attribute: None,
                        challenge_number: None,
                        success_dialogue: Some("ClockSexual".to_string()), 
                        failure_dialogue: None,
                    },

                    DialogueOption {
                        description: "Fine, I'll leave off. What was that about a sun and moon again?".to_string(),
                        challenge_attribute: None,
                        challenge_number: None,
                        success_dialogue: Some("BrokenClock".to_string()), 
                        failure_dialogue: None,
                    },
                ],
                is_hidden: true,
            }
        );

        vestibule_dialogues.insert(
            "ClockSexual".to_string(),
            Dialogue {
                speaker: "Grandfather Clock".to_string(),
                intro: r"Wow. Just wow. Just what about sticking your hands *wrist deep* in your comrade's bloody, \
                pulsating wound seems *sexual* to you?".to_string(),
                options: vec![
                    DialogueOption {
                        description: r"It's not you, it's everyone. Come up with a theoretical justification for your \
                        objectively weird thoughts. ".to_string(),
                        challenge_attribute: Some("pathology".to_string()),
                        challenge_number: Some(8),
                        success_dialogue: Some("ClockPerversion".to_string()), //Pathology check to justify yourself
                        failure_dialogue: Some("ClockTrauma".to_string()),
                    },

                    DialogueOption {
                        description: "There it is again! You're putting this weird *emphasis* on things.".to_string(),
                        challenge_attribute: None,
                        challenge_number: None,
                        success_dialogue: Some("ClockEmphasis".to_string()), 
                        failure_dialogue: None,
                    },

                    DialogueOption {
                        description: "Actually, you know what? This isn't a sexual thing, it's a religious thing.".to_string(),
                        challenge_attribute: None,
                        challenge_number: None,
                        success_dialogue: Some("ClockSexualReligion".to_string()), 
                        failure_dialogue: None,
                    },
                ],
                is_hidden: true,
            }
        );

        vestibule_dialogues.insert(
            "ClockPerversion".to_string(),
            Dialogue {
                speaker: "Grandfather Clock".to_string(),
                intro: r"It's an extremely common and not at all shameful perversion, characteristic of the Republic's \
                struggles against the insidious American *Central Intelligence Agency*.".to_string(),
                options: vec![
                    DialogueOption {
                        description: "Let's go with that.".to_string(),
                        challenge_attribute: None,
                        challenge_number: None,
                        success_dialogue: Some("ClockCIA".to_string()), // Go back to the room's main dialogue
                        failure_dialogue: None,
                    },

                    DialogueOption {
                        description: "No, you can't possibly be serious. How can that possibly make sense?".to_string(),
                        challenge_attribute: None,
                        challenge_number: None,
                        success_dialogue: Some("ClockExplanation".to_string()), // Go back to the room's main dialogue
                        failure_dialogue: None,
                    },
                ],
                is_hidden: true,
            }
        );

        vestibule_dialogues.insert(
            "ClockTrauma".to_string(),
            Dialogue {
                speaker: "Grandfather Clock".to_string(),
                intro: "It's a trauma response.".to_string(),
                options: vec![
                    DialogueOption {
                        description: "What, seriously?".to_string(),
                        challenge_attribute: None,
                        challenge_number: None,
                        success_dialogue: Some("ClockNotTrauma".to_string()), // Go back to the room's main dialogue
                        failure_dialogue: None,
                    },

                    DialogueOption {
                        description: "Did I have to do... that... during The War? Do I have repressed memories I need to process?".to_string(),
                        challenge_attribute: None,
                        challenge_number: None,
                        success_dialogue: Some("ClockRepression".to_string()), // Go back to the room's main dialogue
                        failure_dialogue: None,
                    },
                ],
                is_hidden: true,
            }
        );

        vestibule_dialogues.insert(
            "ClockNotTrauma".to_string(),
            Dialogue {
                speaker: "Grandfather Clock".to_string(),
                intro: r"Look, all I now is that whenever something like this happens to you, there's exactly one \
                phrase bouncing around this empty skull, and it's 'trauma response'. Trauma response this, \
                trauma response that. It gets you out of anything.".to_string(),
                options: vec![
                    DialogueOption {
                        description: "".to_string(),
                        challenge_attribute: None,
                        challenge_number: None,
                        success_dialogue: Some("".to_string()), 
                        failure_dialogue: None,
                    },
                ],
                is_hidden: true,
            }
        );

        vestibule_dialogues.insert(
            "ClockRepression".to_string(),
            Dialogue {
                speaker: "Grandfather Clock".to_string(),
                intro: r"Something here is definitely repressed, but it's not memories. You're no amnesiac, \
                as much as you might like to be, at times.".to_string(),
                options: vec![
                    DialogueOption {
                        description: "".to_string(),
                        challenge_attribute: None,
                        challenge_number: None,
                        success_dialogue: Some("".to_string()), 
                        failure_dialogue: None,
                    },
                ],
                is_hidden: true,
            }
        );

        vestibule_dialogues.insert(
            "ClockCIA".to_string(),
            Dialogue {
                speaker: "Grandfather Clock".to_string(),
                intro: "".to_string(),
                options: vec![
                    DialogueOption {
                        description: "".to_string(),
                        challenge_attribute: None,
                        challenge_number: None,
                        success_dialogue: Some("".to_string()),
                        failure_dialogue: None,
                    },
                ],
                is_hidden: true,
            }
        );

        vestibule_dialogues.insert(
            "ClockExplanation".to_string(),
            Dialogue {
                speaker: "Grandfather Clock".to_string(),
                intro: r"Simple. Communism is built upon love for your comrade. \
                Not bourgeous romantic love, or feudal familial love, or reactionary \
                love of country, but the superior and pure love of the worker standing \
                by your side. The CIA's modus operandi is to undermine the foundation \
                of communism by turning comrades against one another. This objective, \
                to our great misfortune, has been largely successful. Everywhere now, \
                you can see signs of bourgeois romantic love displacing \
                true communist camaraderie, and bringing with it a perverse focus on \
                sex. But this train of thought, by re-associating the apparently \
                sexual elements with the suffering of one's comrades, reinforces the \
                revolutionary spirit and wards off capitalist infiltration. The only \
                reason you recoil from this image, instead of embracing it, is \
                because you yet possess a remnant of the bourgeois ego, nurtured by CIA \
                propaganda, to which laying down all and destroying all boundaries \
                for the sake of your comrades is unacceptable.".to_string(),
                options: vec![
                    DialogueOption {
                        description: "".to_string(),
                        challenge_attribute: None,
                        challenge_number: None,
                        success_dialogue: Some("".to_string()), 
                        failure_dialogue: None,
                    },
                ],
                is_hidden: true,
            }
        );


        // default template

        vestibule_dialogues.insert(
            "".to_string(),
            Dialogue {
                speaker: "".to_string(),
                intro: "".to_string(),
                options: vec![
                    DialogueOption {
                        description: "".to_string(),
                        challenge_attribute: None,
                        challenge_number: None,
                        success_dialogue: Some("Start".to_string()), // Go back to the room's main dialogue
                        failure_dialogue: None,
                    },
                ],
                is_hidden: true,
            }
        );

        // vestibule_dialogues.insert(
        //     "InspectClock".to_string(),
        //     Dialogue {
        //         intro: "A round, pale face crossed by dark lines stares down at you. ".to_string(),
        //         options: vec![
        //             DialogueOption {
        //                 description: "Exit conversation.".to_string(), // This sends you back to Vestibule's "Start"
        //                 challenge_attribute: None,
        //                 challenge_number: None,
        //                 success_dialogue: Some("Start".to_string()), // Go back to the room's main dialogue
        //                 failure_dialogue: None,
        //             },
        //         ],
        //         is_hidden: false,
        //     },
        // );




        /////// bellhop dialogues above here
        /// replace with a grandfather clock, secrets within




        // Define sample dialogues for the First Floor
        let mut first_floor_dialogues = HashMap::new();
        first_floor_dialogues.insert(
            "Start".to_string(),
            Dialogue {
                speaker: "".to_string(),
                intro: "You are now on the first floor. A butler greets you.".to_string(),
                options: vec![
                    DialogueOption {
                        description: "Talk to the butler.".to_string(),
                        challenge_attribute: None,
                        challenge_number: None,
                        success_dialogue: Some("ButlerResponse".to_string()),
                        failure_dialogue: None,
                    },
                    DialogueOption {
                        description: "Try to open the stuck door (Dossier challenge).".to_string(),
                        challenge_attribute: Some("dossier".to_string()),
                        challenge_number: Some(12),
                        success_dialogue: Some("Garden".to_string()),
                        failure_dialogue: Some("FailedDoor".to_string()),
                    },
                    DialogueOption {
                        description: "Go back to the vestibule.".to_string(),
                        challenge_attribute: None,
                        challenge_number: None,
                        success_dialogue: Some("Vestibule".to_string()),
                        failure_dialogue: None,
                    },
                ],
                is_hidden: false,
            },
        );

        // Butler Dialogue
        first_floor_dialogues.insert(
            "ButlerResponse".to_string(),
            Dialogue {
                speaker: "".to_string(),
                intro: "The butler nods respectfully. 'The garden is beyond the stuck door,' he mentions.".to_string(),
                options: vec![
                    DialogueOption {
                        description: "Exit conversation.".to_string(), // This sends you back to First Floor's "Start"
                        challenge_attribute: None,
                        challenge_number: None,
                        success_dialogue: Some("Start".to_string()), // Go back to the room's main dialogue
                        failure_dialogue: None,
                    },
                ],
                is_hidden: false,
            },
        );

        first_floor_dialogues.insert(
            "FailedDoor".to_string(),
            Dialogue {
                speaker: "".to_string(),
                intro: "The door remains stuck, refusing to budge.".to_string(),
                options: vec![
                    DialogueOption {
                        description: "Go back to the first floor.".to_string(),
                        challenge_attribute: None,
                        challenge_number: None,
                        success_dialogue: Some("Start".to_string()), // Go back to the first floor's main dialogue
                        failure_dialogue: None,
                    },
                ],
                is_hidden: false,
            },
        );

        // Define sample dialogues for the Garden
        let mut garden_dialogues = HashMap::new();
        garden_dialogues.insert(
            "Start".to_string(),
            Dialogue {
                speaker: "".to_string(),
                intro: "You are now in the garden. An old woman sits on a bench.".to_string(),
                options: vec![
                    DialogueOption {
                        description: "Talk to the old woman.".to_string(),
                        challenge_attribute: None,
                        challenge_number: None,
                        success_dialogue: Some("OldWomanResponse".to_string()),
                        failure_dialogue: None,
                    },
                    DialogueOption {
                        description: "Go back to the first floor.".to_string(),
                        challenge_attribute: None,
                        challenge_number: None,
                        success_dialogue: Some("FirstFloor".to_string()),
                        failure_dialogue: None,
                    },
                ],
                is_hidden: false,
            },
        );

        // Old Woman Dialogue
        garden_dialogues.insert(
            "OldWomanResponse".to_string(),
            Dialogue {
                speaker: "".to_string(),
                intro: "The old woman smiles softly and speaks in a quiet voice.".to_string(),
                options: vec![
                    DialogueOption {
                        description: "Exit conversation.".to_string(), // Go back to Garden's "Start"
                        challenge_attribute: None,
                        challenge_number: None,
                        success_dialogue: Some("Start".to_string()), // Go back to the Garden's main dialogue
                        failure_dialogue: None,
                    },
                ],
                is_hidden: false,
            },
        );

        // Define Locations
        locations.insert(
            "Vestibule".to_string(),
            Location {
                name: "The Vestibule".to_string(),
                dialogues: vestibule_dialogues,
                exits: vec!["FirstFloor".to_string()],
            },
        );

        locations.insert(
            "FirstFloor".to_string(),
            Location {
                name: "The First Floor".to_string(),
                dialogues: first_floor_dialogues,
                exits: vec!["Vestibule".to_string(), "Garden".to_string()],
            },
        );

        locations.insert(
            "Garden".to_string(),
            Location {
                name: "The Garden".to_string(),
                dialogues: garden_dialogues,
                exits: vec!["FirstFloor".to_string()],
            },
        );

        Self {
            current_text: "Welcome!".to_string(),
            player: Player {
                tech: 2,
                arts: 4,
                bur: 1, //short for bureaucracy
                und: 2, //short for underworld
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
            },
            locations,
            current_location_id: "Vestibule".to_string(), // Start in the Vestibule
            current_dialogue_id: Some("Start".to_string()), // Start with the "Start" dialogue
        }
    }
}

















// Implement eframe::App
impl eframe::App for DialogueApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let current_dialogue_id_clone = self.current_dialogue_id.clone();

            if let Some(current_dialogue_id) = &current_dialogue_id_clone {
                if let Some(current_dialogue) = self.get_current_dialogue_from_id(current_dialogue_id) {

                    // Display the speaker's name before the dialogue
                    ui.heading(&format!("{} ", current_dialogue.speaker));  // Display the speaker's name

                    // Display the dialogue
                    ui.heading(&current_dialogue.intro);

                    let mut new_dialogue_id = None;
                    let mut new_location_id = None;

                    for option in current_dialogue.options.iter() {
                        if ui.button(&option.description).clicked() {
                            if option.challenge_number.is_some() {
                                let success = handle_challenge(&self.player, option);
                                if success {
                                    // Dossier challenge success - transition to Garden
                                    if option.success_dialogue == Some("Garden".to_string()) {
                                        new_location_id = Some("Garden".to_string());
                                        new_dialogue_id = Some("Start".to_string());  // Set to the Garden's start dialogue
                                    } else {
                                        new_dialogue_id = option.success_dialogue.clone();
                                    }
                                } else {
                                    new_dialogue_id = option.failure_dialogue.clone();
                                }
                            } else if let Some(success_dialogue) = &option.success_dialogue {
                                if self.locations.contains_key(success_dialogue) {
                                    new_location_id = Some(success_dialogue.clone());
                                    new_dialogue_id = None;
                                } else {
                                    new_dialogue_id = Some(success_dialogue.clone());
                                }
                            }
                        }
                    }

                    if let Some(new_id) = new_dialogue_id {
                        self.current_dialogue_id = Some(new_id);
                    }
                    if let Some(new_location) = new_location_id {
                        self.current_location_id = new_location;
                    }
                }
            } else {
                ui.heading(&format!("You are in {}", self.get_current_location().name));

                let mut new_location_id = None;

                ui.label("Exits:");
                for exit in &self.get_current_location().exits {
                    if ui.button(exit).clicked() {
                        new_location_id = Some(exit.clone());
                    }
                }

                if let Some(new_location) = new_location_id {
                    self.current_location_id = new_location;
                    self.current_dialogue_id = Some("Start".to_string()); // Reset to "Start" dialogue in the new location
                }
            }
        });
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

    fn get_current_location(&self) -> &Location {
        self.locations.get(&self.current_location_id).unwrap()
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
    oldtime_religion_mod: i32
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
}

#[derive(Clone)]
struct DialogueOption {
    description: String,
    challenge_attribute: Option<String>,
    challenge_number: Option<i32>,
    success_dialogue: Option<String>,
    failure_dialogue: Option<String>,
}

#[derive(Clone)]
struct Dialogue {
    speaker: String,
    intro: String,
    options: Vec<DialogueOption>,
    is_hidden: bool,
}

fn main() {
    let app = DialogueApp::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Dialogue System",  // App name
        native_options,     // Window options
        Box::new(|_cc| Box::new(app)),  // Closure to create the app
    );
}










//// above is the most recent working version of the code
//// below is attempts to go further that failed, as well as useful legacy code including the full set of character traits




//////
//////



// use eframe::{egui, Frame};
// use rand::Rng;
// use std::collections::HashMap;

// // Define Location structure
// struct Location {
//     name: String,
//     dialogues: HashMap<String, Dialogue>,
//     exits: Vec<String>, // Names of other locations you can move to
// }

// struct DialogueApp {
//     current_text: String,
//     player: Player,
//     locations: HashMap<String, Location>, // All locations in the game
//     current_location_id: String,          // Current location ID
//     current_dialogue_id: Option<String>,  // Current dialogue ID, or None if not in a dialogue
// }

// // impl Default for DialogueApp {
// //     fn default() -> Self {
// //         let mut locations = HashMap::new();

// //         // Define sample dialogues for the Vestibule
// //         let mut vestibule_dialogues = HashMap::new();
// //         vestibule_dialogues.insert(
// //             "Start".to_string(),
// //             Dialogue {
// //                 intro: "You are in the vestibule. A bellhop stands at attention.".to_string(),
// //                 options: vec![
// //                     DialogueOption {
// //                         description: "Talk to the bellhop.".to_string(),
// //                         challenge_attribute: None,
// //                         challenge_number: None,
// //                         success_dialogue: Some("BellhopResponse".to_string()),  // Keep player in vestibule
// //                         failure_dialogue: None,
// //                     },
// //                     DialogueOption {
// //                         description: "Go to the first floor.".to_string(),
// //                         challenge_attribute: None,
// //                         challenge_number: None,
// //                         success_dialogue: Some("FirstFloor".to_string()),  // Go to first floor
// //                         failure_dialogue: None,
// //                     },
// //                 ],
// //                 is_hidden: false,
// //             },
// //         );

// //         vestibule_dialogues.insert(
// //             "BellhopResponse".to_string(),
// //             Dialogue {
// //                 intro: "The bellhop nods and smiles warmly. 'Welcome, sir,' he says.".to_string(),
// //                 options: vec![
// //                     DialogueOption {
// //                         description: "Stay in the vestibule.".to_string(),  // Add this option to stay
// //                         challenge_attribute: None,
// //                         challenge_number: None,
// //                         success_dialogue: Some("Start".to_string()),  // Loop back to vestibule start
// //                         failure_dialogue: None,
// //                     },
// //                     DialogueOption {
// //                         description: "Go to the first floor.".to_string(),
// //                         challenge_attribute: None,
// //                         challenge_number: None,
// //                         success_dialogue: Some("FirstFloor".to_string()),
// //                         failure_dialogue: None,
// //                     },
// //                 ],
// //                 is_hidden: false,
// //             },
// //         );

// //         // Define sample dialogues for the First Floor
// //         let mut first_floor_dialogues = HashMap::new();
// //         first_floor_dialogues.insert(
// //             "Start".to_string(),
// //             Dialogue {
// //                 intro: "You are now on the first floor. A butler greets you.".to_string(),
// //                 options: vec![
// //                     DialogueOption {
// //                         description: "Talk to the butler.".to_string(),
// //                         challenge_attribute: None,
// //                         challenge_number: None,
// //                         success_dialogue: Some("ButlerResponse".to_string()),
// //                         failure_dialogue: None,
// //                     },
// //                     DialogueOption {
// //                         description: "Try to open the stuck door (Strength challenge).".to_string(),
// //                         challenge_attribute: Some("strength".to_string()),
// //                         challenge_number: Some(12),
// //                         success_dialogue: Some("Garden".to_string()),  // Ensure this leads to Garden
// //                         failure_dialogue: Some("FailedDoor".to_string()),
// //                     },
// //                     DialogueOption {
// //                         description: "Go back to the vestibule.".to_string(),
// //                         challenge_attribute: None,
// //                         challenge_number: None,
// //                         success_dialogue: Some("Vestibule".to_string()),
// //                         failure_dialogue: None,
// //                     },
// //                 ],
// //                 is_hidden: false,
// //             },
// //         );

// //         first_floor_dialogues.insert(
// //             "ButlerResponse".to_string(),
// //             Dialogue {
// //                 intro: "The butler nods respectfully. 'The garden is beyond the stuck door,' he mentions.".to_string(),
// //                 options: vec![
// //                     DialogueOption {
// //                         description: "Try to open the stuck door (Strength challenge).".to_string(),
// //                         challenge_attribute: Some("strength".to_string()),
// //                         challenge_number: Some(12),
// //                         success_dialogue: Some("Garden".to_string()),  // Make sure this leads to Garden
// //                         failure_dialogue: Some("FailedDoor".to_string()),
// //                     },
// //                 ],
// //                 is_hidden: false,
// //             },
// //         );

// //         first_floor_dialogues.insert(
// //             "FailedDoor".to_string(),
// //             Dialogue {
// //                 intro: "The door remains stuck, refusing to budge.".to_string(),
// //                 options: vec![
// //                     DialogueOption {
// //                         description: "Go back to the vestibule.".to_string(),
// //                         challenge_attribute: None,
// //                         challenge_number: None,
// //                         success_dialogue: Some("Vestibule".to_string()),
// //                         failure_dialogue: None,
// //                     },
// //                 ],
// //                 is_hidden: false,
// //             },
// //         );

// //         // Define sample dialogues for the Garden
// //         let mut garden_dialogues = HashMap::new();
// //         garden_dialogues.insert(
// //             "Start".to_string(),
// //             Dialogue {
// //                 intro: "You are now in the garden. An old woman sits on a bench.".to_string(),
// //                 options: vec![
// //                     DialogueOption {
// //                         description: "Talk to the old woman.".to_string(),
// //                         challenge_attribute: None,
// //                         challenge_number: None,
// //                         success_dialogue: Some("OldWomanResponse".to_string()),
// //                         failure_dialogue: None,
// //                     },
// //                 ],
// //                 is_hidden: false,
// //             },
// //         );

// //         garden_dialogues.insert(
// //             "OldWomanResponse".to_string(),
// //             Dialogue {
// //                 intro: "The old woman smiles softly and speaks in a quiet voice.".to_string(),
// //                 options: vec![
// //                     DialogueOption {
// //                         description: "Go back to the first floor.".to_string(),
// //                         challenge_attribute: None,
// //                         challenge_number: None,
// //                         success_dialogue: Some("FirstFloor".to_string()),
// //                         failure_dialogue: None,
// //                     },
// //                 ],
// //                 is_hidden: false,
// //             },
// //         );

// //         // Define Locations
// //         locations.insert(
// //             "Vestibule".to_string(),
// //             Location {
// //                 name: "The Vestibule".to_string(),
// //                 dialogues: vestibule_dialogues,
// //                 exits: vec!["FirstFloor".to_string()],
// //             },
// //         );

// //         locations.insert(
// //             "FirstFloor".to_string(),
// //             Location {
// //                 name: "The First Floor".to_string(),
// //                 dialogues: first_floor_dialogues,
// //                 exits: vec!["Vestibule".to_string(), "Garden".to_string()],
// //             },
// //         );

// //         locations.insert(
// //             "Garden".to_string(),
// //             Location {
// //                 name: "The Garden".to_string(),
// //                 dialogues: garden_dialogues,
// //                 exits: vec!["FirstFloor".to_string()],
// //             },
// //         );

// //         Self {
// //             current_text: "Welcome!".to_string(),
// //             player: Player {
// //                 strength: 10,
// //                 wisdom: 8,
// //                 knowledge: 7,
// //             },
// //             locations,
// //             current_location_id: "Vestibule".to_string(), // Start in the Vestibule
// //             current_dialogue_id: Some("Start".to_string()), // Start with the "Start" dialogue
// //         }
// //     }
// // }

// // Add state tracking to Player to track success
// struct Player {
//     strength: i32,
//     wisdom: i32,
//     knowledge: i32,
//     door_opened: bool, // Track if the player has successfully opened the door
// }

// impl Player {
//     fn intelligence(&self) -> i32 {
//         self.wisdom + self.knowledge
//     }
// }

// impl Default for Player {
//     fn default() -> Self {
//         Self {
//             strength: 10,
//             wisdom: 8,
//             knowledge: 7,
//             door_opened: false, // Default to door not opened
//         }
//     }
// }

// impl Default for DialogueApp {
//     fn default() -> Self {
//         let mut locations = HashMap::new();

//         // Define sample dialogues for the Vestibule
//         let mut vestibule_dialogues = HashMap::new();
//         vestibule_dialogues.insert(
//             "Start".to_string(),
//             Dialogue {
//                 intro: "You are in the vestibule. A bellhop stands at attention.".to_string(),
//                 options: vec![
//                     DialogueOption {
//                         description: "Talk to the bellhop.".to_string(),
//                         challenge_attribute: None,
//                         challenge_number: None,
//                         success_dialogue: Some("BellhopResponse".to_string()),
//                         failure_dialogue: None,
//                     },
//                     DialogueOption {
//                         description: "Go to the first floor.".to_string(),
//                         challenge_attribute: None,
//                         challenge_number: None,
//                         success_dialogue: Some("FirstFloor".to_string()),
//                         failure_dialogue: None,
//                     },
//                 ],
//                 is_hidden: false,
//             },
//         );

//         // Bellhop Dialogue
//         vestibule_dialogues.insert(
//             "BellhopResponse".to_string(),
//             Dialogue {
//                 intro: "The bellhop nods and smiles warmly. 'Welcome, sir,' he says.".to_string(),
//                 options: vec![
//                     DialogueOption {
//                         description: "Exit conversation.".to_string(),
//                         challenge_attribute: None,
//                         challenge_number: None,
//                         success_dialogue: Some("Start".to_string()), // Go back to the room's main dialogue
//                         failure_dialogue: None,
//                     },
//                 ],
//                 is_hidden: false,
//             },
//         );

//         // Define sample dialogues for the First Floor
//         let mut first_floor_dialogues = HashMap::new();
//         first_floor_dialogues.insert(
//             "Start".to_string(),
//             Dialogue {
//                 intro: "You are now on the first floor. A butler greets you.".to_string(),
//                 options: vec![
//                     DialogueOption {
//                         description: "Talk to the butler.".to_string(),
//                         challenge_attribute: None,
//                         challenge_number: None,
//                         success_dialogue: Some("ButlerResponse".to_string()),
//                         failure_dialogue: None,
//                     },
//                     // Strength challenge for opening the door will be conditionally shown
//                     DialogueOption {
//                         description: "Try to open the stuck door (Strength challenge).".to_string(),
//                         challenge_attribute: Some("strength".to_string()),
//                         challenge_number: Some(12),
//                         success_dialogue: Some("Garden".to_string()),
//                         failure_dialogue: Some("FailedDoor".to_string()),
//                     },
//                     DialogueOption {
//                         description: "Go back to the vestibule.".to_string(),
//                         challenge_attribute: None,
//                         challenge_number: None,
//                         success_dialogue: Some("Vestibule".to_string()),
//                         failure_dialogue: None,
//                     },
//                 ],
//                 is_hidden: false,
//             },
//         );

//         // Butler Dialogue
//         first_floor_dialogues.insert(
//             "ButlerResponse".to_string(),
//             Dialogue {
//                 intro: "The butler nods respectfully. 'The garden is beyond the stuck door,' he mentions.".to_string(),
//                 options: vec![
//                     DialogueOption {
//                         description: "Exit conversation.".to_string(),
//                         challenge_attribute: None,
//                         challenge_number: None,
//                         success_dialogue: Some("Start".to_string()), // Go back to the room's main dialogue
//                         failure_dialogue: None,
//                     },
//                 ],
//                 is_hidden: false,
//             },
//         );

//         // Failed door option
//         first_floor_dialogues.insert(
//             "FailedDoor".to_string(),
//             Dialogue {
//                 intro: "The door remains stuck, refusing to budge.".to_string(),
//                 options: vec![
//                     DialogueOption {
//                         description: "Go back to the first floor.".to_string(),
//                         challenge_attribute: None,
//                         challenge_number: None,
//                         success_dialogue: Some("Start".to_string()), // Go back to the first floor's main dialogue
//                         failure_dialogue: None,
//                     },
//                 ],
//                 is_hidden: false,
//             },
//         );

//         // Define sample dialogues for the Garden
//         let mut garden_dialogues = HashMap::new();
//         garden_dialogues.insert(
//             "Start".to_string(),
//             Dialogue {
//                 intro: "You are now in the garden. An old woman sits on a bench.".to_string(),
//                 options: vec![
//                     DialogueOption {
//                         description: "Talk to the old woman.".to_string(),
//                         challenge_attribute: None,
//                         challenge_number: None,
//                         success_dialogue: Some("OldWomanResponse".to_string()),
//                         failure_dialogue: None,
//                     },
//                     DialogueOption {
//                         description: "Go back to the first floor.".to_string(),
//                         challenge_attribute: None,
//                         challenge_number: None,
//                         success_dialogue: Some("FirstFloor".to_string()),
//                         failure_dialogue: None,
//                     },
//                 ],
//                 is_hidden: false,
//             },
//         );

//         // Old Woman Dialogue
//         garden_dialogues.insert(
//             "OldWomanResponse".to_string(),
//             Dialogue {
//                 intro: "The old woman smiles softly and speaks in a quiet voice.".to_string(),
//                 options: vec![
//                     DialogueOption {
//                         description: "Exit conversation.".to_string(),
//                         challenge_attribute: None,
//                         challenge_number: None,
//                         success_dialogue: Some("Start".to_string()), // Go back to Garden's "Start"
//                         failure_dialogue: None,
//                     },
//                 ],
//                 is_hidden: false,
//             },
//         );

//         // Define Locations
//         locations.insert(
//             "Vestibule".to_string(),
//             Location {
//                 name: "The Vestibule".to_string(),
//                 dialogues: vestibule_dialogues,
//                 exits: vec!["FirstFloor".to_string()],
//             },
//         );

//         locations.insert(
//             "FirstFloor".to_string(),
//             Location {
//                 name: "The First Floor".to_string(),
//                 dialogues: first_floor_dialogues,
//                 exits: vec!["Vestibule".to_string(), "Garden".to_string()],
//             },
//         );

//         locations.insert(
//             "Garden".to_string(),
//             Location {
//                 name: "The Garden".to_string(),
//                 dialogues: garden_dialogues,
//                 exits: vec!["FirstFloor".to_string()],
//             },
//         );

//         Self {
//             current_text: "Welcome!".to_string(),
//             player: Player::default(), // Use the default Player with door_opened set to false
//             locations,
//             current_location_id: "Vestibule".to_string(), // Start in the Vestibule
//             current_dialogue_id: Some("Start".to_string()), // Start with the "Start" dialogue
//         }
//     }
// }

// // Update dialogue logic to check for the door_opened flag
// impl eframe::App for DialogueApp {
//     fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
//         egui::CentralPanel::default().show(ctx, |ui| {
//             let current_dialogue_id_clone = self.current_dialogue_id.clone();

//             if let Some(current_dialogue_id) = &current_dialogue_id_clone {
//                 if let Some(current_dialogue) = self.get_current_dialogue_from_id(current_dialogue_id) {
//                     ui.heading(&current_dialogue.intro);

//                     let mut new_dialogue_id = None;
//                     let mut new_location_id = None;

//                     // If we are in the First Floor, modify the dialogue options based on whether the door is opened
//                     if self.current_location_id == "FirstFloor" {
//                         // Check if the door is already opened
//                         if self.player.door_opened {
//                             // Replace the strength challenge with "Go through the door"
//                             for option in current_dialogue.options.iter() {
//                                 if option.description.contains("Go through the door") {
//                                     ui.label("You can go through the door."); // or whatever message you'd like
//                                     if ui.button("Go through the door").clicked() {
//                                         new_location_id = Some("Garden".to_string());
//                                         new_dialogue_id = Some("Start".to_string());
//                                     }
//                                 }
//                             }
//                         } else {
//                             // Regular strength challenge appears
//                             for option in current_dialogue.options.iter() {
//                                 if ui.button(&option.description).clicked() {
//                                     if option.challenge_number.is_some() {
//                                         let success = handle_challenge(&self.player, option);
//                                         if success {
//                                             // Strength challenge success - transition to Garden
//                                             self.player.door_opened = true;  // Mark door as opened
//                                             new_location_id = Some("Garden".to_string());
//                                             new_dialogue_id = Some("Start".to_string());
//                                         } else {
//                                             new_dialogue_id = option.failure_dialogue.clone();
//                                         }
//                                     }
//                                 }
//                             }
//                         }
//                     } else {
//                         // Regular option handling for other locations
//                         for option in current_dialogue.options.iter() {
//                             if ui.button(&option.description).clicked() {
//                                 if option.challenge_number.is_some() {
//                                     let success = handle_challenge(&self.player, option);
//                                     if success {
//                                         new_dialogue_id = option.success_dialogue.clone();
//                                     } else {
//                                         new_dialogue_id = option.failure_dialogue.clone();
//                                     }
//                                 } else if let Some(success_dialogue) = &option.success_dialogue {
//                                     if self.locations.contains_key(success_dialogue) {
//                                         new_location_id = Some(success_dialogue.clone());
//                                         new_dialogue_id = None;
//                                     } else {
//                                         new_dialogue_id = Some(success_dialogue.clone());
//                                     }
//                                 }
//                             }
//                         }
//                     }

//                     if let Some(new_id) = new_dialogue_id {
//                         self.current_dialogue_id = Some(new_id);
//                     }
//                     if let Some(new_location) = new_location_id {
//                         self.current_location_id = new_location;
//                     }
//                 }
//             } else {
//                 ui.heading(&format!("You are in {}", self.get_current_location().name));

//                 let mut new_location_id = None;

//                 ui.label("Exits:");
//                 for exit in &self.get_current_location().exits {
//                     if ui.button(exit).clicked() {
//                         new_location_id = Some(exit.clone());
//                     }
//                 }

//                 if let Some(new_location) = new_location_id {
//                     self.current_location_id = new_location;
//                     self.current_dialogue_id = Some("Start".to_string()); // Reset to "Start" dialogue in the new location
//                 }
//             }
//         });
//     }
// }




// // Add a helper function to get the dialogue based on the cloned dialogue ID
// impl DialogueApp {
//     fn get_current_dialogue_from_id(&self, dialogue_id: &String) -> Option<&Dialogue> {
//         if let Some(location) = self.locations.get(&self.current_location_id) {
//             return location.dialogues.get(dialogue_id);
//         }
//         None
//     }

//     fn get_current_location(&self) -> &Location {
//         self.locations.get(&self.current_location_id).unwrap()
//     }
// }

// // Challenge logic
// fn handle_challenge(player: &Player, option: &DialogueOption) -> bool {
//     if let Some(challenge_attribute) = &option.challenge_attribute {
//         if let Some(challenge_number) = option.challenge_number {
//             let attribute_value = match challenge_attribute.as_str() {
//                 "intelligence" => player.intelligence(),
//                 "strength" => player.strength,
//                 _ => 0,
//             };

//             let (die1, die2) = roll_dice();
//             let roll_sum = die1 + die2;

//             println!("You rolled: {} + {} = {}", die1, die2, roll_sum);

//             if die1 == 6 && die2 == 6 {
//                 println!("Double sixes! Automatic success.");
//                 return true;
//             } else if die1 == 1 && die2 == 1 {
//                 println!("Double ones! Automatic failure.");
//                 return false;
//             }

//             let total = roll_sum + attribute_value;
//             if total >= challenge_number {
//                 println!("Success! You needed {}, and you got {}.", challenge_number, total);
//                 return true;
//             } else {
//                 println!("Failure. You needed {}, but you got {}.", challenge_number, total);
//                 return false;
//             }
//         }
//     }
//     false
// }

// fn roll_dice() -> (i32, i32) {
//     let mut rng = rand::thread_rng();
//     (rng.gen_range(1..=6), rng.gen_range(1..=6))
// }

// // struct Player {
// //     strength: i32,
// //     wisdom: i32,
// //     knowledge: i32,
// // }

// // impl Player {
// //     fn intelligence(&self) -> i32 {
// //         self.wisdom + self.knowledge
// //     }
// // }

// #[derive(Clone)]
// struct DialogueOption {
//     description: String,
//     challenge_attribute: Option<String>,
//     challenge_number: Option<i32>,
//     success_dialogue: Option<String>,
//     failure_dialogue: Option<String>,
// }

// #[derive(Clone)]
// struct Dialogue {
//     intro: String,
//     options: Vec<DialogueOption>,
//     is_hidden: bool,
// }

// fn main() {
//     let app = DialogueApp::default();
//     let native_options = eframe::NativeOptions::default();
//     eframe::run_native(
//         "Dialogue System",  // App name
//         native_options,     // Window options
//         Box::new(|_cc| Box::new(app)),  // Closure to create the app
//     );
// }








//////////////////////////////
//////////////////////////////







// use eframe::{egui, Frame};
// use rand::Rng;
// use std::collections::HashMap;

// // Define Location structure
// struct Location {
//     name: String,
//     dialogues: HashMap<String, Dialogue>,
//     exits: Vec<String>, // Names of other locations you can move to
// }

// struct DialogueApp {
//     current_text: String,
//     player: Player,
//     locations: HashMap<String, Location>, // All locations in the game
//     current_location_id: String,          // Current location ID
//     current_dialogue_id: Option<String>,  // Current dialogue ID, or None if not in a dialogue
// }

// impl Default for DialogueApp {
//     fn default() -> Self {
//         let mut locations = HashMap::new();

//         // Define sample dialogues for Location 1
//         let mut location1_dialogues = HashMap::new();
//         location1_dialogues.insert(
//             "Start".to_string(),
//             Dialogue {
//                 intro: "You encounter a locked door.".to_string(),
//                 options: vec![
//                     DialogueOption {
//                         description: "Try to pick the lock (Intelligence challenge)".to_string(),
//                         challenge_attribute: Some("intelligence".to_string()),
//                         challenge_number: Some(12),
//                         success_dialogue: Some("UnlockedDoor".to_string()),
//                         failure_dialogue: Some("LockedDoor".to_string()),
//                     },
//                     DialogueOption {
//                         description: "Try to force the door open (Strength challenge)".to_string(),
//                         challenge_attribute: Some("strength".to_string()),
//                         challenge_number: Some(14),
//                         success_dialogue: Some("ForcedDoor".to_string()),
//                         failure_dialogue: Some("LockedDoor".to_string()),
//                     },
//                 ],
//                 is_hidden: false,
//             },
//         );

//         location1_dialogues.insert(
//             "UnlockedDoor".to_string(),
//             Dialogue {
//                 intro: "The door opens silently.".to_string(),
//                 options: vec![],
//                 is_hidden: false,
//             },
//         );

//         location1_dialogues.insert(
//             "LockedDoor".to_string(),
//             Dialogue {
//                 intro: "The door remains locked.".to_string(),
//                 options: vec![],
//                 is_hidden: false,
//             },
//         );

//         location1_dialogues.insert(
//             "ForcedDoor".to_string(),
//             Dialogue {
//                 intro: "You force the door open with a loud crack.".to_string(),
//                 options: vec![],
//                 is_hidden: false,
//             },
//         );

//         // Define Location 1
//         locations.insert(
//             "Room1".to_string(),
//             Location {
//                 name: "Room 1".to_string(),
//                 dialogues: location1_dialogues,
//                 exits: vec!["Room2".to_string()],
//             },
//         );

//         // Define Location 2 with its own dialogues
//         let mut location2_dialogues = HashMap::new();
//         location2_dialogues.insert(
//             "Start".to_string(),
//             Dialogue {
//                 intro: "You are now in a quiet garden.".to_string(),
//                 options: vec![
//                     DialogueOption {
//                         description: "Sit and relax.".to_string(),
//                         challenge_attribute: None,
//                         challenge_number: None,
//                         success_dialogue: None,
//                         failure_dialogue: None,
//                     },
//                     DialogueOption {
//                         description: "Return to Room 1.".to_string(),
//                         challenge_attribute: None,
//                         challenge_number: None,
//                         success_dialogue: Some("Room1".to_string()),
//                         failure_dialogue: None,
//                     },
//                 ],
//                 is_hidden: false,
//             },
//         );

//         // Define Location 2
//         locations.insert(
//             "Room2".to_string(),
//             Location {
//                 name: "Room 2 (Garden)".to_string(),
//                 dialogues: location2_dialogues,
//                 exits: vec!["Room1".to_string()],
//             },
//         );

//         Self {
//             current_text: "Welcome!".to_string(),
//             player: Player {
//                 strength: 10,
//                 wisdom: 8,
//                 knowledge: 7,
//             },
//             locations,
//             current_location_id: "Room1".to_string(), // Start in Room 1
//             current_dialogue_id: Some("Start".to_string()), // Start with the "Start" dialogue
//         }
//     }
// }

// // Implement eframe::App
// impl eframe::App for DialogueApp {
//     fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
//         egui::CentralPanel::default().show(ctx, |ui| {
//             // Clone the current dialogue ID and location ID outside of the borrow scope
//             let current_dialogue_id_clone = self.current_dialogue_id.clone();
//             let current_location_id_clone = self.current_location_id.clone();

//             // Handle dialogue logic
//             if let Some(current_dialogue_id) = &current_dialogue_id_clone {
//                 // Get the current dialogue based on the cloned ID (avoid borrowing self)
//                 if let Some(current_dialogue) = self.get_current_dialogue_from_id(current_dialogue_id) {
//                     ui.heading(&current_dialogue.intro);

//                     // Prepare variables for deferred assignment
//                     let mut new_dialogue_id = None;
//                     let mut new_location_id = None;

//                     // Display dialogue options
//                     for option in current_dialogue.options.iter() {
//                         if ui.button(&option.description).clicked() {
//                             if option.challenge_number.is_some() {
//                                 let success = handle_challenge(&self.player, option);
//                                 new_dialogue_id = if success {
//                                     option.success_dialogue.clone()
//                                 } else {
//                                     option.failure_dialogue.clone()
//                                 };
//                             } else if let Some(success_dialogue) = &option.success_dialogue {
//                                 // Handle non-challenge option that leads to another dialogue or room
//                                 if self.locations.contains_key(success_dialogue) {
//                                     // Prepare to move to another location
//                                     new_location_id = Some(success_dialogue.clone());
//                                     new_dialogue_id = None;
//                                 } else {
//                                     new_dialogue_id = Some(success_dialogue.clone());
//                                 }
//                             }
//                         }
//                     }

//                     // Apply deferred assignments outside of the borrow scope
//                     if let Some(new_id) = new_dialogue_id {
//                         self.current_dialogue_id = Some(new_id);
//                     }
//                     if let Some(new_location) = new_location_id {
//                         self.current_location_id = new_location;
//                     }
//                 }
//             } else {
//                 // Handle location exit logic
//                 ui.heading(&format!("You are in {}", self.get_current_location().name));

//                 // Prepare variable for deferred assignment
//                 let mut new_location_id = None;

//                 ui.label("Exits:");
//                 for exit in &self.get_current_location().exits {
//                     if ui.button(exit).clicked() {
//                         new_location_id = Some(exit.clone());
//                     }
//                 }

//                 // Apply deferred assignment
//                 if let Some(new_location) = new_location_id {
//                     self.current_location_id = new_location;
//                     self.current_dialogue_id = Some("Start".to_string()); // Reset to "Start" dialogue in the new location
//                 }
//             }
//         });
//     }
// }

// // Helper function to get the dialogue based on the cloned dialogue ID
// impl DialogueApp {
//     fn get_current_dialogue_from_id(&self, dialogue_id: &String) -> Option<&Dialogue> {
//         if let Some(location) = self.locations.get(&self.current_location_id) {
//             return location.dialogues.get(dialogue_id);
//         }
//         None
//     }

//     // Method to get the current location based on the current_location_id
//     fn get_current_location(&self) -> &Location {
//         // Safe unwrap because we assume current_location_id always points to a valid location
//         self.locations.get(&self.current_location_id).unwrap()
//     }
// }




// // Handle challenge logic here
// fn handle_challenge(player: &Player, option: &DialogueOption) -> bool {
//     if let Some(challenge_attribute) = &option.challenge_attribute {
//         if let Some(challenge_number) = option.challenge_number {
//             let attribute_value = match challenge_attribute.as_str() {
//                 "intelligence" => player.intelligence(),
//                 "strength" => player.strength,
//                 _ => 0,
//             };

//             let (die1, die2) = roll_dice();
//             let roll_sum = die1 + die2;

//             println!("You rolled: {} + {} = {}", die1, die2, roll_sum);

//             if die1 == 6 && die2 == 6 {
//                 println!("Double sixes! Automatic success.");
//                 return true;
//             } else if die1 == 1 && die2 == 1 {
//                 println!("Double ones! Automatic failure.");
//                 return false;
//             }

//             let total = roll_sum + attribute_value;
//             if total >= challenge_number {
//                 println!("Success! You needed {}, and you got {}.", challenge_number, total);
//                 return true;
//             } else {
//                 println!("Failure. You needed {}, but you got {}.", challenge_number, total);
//                 return false;
//             }
//         }
//     }
//     false
// }

// // Sample dice roller
// fn roll_dice() -> (i32, i32) {
//     let mut rng = rand::thread_rng();
//     (rng.gen_range(1..=6), rng.gen_range(1..=6))
// }

// struct Player {
//     strength: i32,
//     wisdom: i32,
//     knowledge: i32,
// }

// impl Player {
//     fn intelligence(&self) -> i32 {
//         self.wisdom + self.knowledge
//     }
// }

// #[derive(Clone)]
// struct DialogueOption {
//     description: String,
//     challenge_attribute: Option<String>,
//     challenge_number: Option<i32>,
//     success_dialogue: Option<String>,
//     failure_dialogue: Option<String>,
// }

// #[derive(Clone)]
// struct Dialogue {
//     intro: String,
//     options: Vec<DialogueOption>,
//     is_hidden: bool,
// }

// fn main() {
//     let app = DialogueApp::default();
//     let native_options = eframe::NativeOptions::default();
//     eframe::run_native(
//         "Dialogue System",  // App name
//         native_options,     // Window options
//         Box::new(|_cc| Box::new(app)),  // Closure to create the app
//     );
// }










//////////////////////////////
//////////////////////////////




// use eframe::{egui, Frame};
// use rand::Rng; // For dice rolling
// use std::collections::HashMap;
// use std::io::{self, Write};

// #[derive(Clone)]
// struct Location {
//     name: String,
//     dialogues: HashMap<String, Dialogue>, // Dialogues inside the location
//     exits: Vec<String>,                   // Other locations you can move to from this location
// }

// struct DialogueApp {
//     current_text: String,
//     player: Player,
//     dialogues: std::collections::HashMap<String, Dialogue>,
//     current_dialogue_id: String,
// }

// // Function to run a Dialogue, returning the action to take afterward
// fn run_dialogue(dialogues: &mut HashMap<String, Dialogue>, current_dialogue_id: &str) -> Option<String> {
//     if let Some(dialogue) = dialogues.get(current_dialogue_id) {
//         // Print the dialogue intro
//         println!("{}", dialogue.intro);

//         // Show the available options, starting from 1
//         for (i, (option, _)) in dialogue.options.iter().enumerate() {
//             println!("{}: {}", i + 1, option);
//         }

//         // Read user input
//         let mut input = String::new();
//         print!("Choose an option: ");
//         io::stdout().flush().unwrap();
//         io::stdin().read_line(&mut input).expect("Failed to read input");
//         let input: usize = input.trim().parse().unwrap_or(0);

//         // Get the next action based on the user's choice
//         if let Some((_, next_id)) = dialogue.options.get(input - 1) {
//             let next_id = next_id.clone(); // Clone the `next_id` here to break the borrow
//             if next_id == "EXIT" {
//                 return Some("EXIT".to_string());
//             }

//             // Hidden dialogues remain hidden, no need to unlock them
//             return Some(next_id);
//         } else {
//             println!("Invalid choice, try again.");
//             return run_dialogue(dialogues, current_dialogue_id);
//         }
//     }

//     None
// }

// // Function to run a Location and return the next location to move to
// fn run_location(location: &mut Location) -> Option<String> {
//     println!("You are now at: {}", location.name);

//     loop {
//         let mut available_dialogues: Vec<String> = Vec::new();

//         // Collect non-hidden dialogues to avoid immutable borrow during the iteration
//         for (dialogue_id, dialogue) in &location.dialogues {
//             if !dialogue.is_hidden {
//                 available_dialogues.push(dialogue_id.clone());
//             }
//         }

//         // Present dialogue options (only show non-hidden dialogues)
//         println!("Available dialogues:");
//         for (i, dialogue_id) in available_dialogues.iter().enumerate() {
//             println!("{}: Start '{}'", i + 1, dialogue_id);
//         }

//         // Present location exits
//         println!("0: Move to another location");

//         // Choose dialogue or exit to another location
//         let mut input = String::new();
//         print!("Choose a dialogue or move to another location: ");
//         io::stdout().flush().unwrap();
//         io::stdin().read_line(&mut input).expect("Failed to read input");
//         let choice: usize = input.trim().parse().unwrap_or(0);

//         if choice == 0 {
//             // Move to another location
//             println!("You have exited the dialogue. You can move to other locations.");
//             for (i, loc) in location.exits.iter().enumerate() {
//                 println!("{}: Move to {}", i + 1, loc);
//             }

//             let mut location_input = String::new();
//             print!("Choose a location to move to: ");
//             io::stdout().flush().unwrap();
//             io::stdin().read_line(&mut location_input).expect("Failed to read input");
//             let location_choice: usize = location_input.trim().parse().unwrap_or(0);

//             if let Some(next_location) = location.exits.get(location_choice - 1) {
//                 println!("Moving to {}", next_location);
//                 return Some(next_location.clone()); // Return the next location
//             } else {
//                 println!("Invalid location choice.");
//             }
//         } else if choice > 0 && choice <= available_dialogues.len() {
//             // Get the corresponding dialogue ID based on the user's input
//             let mut current_dialogue = available_dialogues[choice - 1].clone();

//             let mut in_dialogue = true;
//             while in_dialogue {
//                 match run_dialogue(&mut location.dialogues, &current_dialogue) {
//                     Some(next_action) => {
//                         if next_action == "EXIT" {
//                             in_dialogue = false; // Exit the dialogue and return to the location
//                         } else {
//                             current_dialogue = next_action.clone(); // Clone the next_action to avoid lifetime issues
//                         }
//                     }
//                     None => in_dialogue = false, // Dialogue not found, exit
//                 }
//             }
//         } else {
//             println!("Invalid choice, try again.");
//         }
//     }
// }

// impl Default for DialogueApp {
//     fn default() -> Self {
//         let mut dialogues = std::collections::HashMap::new();

//         // Sample Dialogues
//         dialogues.insert(
//             "Start".to_string(),
//             Dialogue {
//                 intro: "You encounter a locked door.".to_string(),
//                 options: vec![
//                     DialogueOption {
//                         description: "Try to pick the lock (Dossier challenge)".to_string(),
//                         challenge_attribute: Some("dossier".to_string()),
//                         challenge_number: Some(12),
//                         success_dialogue: Some("UnlockedDoor".to_string()),
//                         failure_dialogue: Some("LockedDoor".to_string()),
//                     },
//                     DialogueOption {
//                         description: "Try to force the door open (Gizmo challenge)".to_string(),
//                         challenge_attribute: Some("gizmo".to_string()),
//                         challenge_number: Some(14),
//                         success_dialogue: Some("ForcedDoor".to_string()),
//                         failure_dialogue: Some("LockedDoor".to_string()),
//                     },
//                 ],
//                 is_hidden: false,
//             },
//         );

//         dialogues.insert(
//             "UnlockedDoor".to_string(),
//             Dialogue {
//                 intro: "The door opens silently.".to_string(),
//                 options: vec![],
//                 is_hidden: false,
//             },
//         );

//         dialogues.insert(
//             "LockedDoor".to_string(),
//             Dialogue {
//                 intro: "The door remains locked.".to_string(),
//                 options: vec![],
//                 is_hidden: false,
//             },
//         );

//         dialogues.insert(
//             "ForcedDoor".to_string(),
//             Dialogue {
//                 intro: "You force the door open with a loud crack.".to_string(),
//                 options: vec![],
//                 is_hidden: false,
//             },
//         );

//         Self {
//             current_text: "Welcome!".to_string(),
            // player: Player {
            //     tech: 2,
            //     arts: 4,
            //     bur: 1, //short for bureaucracy
            //     und: 2, //short for underworld
            //     checkmate_mod: 0,
            //     rocketry_mod: 0,
            //     pathology_mod: 0,
            //     civic_engineering_mod: 0,
            //     apparatchik_mod: 0,
            //     quota_mod: 0,
            //     robot_mod: 0,
            //     dossier_mod: 0,
            //     arts1_mod: 0,
            //     arts2_mod: 0,
            //     arts3_mod: 0,
            //     arts4_mod: 0,
            //     high_proof_mod: 0,
            //     prohibition_mod: 0,
            //     gizmo_mod: 0,
            //     oldtime_religion_mod: 0,
            // },
//             dialogues,
//             current_dialogue_id: "Start".to_string(),
//         }
//     }
// }

// // Use `eframe::App` directly without `epi`
// impl eframe::App for DialogueApp {
//     fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
//         egui::CentralPanel::default().show(ctx, |ui| {
//             // Display current dialogue intro text
//             if let Some(current_dialogue) = self.dialogues.get(&self.current_dialogue_id) {
//                 ui.heading(&current_dialogue.intro);

//                 // Display options
//                 for option in current_dialogue.options.iter() {
//                     if ui.button(&option.description).clicked() {
//                         let success = handle_challenge(&self.player, option);
//                         self.current_dialogue_id = if success {
//                             option.success_dialogue.clone().unwrap_or_else(|| self.current_dialogue_id.clone())
//                         } else {
//                             option.failure_dialogue.clone().unwrap_or_else(|| self.current_dialogue_id.clone())
//                         };
//                     }
//                 }
//             }
//         });
//     }
// }

// // Handle challenge logic here
// fn handle_challenge(player: &Player, option: &DialogueOption) -> bool {
//     if let Some(challenge_attribute) = &option.challenge_attribute {
//         if let Some(challenge_number) = option.challenge_number {
            // let attribute_value = match challenge_attribute.as_str() {

            //     "checkmate" => player.checkmate(),
            //     "rocketry" => player.rocketry(),
            //     "pathology" => player.pathology(),
            //     "civic engineering" => player.civic_engineering(),
            //     "apparatchik" => player.apparatchik(),
            //     "quota" => player.quota(),
            //     "robot" => player.robot(),
            //     "dossier" => player.dossier(),
            //     "arts1" => player.arts1(),
            //     "arts2" => player.arts2(),
            //     "arts3" => player.arts3(),
            //     "arts4" => player.arts4(),
            //     "high proof" => player.high_proof(),
            //     "prohibition" => player.prohibition(),
            //     "gizmo" => player.gizmo(),
            //     "oldtime religion" => player.oldtime_religion(),
            //     _ => 0,
            // };

//             let (die1, die2) = roll_dice();
//             let roll_sum = die1 + die2;

//             println!("You rolled: {} + {} = {}", die1, die2, roll_sum);

//             // Special rules for double sixes (auto success) and double ones (auto failure)
//             if die1 == 6 && die2 == 6 {
//                 println!("Double sixes! Automatic success.");
//                 return true;
//             } else if die1 == 1 && die2 == 1 {
//                 println!("Double ones! Automatic failure.");
//                 return false;
//             }

//             // Check if the roll + attribute meets or exceeds the challenge number
//             let total = roll_sum + attribute_value;
//             if total >= challenge_number {
//                 println!("Success! You needed {}, and you got {}.", challenge_number, total);
//                 return true;
//             } else {
//                 println!("Failure. You needed {}, but you got {}.", challenge_number, total);
//                 return false;
//             }
//         }
//     }
//     false
// }

// // Sample dice roller
// fn roll_dice() -> (i32, i32) {
//     let mut rng = rand::thread_rng();
//     (rng.gen_range(1..=6), rng.gen_range(1..=6))
// }

// struct Player {
//     tech: i32,
//     arts: i32,
//     bur: i32, //short for bureaucracy
//     und: i32, //short for underworld
//     checkmate_mod: i32,
//     rocketry_mod: i32,
//     pathology_mod: i32,
//     civic_engineering_mod: i32,
//     apparatchik_mod: i32,
//     quota_mod: i32,
//     robot_mod: i32,
//     dossier_mod: i32,
//     arts1_mod: i32,
//     arts2_mod: i32,
//     arts3_mod: i32,
//     arts4_mod: i32,
//     high_proof_mod: i32,
//     prohibition_mod: i32,
//     gizmo_mod: i32,
//     oldtime_religion_mod: i32
// }

// impl Player {
//     fn checkmate(&self) -> i32 {
//         self.tech + self.checkmate_mod
//     }

//     fn rocketry(&self) -> i32 {
//         self.tech + self.rocketry_mod
//     }

//     fn pathology(&self) -> i32 {
//         self.tech + self.pathology_mod
//     }

//     fn civic_engineering(&self) -> i32 {
//         self.tech + self.civic_engineering_mod
//     }

//     fn apparatchik(&self) -> i32 {
//         self.bur + self.apparatchik_mod
//     }

//     fn quota(&self) -> i32 {
//         self.bur + self.quota_mod
//     }

//     fn robot(&self) -> i32 {
//         self.bur + self.robot_mod
//     }

//     fn dossier(&self) -> i32 {
//         self.bur + self.dossier_mod
//     }

//     fn arts1(&self) -> i32 {
//         self.arts + self.arts1_mod
//     }

//     fn arts2(&self) -> i32 {
//         self.arts + self.arts2_mod
//     }

//     fn arts3(&self) -> i32 {
//         self.arts + self.arts3_mod
//     }

//     fn arts4(&self) -> i32 {
//         self.arts + self.arts4_mod
//     }

//     fn high_proof(&self) -> i32 {
//         self.und + self.high_proof_mod
//     }

//     fn prohibition(&self) -> i32 {
//         self.und + self.prohibition_mod
//     }

//     fn gizmo(&self) -> i32 {
//         self.und + self.gizmo_mod
//     }

//     fn oldtime_religion(&self) -> i32 {
//         self.und + self.oldtime_religion_mod
//     }
// }

// #[derive(Clone)]
// struct DialogueOption {
//     description: String,
//     challenge_attribute: Option<String>,
//     challenge_number: Option<i32>,
//     success_dialogue: Option<String>,
//     failure_dialogue: Option<String>,
// }

// #[derive(Clone)]
// struct Dialogue {
//     intro: String,
//     options: Vec<DialogueOption>,
//     is_hidden: bool,
// }

// fn main() {
//     let app = DialogueApp::default();
//     let native_options = eframe::NativeOptions::default();
//     eframe::run_native(
//         "Shadow Soldiers",  // App name
//         native_options,     // Window options
//         Box::new(|_cc| Box::new(app)),  // Closure to create the app
//     );
// }
