use eframe::{egui, Frame};
use rand::Rng;
use std::collections::HashMap;
use std::collections::HashSet;

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
    state: GameState,
    previous_dialogue_id: Option<String>,
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
                intro: "The front door swings shut, cutting off the bitter wind like a scythe. You stand in the harsh light of a public apartment vestibule. A grandfather clock stands stout against the wall, like an elderly servant whose crooked back can't quite stand up to attention.".to_string(),
                options: vec![
                    DialogueOption {
                        description: "Inspect the grandfather clock.".to_string(),
                        challenge_attribute: None,
                        challenge_number: None,
                        success_dialogue: Some("InspectClock".to_string()),
                        failure_dialogue: None,
                        item_to_pickup: None,
                    },
                    DialogueOption {
                        description: "Go to the first floor.".to_string(),
                        challenge_attribute: None,
                        challenge_number: None,
                        success_dialogue: Some("FirstFloor".to_string()),
                        failure_dialogue: None,
                        item_to_pickup: None,
                    },
                ],
                passive_check: vec![],
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
                        item_to_pickup: None,

                        // hungry for the gear? it wants to eat its beating heart. Can give you a hint of where to find it
                    },

                    //to go back to the room's origin, the code is 'Start'

                    DialogueOption {
                        description: "Check the time.".to_string(), 
                        challenge_attribute: None,
                        challenge_number: None,
                        success_dialogue: Some("BrokenClock".to_string()), 
                        failure_dialogue: None,
                        item_to_pickup: None,
                    },

                    DialogueOption {
                        description: "Goodbye, fair clock. (End conversation).".to_string(), 
                        challenge_attribute: None,
                        challenge_number: None,
                        success_dialogue: Some("Start".to_string()), 
                        failure_dialogue: None,
                        item_to_pickup: None,
                    },
                ],
                passive_check: vec![
                    PassiveCheck {
                        skill: "robot".to_string(),
                        target: 10,
                        success_text: Some("You know its kind. The unrelenting metronome to which you dance.".to_string()),
                        failure_text: None,
                        speaker: Some("Robot".to_string())
                    },
                    PassiveCheck {
                        skill: "gizmo".to_string(),
                        target: 12,
                        success_text: Some("It's in bad shape, boss. The varnish is falling off, the face needs a solid wipe down, 
                        and I don't see a notice of last maintenance *anywhere*.".to_string()),
                        failure_text: None,
                        speaker: Some("Gizmo".to_string())
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
                        item_to_pickup: None,
                    },
                ],
                passive_check: vec![],
                is_hidden: true,
            }
        );

        vestibule_dialogues.insert(
            "BrokenClock".to_string(),
            Dialogue {
                speaker: "Grandfather Clock".to_string(),
                intro: r"Its two hands, the shorter ending in a stylized sun and the longer in a crescent moon, are stuck at 5 hours and 37 minutes".to_string(), // this is a CLUE
                options: vec![
                    DialogueOption {
                        description: "What a waste. Surely I could fix it up?".to_string(),
                        challenge_attribute: None,
                        challenge_number: None,
                        success_dialogue: Some("ClockRecommends".to_string()), //gizmo to fix it, or telling you to open it up to do so
                        failure_dialogue: None,
                        item_to_pickup: None,
                    },

                    DialogueOption {
                        description: "Five-thirty-seven in the morning or night?".to_string(),
                        challenge_attribute: None,
                        challenge_number: None,
                        success_dialogue: Some("ClockFutile".to_string()), //futilely ask if this is morning or night
                        failure_dialogue: None,
                        item_to_pickup: None,
                    },

                    DialogueOption {
                        description: "Stand proud, fallen soldier of the Republic! (Salute the clock)".to_string(),
                        challenge_attribute: None,
                        challenge_number: None,
                        success_dialogue: Some("ClockSalute".to_string()), //salute it
                        failure_dialogue: None,
                        item_to_pickup: None,
                    },

                    DialogueOption {
                        description: "Goodbye, mysterious clock.".to_string(),
                        challenge_attribute: None,
                        challenge_number: None,
                        success_dialogue: Some("Start".to_string()), //exit
                        failure_dialogue: None,
                        item_to_pickup: None,
                    },
                ],
                passive_check: vec![],
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
                intro: "The oak casing is stiff, the hinges squeaky from disuse. It takes a solid heave with both hands to pull it out, 
                and the cover nearly comes off in your grasp. Inside, you see a forest of gears, exposed to the eye and glimmering 
                under a thin coat of dust. On the bottom, resting against the casing, lies a tin model airplane.".to_string(),
                options: vec![
                    DialogueOption {
                        description: "What kind of plane is it? (Rocketry 6)".to_string(),
                        challenge_attribute: Some("rocketry".to_string()),
                        challenge_number: Some(6),
                        success_dialogue: Some("ClockPlane".to_string()),
                        failure_dialogue: Some("ClockPlaneFail".to_string()),
                        item_to_pickup: None,
                    },

                    DialogueOption {
                        description: "Let's fix you up, soldier. (Gizmo 12)".to_string(),
                        challenge_attribute: Some("gizmo".to_string()),
                        challenge_number: Some(12),
                        success_dialogue: Some("ClockMissingGear".to_string()),
                        failure_dialogue: Some("ClockFixImpossible".to_string()),
                        item_to_pickup: None,
                    },

                    DialogueOption {
                        description: "Close the casing.".to_string(),
                        challenge_attribute: None,
                        challenge_number: None,
                        success_dialogue: Some("InspectClock".to_string()),
                        failure_dialogue: None,
                        item_to_pickup: None,
                    },
                ],
                passive_check: vec![],
                is_hidden: true,
            }
        );

        vestibule_dialogues.insert(
            "MockingClock".to_string(),
            Dialogue {
                speaker: "Grandfather Clock".to_string(),
                intro: r"It seems to talk to you. It says 'Why are you talking to a clock, you absolute dolt? You twat. Has my non-sentient existence come to the point that madmen come in off the street and try to engage me in conversation?'".to_string(),
                options: vec![
                    DialogueOption {
                        description: "Okay, that's just, like, your opinion, man. (End conversation)".to_string(),
                        challenge_attribute: None,
                        challenge_number: None,
                        success_dialogue: Some("Start".to_string()), // Go back to the room's main dialogue
                        failure_dialogue: None,
                        item_to_pickup: None,
                    },

                    DialogueOption {
                        description: "Tell me your secrets!".to_string(),
                        challenge_attribute: None,
                        challenge_number: None,
                        success_dialogue: Some("ClockDecline".to_string()), // Go back to the room's main dialogue
                        failure_dialogue: None,
                        item_to_pickup: None,
                    },
                ],
                passive_check: vec![],
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
                        item_to_pickup: None,
                    },

                    DialogueOption {
                        description: "Fuck you, and your little cuckoo too.".to_string(),
                        challenge_attribute: None,
                        challenge_number: None,
                        success_dialogue: Some("Start".to_string()), // Go back to the room's main dialogue
                        failure_dialogue: None,
                        item_to_pickup: None,
                    },
                ],
                passive_check: vec![],
                is_hidden: true,
            }
        );

        vestibule_dialogues.insert(
            "ClockRecommends".to_string(),
            Dialogue {
                speaker: "Grandfather Clock".to_string(),
                intro: r"You'd have to get into this thing's guts. Are you mentally and *spiritually* prepared to mess around with this poor, fallen soldier's insides? On the off chance you can restore life to the dead?".to_string(),
                options: vec![
                    DialogueOption {
                        description: "On second thought, let's not do that.".to_string(),
                        challenge_attribute: None,
                        challenge_number: None,
                        success_dialogue: Some("BrokenClock".to_string()), 
                        failure_dialogue: None,
                        item_to_pickup: None,
                    },

                    DialogueOption {
                        description: "Come on, why did you have to phrase it that way?".to_string(),
                        challenge_attribute: None,
                        challenge_number: None,
                        success_dialogue: Some("ClockPhrasing".to_string()), 
                        failure_dialogue: None,
                        item_to_pickup: None,
                    },

                    DialogueOption {
                        description: "Let's do this.".to_string(),
                        challenge_attribute: None,
                        challenge_number: None,
                        success_dialogue: Some("ClockInterior".to_string()), // Go back to the room's main dialogue
                        failure_dialogue: None,
                        item_to_pickup: None,
                    },
                ],
                passive_check: vec![],
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
                        item_to_pickup: None,
                    },

                    DialogueOption {
                        description: "Like this is some *religious* thing. That's not what I'm about.".to_string(),
                        challenge_attribute: None,
                        challenge_number: None,
                        success_dialogue: Some("ReligiousClock".to_string()), 
                        failure_dialogue: None,
                        item_to_pickup: None,
                    },

                    DialogueOption {
                        description: "Like... you know.".to_string(),
                        challenge_attribute: None,
                        challenge_number: None,
                        success_dialogue: Some("ClockSuggestive".to_string()), 
                        failure_dialogue: None,
                        item_to_pickup: None,
                    },

                    DialogueOption {
                        description: "You know what, never mind.".to_string(),
                        challenge_attribute: None,
                        challenge_number: None,
                        success_dialogue: Some("BrokenClock".to_string()), 
                        failure_dialogue: None,
                        item_to_pickup: None,
                    },

                    DialogueOption {
                        description: "Fine, I'll get *into its guts*. What's in there?".to_string(),
                        challenge_attribute: None,
                        challenge_number: None,
                        success_dialogue: Some("ClockInterior".to_string()), 
                        failure_dialogue: None,
                        item_to_pickup: None,
                    },
                ],
                passive_check: vec![],
                is_hidden: true,
            }
        );

        vestibule_dialogues.insert(
            "JustABrokenClock".to_string(),
            Dialogue {
                speaker: "Grandfather Clock".to_string(),
                intro: "*Just* a broken clock? Shows what you know. This right here is a veteran of public administration, an honored servant of the people, languishing under the failures of the regime. How dare you ignore his deeds and his suffering.".to_string(),
                options: vec![
                    DialogueOption {
                        description: "Well, Grandfather Clockovitch here can languish a little longer.".to_string(),
                        challenge_attribute: None,
                        challenge_number: None,
                        success_dialogue: Some("BrokenClock".to_string()), // Go back to the room's main dialogue
                        failure_dialogue: None,
                        item_to_pickup: None,
                    },

                    DialogueOption {
                        description: "Fine, I'll look inside. As long as there's no more creepy phrasing.".to_string(),
                        challenge_attribute: None,
                        challenge_number: None,
                        success_dialogue: Some("ClockInterior".to_string()), // Go back to the room's main dialogue
                        failure_dialogue: None,
                        item_to_pickup: None,
                    },
                ],
                passive_check: vec![],
                is_hidden: true,
            }
        );

        vestibule_dialogues.insert(
            "ReligiousClock".to_string(),
            Dialogue {
                speaker: "Grandfather Clock".to_string(),
                intro: "Religious? What are you talking about? Grandfather Clockovitch is a ".to_string(),
                options: vec![
                    DialogueOption {
                        description: "".to_string(),
                        challenge_attribute: None,
                        challenge_number: None,
                        success_dialogue: Some("Start".to_string()), // Go back to the room's main dialogue
                        failure_dialogue: None,
                        item_to_pickup: None,
                    },
                ],
                passive_check: vec![],
                is_hidden: true, //grandfather clockovitch is a secret religionist! asks that you forgive him anyway. Will you hold back your generosity from this sinne- I mean, reactionary?
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
                        item_to_pickup: None,
                    },

                    DialogueOption {
                        description: "Fine, I'll leave off. What was that about a sun and moon again?".to_string(),
                        challenge_attribute: None,
                        challenge_number: None,
                        success_dialogue: Some("BrokenClock".to_string()), 
                        failure_dialogue: None,
                        item_to_pickup: None,
                    },
                ],
                passive_check: vec![],
                is_hidden: true,
            }
        );

        vestibule_dialogues.insert(
            "ClockSexual".to_string(),
            Dialogue {
                speaker: "Grandfather Clock".to_string(),
                intro: r"Wow. Just wow. Just what about sticking your hands *wrist deep* in your comrade's bloody, pulsating wound seems *sexual* to you?".to_string(),
                options: vec![
                    DialogueOption {
                        description: r"It's not you, it's everyone. Come up with a theoretical justification for your objectively weird thoughts. ".to_string(),
                        challenge_attribute: Some("pathology".to_string()),
                        challenge_number: Some(8),
                        success_dialogue: Some("ClockPerversion".to_string()), //Pathology check to justify yourself
                        failure_dialogue: Some("ClockTrauma".to_string()),
                        item_to_pickup: None,
                    },

                    DialogueOption {
                        description: "There it is again! You're putting this weird *emphasis* on things.".to_string(),
                        challenge_attribute: None,
                        challenge_number: None,
                        success_dialogue: Some("ClockEmphasis".to_string()), 
                        failure_dialogue: None,
                        item_to_pickup: None,
                    },

                    DialogueOption {
                        description: "Actually, you know what? This isn't a sexual thing, it's a religious thing.".to_string(),
                        challenge_attribute: None,
                        challenge_number: None,
                        success_dialogue: Some("ClockSexualReligion".to_string()), 
                        failure_dialogue: None,
                        item_to_pickup: None,
                    },
                ],
                passive_check: vec![],
                is_hidden: true,
            }
        );

        vestibule_dialogues.insert(
            "ClockPerversion".to_string(),
            Dialogue {
                speaker: "Grandfather Clock".to_string(),
                intro: r"It's an extremely common and not at all shameful perversion, characteristic of the Republic's struggles against the insidious American *Central Intelligence Agency*.".to_string(),
                options: vec![
                    DialogueOption {
                        description: "Let's go with that.".to_string(),
                        challenge_attribute: None,
                        challenge_number: None,
                        success_dialogue: Some("ClockCIA".to_string()), // Go back to the room's main dialogue
                        failure_dialogue: None,
                        item_to_pickup: None,
                    },

                    DialogueOption {
                        description: "No, you can't possibly be serious. How can that possibly make sense?".to_string(),
                        challenge_attribute: None,
                        challenge_number: None,
                        success_dialogue: Some("ClockExplanation".to_string()), // Go back to the room's main dialogue
                        failure_dialogue: None,
                        item_to_pickup: None,
                    },
                ],
                passive_check: vec![],
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
                        item_to_pickup: None,
                    },

                    DialogueOption {
                        description: "Did I have to do... that... during The War? Do I have repressed memories I need to process?".to_string(),
                        challenge_attribute: None,
                        challenge_number: None,
                        success_dialogue: Some("ClockRepression".to_string()), // Go back to the room's main dialogue
                        failure_dialogue: None,
                        item_to_pickup: None,
                    },
                ],
                passive_check: vec![],
                is_hidden: true,
            }
        );

        vestibule_dialogues.insert(
            "ClockNotTrauma".to_string(),
            Dialogue {
                speaker: "Grandfather Clock".to_string(),
                intro: r"Look, all I now is that whenever something like this happens to you, there's exactly one phrase bouncing around this empty skull, and it's 'trauma response'. Trauma response this, trauma response that. It gets you out of anything.".to_string(),
                options: vec![
                    DialogueOption {
                        description: "".to_string(),
                        challenge_attribute: None,
                        challenge_number: None,
                        success_dialogue: Some("".to_string()), 
                        failure_dialogue: None,
                        item_to_pickup: None,
                    },
                ],
                passive_check: vec![],
                is_hidden: true,
            }
        );

        vestibule_dialogues.insert(
            "ClockRepression".to_string(),
            Dialogue {
                speaker: "Grandfather Clock".to_string(),
                intro: r"Something here is definitely repressed, but it's not memories. You're no amnesiac, as much as you might like to be, at times.".to_string(),
                options: vec![
                    DialogueOption {
                        description: "".to_string(),
                        challenge_attribute: None,
                        challenge_number: None,
                        success_dialogue: Some("".to_string()), 
                        failure_dialogue: None,
                        item_to_pickup: None,
                    },
                ],
                passive_check: vec![],
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
                        item_to_pickup: None,
                    },
                ],
                passive_check: vec![],
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
                        description: "You know, somehow that fails to make me okay with all of this.".to_string(),
                        challenge_attribute: None,
                        challenge_number: None,
                        success_dialogue: Some("".to_string()), 
                        failure_dialogue: None,
                        item_to_pickup: None,
                    },

                    DialogueOption {
                        description: "".to_string(),
                        challenge_attribute: None,
                        challenge_number: None,
                        success_dialogue: Some("".to_string()), 
                        failure_dialogue: None,
                        item_to_pickup: None,
                    },

                    DialogueOption {
                        description: "If this is what communism has come to, shave my mustache and call me Milton Friedman. (Leave in disgust)".to_string(),
                        challenge_attribute: None,
                        challenge_number: None,
                        success_dialogue: Some("Start".to_string()), 
                        failure_dialogue: None,
                        item_to_pickup: None,
                    },
                ],
                passive_check: vec![],
                is_hidden: true,
            }
        );

        vestibule_dialogues.insert(
            "ClockMissingGear".to_string(),
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
                        item_to_pickup: None,
                    },
                ],
                passive_check: vec![],
                is_hidden: true,
            }
        );

        vestibule_dialogues.insert(
            "ClockFixImpossible".to_string(),
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
                        item_to_pickup: None,
                    },
                ],
                passive_check: vec![],
                is_hidden: true,
            }
        );

        vestibule_dialogues.insert(
            "ClockPlane".to_string(),
            Dialogue {
                speaker: "SU-25 Grach".to_string(),
                intro: "This is the Sukhoi SU-25 Grach, a single-seat twin-engine close air support jet, most notable for its extensive counter-insurgency missions in Afghanistan.".to_string(),
                options: vec![
                    DialogueOption {
                        description: "Pick it up".to_string(),
                        challenge_attribute: None,
                        challenge_number: None,
                        success_dialogue: Some("ClockInterior".to_string()), 
                        failure_dialogue: None,
                        item_to_pickup: Some("Sukhoi SU-25 Grach model".to_string()),
                    },
                    DialogueOption {
                        description: "It's only a model (Return)".to_string(),
                        challenge_attribute: None,
                        challenge_number: None,
                        success_dialogue: Some("ClockInterior".to_string()), // Go back to the room's main dialogue
                        failure_dialogue: None,
                        item_to_pickup: None,
                    },
                ],
                passive_check: vec![],
                is_hidden: true,
            }
        );

        // add it's only a model passive dialogue

        vestibule_dialogues.insert(
            "ClockPlaneFail".to_string(),
            Dialogue {
                speaker: "Toy Plane".to_string(),
                intro: "It's just a toy plane. Looks cool though.".to_string(),
                options: vec![
                    DialogueOption {
                        description: "Pick it up".to_string(),
                        challenge_attribute: None,
                        challenge_number: None,
                        success_dialogue: Some("ClockInterior".to_string()), // Go back to the room's main dialogue
                        failure_dialogue: None,
                        item_to_pickup: Some("Toy Plane".to_string()),
                    },

                    DialogueOption {
                        description: "(Return)".to_string(),
                        challenge_attribute: None,
                        challenge_number: None,
                        success_dialogue: Some("ClockInterior".to_string()), // Go back to the room's main dialogue
                        failure_dialogue: None,
                        item_to_pickup: None,
                    },
                ],
                passive_check: vec![],
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
                        item_to_pickup: None,
                    },
                ],
                passive_check: vec![],
                is_hidden: true,
            }
        );


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
                        item_to_pickup: None,
                    },
                    DialogueOption {
                        description: "Try to open the stuck door (Dossier challenge).".to_string(),
                        challenge_attribute: Some("dossier".to_string()),
                        challenge_number: Some(12),
                        success_dialogue: Some("Garden".to_string()),
                        failure_dialogue: Some("FailedDoor".to_string()),
                        item_to_pickup: None,
                    },
                    DialogueOption {
                        description: "Go back to the vestibule.".to_string(),
                        challenge_attribute: None,
                        challenge_number: None,
                        success_dialogue: Some("Vestibule".to_string()),
                        failure_dialogue: None,
                        item_to_pickup: None,
                    },
                ],
                passive_check: vec![],
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
                        item_to_pickup: None,
                    },
                ],
                passive_check: vec![],
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
                        item_to_pickup: None,
                    },
                ],
                passive_check: vec![],
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
                        item_to_pickup: None,
                    },
                    DialogueOption {
                        description: "Go back to the first floor.".to_string(),
                        challenge_attribute: None,
                        challenge_number: None,
                        success_dialogue: Some("FirstFloor".to_string()),
                        failure_dialogue: None,
                        item_to_pickup: None,
                    },
                ],
                passive_check: vec![],
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
                        item_to_pickup: None,
                    },
                ],
                passive_check: vec![],
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
                items: vec![],
            },
            locations,
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
                egui::CentralPanel::default().show(ctx, |ui| {
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
                // In-game logic here
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.heading("In-Game Dialogue");
                    let current_dialogue_id_clone = self.current_dialogue_id.clone();
            
                    let mut new_dialogue_id = None;
                    let mut new_location_id = None;
                    let mut options_to_remove = vec![];
                    let mut items_to_add = vec![]; // Store items to be added after the immutable borrow ends
            
                    if let Some(current_dialogue_id) = &current_dialogue_id_clone {
                        if let Some(current_dialogue) = self.get_current_dialogue_from_id(current_dialogue_id) {

                            // Check if the player has already entered this dialogue
                            if !self.player.dialogues_entered.contains(current_dialogue_id) {
                                // Award XP if the dialogue has an XP reward and this is the first time entering
                                if let Some(xp_reward) = current_dialogue.xp_reward {
                                    self.player.add_xp(xp_reward);
                                    println!("You gained {} XP!", xp_reward);
                                }
                                // Mark the dialogue as entered
                                self.player.dialogues_entered.insert(current_dialogue_id.clone());
                            }

                            // Handle multiple passive checks if they exist
                            for passive_check in &current_dialogue.passive_check {
                                // Perform the skill check
                                let player_skill_value = self.get_player_skill(&passive_check.skill) + 6;
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
            
                            // Display the speaker's name before the dialogue
                            ui.heading(&format!("{} ", current_dialogue.speaker));
            
                            // Display the dialogue
                            ui.heading(&current_dialogue.intro);
            
                            for (i, option) in current_dialogue.options.iter().enumerate() {
                                if ui.button(&option.description).clicked() {
                                    // Clone the item to be picked up to avoid immutable borrow conflicts
                                    if let Some(item) = &option.item_to_pickup {
                                        items_to_add.push(item.clone());  // Add item to be processed later
                                        options_to_remove.push(i);        // Remove option after it's clicked
                                    }
            
                                    if option.challenge_number.is_some() {
                                        let success = handle_challenge(&self.player, option);
                                        if success {
                                            new_dialogue_id = option.success_dialogue.clone();
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
            
                            // Now that we are outside the immutable borrow of `current_dialogue`, we can safely mutate `self.player.items`
                            for item in items_to_add {
                                self.player.items.push(item);
                            }
            
                            if !options_to_remove.is_empty() {
                                let dialogue = self.get_current_dialogue_from_id_mut(current_dialogue_id).unwrap();
                                for &index in options_to_remove.iter().rev() {
                                    dialogue.options.remove(index);  // Remove the option in reverse order to avoid shifting indices
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
            
                    // Add the "View Inventory" button
                    if ui.button("View Inventory").clicked() {
                        self.previous_dialogue_id = self.current_dialogue_id.clone();
                        self.state = GameState::InventoryView;
                    }
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

    fn get_current_location(&self) -> &Location {
        self.locations.get(&self.current_location_id).unwrap()
    }

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

    fn display_current_dialogue(&mut self, ui: &mut egui::Ui) {
        if let Some(current_dialogue_id) = &self.current_dialogue_id {
            if let Some(current_dialogue) = self.get_current_dialogue_from_id(current_dialogue_id) {
                ui.heading(&format!("{} says:", current_dialogue.speaker));
                ui.heading(&current_dialogue.intro);

                let mut new_dialogue_id = None;
                for option in current_dialogue.options.iter() {
                    if ui.button(&option.description).clicked() {
                        new_dialogue_id = option.success_dialogue.clone();
                    }
                }

                if let Some(new_id) = new_dialogue_id {
                    self.current_dialogue_id = Some(new_id);
                }
            }
        } else {
            ui.label("No dialogue available.");
        }
    }

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

enum GameState {
    CharacterCreation,
    InGame,
    InventoryView,
}

#[derive(Clone)]
struct DialogueOption {
    description: String,
    challenge_attribute: Option<String>,
    challenge_number: Option<i32>,
    success_dialogue: Option<String>,
    failure_dialogue: Option<String>,
    item_to_pickup: Option<String>,
}

impl Default for DialogueOption {
    fn default() -> Self {
        DialogueOption {
            description: "Continue".to_string(),
            challenge_attribute: None,
            challenge_number: None,
            success_dialogue: Some("Start".to_string()),
            failure_dialogue: None,
            item_to_pickup: None,
        }
    }
}

#[derive(Clone)]
struct Dialogue {
    speaker: String,
    intro: String,
    options: Vec<DialogueOption>,
    passive_check: Vec<PassiveCheck>, // New field for passive dialogue checks
    xp_reward: Option<i32>,
    is_hidden: bool,
}

#[derive(Clone)]
struct PassiveCheck {
    skill: String,          // The player's skill to check
    target: i32,            // The number to check against
    success_text: Option<String>, // Text to display on success (Optional)
    failure_text: Option<String>, // Text to display on failure (Optional)
    speaker: Option<String>, // The speaker, who will be the same in both success and failure cases
}

impl Default for Dialogue {
    fn default() -> Self {
        Dialogue {
            speaker: "Error".to_string(),
            intro: "No dialogue available.".to_string(),
            options: vec![
                DialogueOption::default(),
            ],
            passive_check: vec![],
            is_hidden: true,
        }
    }
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

