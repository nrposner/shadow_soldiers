use std::collections::HashMap;

#[derive(Clone)]
pub struct Location {
    pub name: String,
    pub dialogues: HashMap<String, Dialogue>,
    pub exits: Vec<String>, // Names of other locations you can move to
}

#[derive(Clone)]
pub struct DialogueOption {
    pub description: String,
    pub challenge_attribute: Option<String>,
    pub challenge_number: Option<i32>,
    pub success_dialogue: Option<String>,
    pub failure_dialogue: Option<String>,
    pub item_to_pickup: Option<String>,
    pub visible_when: Option<String>,
    pub flags: Option<Vec<String>>,

}

#[derive(Clone)]
pub struct Dialogue {
    pub speaker: String,
    pub intro: String,
    pub options: Vec<DialogueOption>,
    pub passive_check: Vec<PassiveCheck>, // New field for passive dialogue checks
    pub xp_reward: Option<i32>,
    pub is_hidden: bool,
    pub time: Option<i32>,
}

#[derive(Clone)]
pub struct PassiveCheck {
    pub skill: String,          // The player's skill to check
    pub target: i32,            // The number to check against
    pub success_text: Option<String>, // Text to display on success (Optional)
    pub failure_text: Option<String>, // Text to display on failure (Optional)
    pub speaker: Option<String>, // The speaker, who will be the same in both success and failure cases
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
            visible_when: None,
            flags: None,
        }
    }
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
            xp_reward: None,
            is_hidden: true,
            time: Some(1),
        }
    }
}

impl Location {
    pub fn new(name: String) -> Self {
        Self {
            name,
            dialogues: HashMap::new(),
            exits: vec![],
        }
    }

    pub fn add_dialogue(&mut self, id: String, dialogue: Dialogue) {
        self.dialogues.insert(id, dialogue);
    }

    pub fn add_exit(&mut self, exit: String) {
        self.exits.push(exit);
    }
}


//create defaults and use them, reduce space taken up
















pub fn create_locations() -> HashMap<String, Location> {
    let mut locations = HashMap::new();

    // Define sample dialogues for the Vestibule
    let mut vestibule_dialogues = HashMap::new();
    vestibule_dialogues.insert(
        "Start".to_string(),
        Dialogue {
            speaker: "".to_string(),
            intro: "The front door swings shut, cutting off the bitter wind like a scythe. You stand in the harsh light of a public apartment vestibule. A grid of mailboxes wait, closed, and a grandfather clock stands stout against the wall, like an elderly servant whose crooked back can't quite stand up to attention.".to_string(),
            options: vec![
                DialogueOption {
                    description: "Inspect the grandfather clock.".to_string(),
                    success_dialogue: Some("InspectClock".to_string()),
                    ..Default::default()
                },
                DialogueOption {
                    description: "Look in the mailboxes.".to_string(),
                    success_dialogue: Some("VestibuleMailboxes".to_string()),
                    ..Default::default()
                },
                DialogueOption {
                    description: "Go to the first floor.".to_string(),
                    success_dialogue: Some("FirstFloor".to_string()),
                    ..Default::default()
                },
            ],
            is_hidden: false,
            ..Default::default()
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
                    ..Default::default()

                    // hungry for the gear? it wants to eat its beating heart. Can give you a hint of where to find it
                },

                //to go back to the room's origin, the code is 'Start'

                DialogueOption {
                    description: "Check the time.".to_string(), 
                    success_dialogue: Some("BrokenClock".to_string()), 
                    ..Default::default()
                },

                DialogueOption {
                    description: "Goodbye, fair clock. (End conversation).".to_string(), 
                    success_dialogue: Some("Start".to_string()), 
                    ..Default::default()
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
            ..Default::default()
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
                    success_dialogue: Some("".to_string()),
                    ..Default::default()
                },
            ],
            ..Default::default()
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
                    success_dialogue: Some("ClockRecommends".to_string()), //gizmo to fix it, or telling you to open it up to do so
                    ..Default::default()
                },

                DialogueOption {
                    description: "Five-thirty-seven in the morning or night?".to_string(),
                    success_dialogue: Some("ClockFutile".to_string()), //futilely ask if this is morning or night
                    ..Default::default()
                },

                DialogueOption {
                    description: "Stand proud, fallen soldier of the Republic! (Salute the clock)".to_string(),
                    success_dialogue: Some("ClockSalute".to_string()), //salute it
                    ..Default::default()
                },

                DialogueOption {
                    description: "Goodbye, mysterious clock.".to_string(),
                    success_dialogue: Some("Start".to_string()), //exit
                    ..Default::default()
                },
            ],
            ..Default::default()
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
                    ..Default::default()
                },

                DialogueOption {
                    description: "Let's fix you up, soldier. (Gizmo 12)".to_string(),
                    challenge_attribute: Some("gizmo".to_string()),
                    challenge_number: Some(12),
                    success_dialogue: Some("ClockMissingGear".to_string()),
                    failure_dialogue: Some("ClockFixImpossible".to_string()),
                    ..Default::default()
                },

                DialogueOption {
                    description: "Close the casing.".to_string(),
                    success_dialogue: Some("InspectClock".to_string()),
                    ..Default::default()
                },
            ],
            ..Default::default()
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
                    success_dialogue: Some("Start".to_string()), // Go back to the room's main dialogue
                    ..Default::default()
                },

                DialogueOption {
                    description: "Tell me your secrets!".to_string(),
                    success_dialogue: Some("ClockDecline".to_string()), // Go back to the room's main dialogue
                    ..Default::default()
                },
            ],
            ..Default::default()
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
                    success_dialogue: Some("Start".to_string()), // Go back to the room's main dialogue
                    ..Default::default()
                },

                DialogueOption {
                    description: "Fuck you, and your little cuckoo too.".to_string(),
                    success_dialogue: Some("Start".to_string()), // Go back to the room's main dialogue
                    ..Default::default()
                },
            ],
            ..Default::default()
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
                    success_dialogue: Some("BrokenClock".to_string()), 
                    ..Default::default()
                },

                DialogueOption {
                    description: "Come on, why did you have to phrase it that way?".to_string(),
                    success_dialogue: Some("ClockPhrasing".to_string()), 
                    ..Default::default()
                },

                DialogueOption {
                    description: "Let's do this.".to_string(),
                    success_dialogue: Some("ClockInterior".to_string()), // Go back to the room's main dialogue
                    ..Default::default()
                },
            ],
            ..Default::default()
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
                    success_dialogue: Some("JustABrokenClock".to_string()), 
                    ..Default::default()
                },

                DialogueOption {
                    description: "Like this is some *religious* thing. That's not what I'm about.".to_string(),
                    success_dialogue: Some("ReligiousClock".to_string()), 
                    ..Default::default()
                },

                DialogueOption {
                    description: "Like... you know.".to_string(),
                    success_dialogue: Some("ClockSuggestive".to_string()), 
                    ..Default::default()
                },

                DialogueOption {
                    description: "You know what, never mind.".to_string(),
                    success_dialogue: Some("BrokenClock".to_string()), 
                    ..Default::default()
                },

                DialogueOption {
                    description: "Fine, I'll get *into its guts*. What's in there?".to_string(),
                    success_dialogue: Some("ClockInterior".to_string()), 
                    ..Default::default()
                },
            ],
            ..Default::default()
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
                    success_dialogue: Some("BrokenClock".to_string()), // Go back to the room's main dialogue
                    ..Default::default()
                },

                DialogueOption {
                    description: "Fine, I'll look inside. As long as there's no more creepy phrasing.".to_string(),
                    success_dialogue: Some("ClockInterior".to_string()), // Go back to the room's main dialogue
                    ..Default::default()
                },
            ],
            ..Default::default()
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
                    success_dialogue: Some("Start".to_string()), // Go back to the room's main dialogue
                    ..Default::default()
                },
            ],
            //grandfather clockovitch is a secret religionist! asks that you forgive him anyway. Will you hold back your generosity from this sinne- I mean, reactionary?
            ..Default::default()
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
                    success_dialogue: Some("ClockSexual".to_string()), 
                    ..Default::default()
                },

                DialogueOption {
                    description: "Fine, I'll leave off. What was that about a sun and moon again?".to_string(),
                    success_dialogue: Some("BrokenClock".to_string()), 
                    ..Default::default()
                },
            ],
            ..Default::default()
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
                    ..Default::default()
                },

                DialogueOption {
                    description: "There it is again! You're putting this weird *emphasis* on things.".to_string(),
                    success_dialogue: Some("ClockEmphasis".to_string()), 
                    ..Default::default()
                },

                DialogueOption {
                    description: "Actually, you know what? This isn't a sexual thing, it's a religious thing.".to_string(),
                    success_dialogue: Some("ClockSexualReligion".to_string()), 
                    ..Default::default()
                },
            ],
            ..Default::default()
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
                    success_dialogue: Some("ClockCIA".to_string()), // Go back to the room's main dialogue
                    ..Default::default()
                },

                DialogueOption {
                    description: "No, you can't possibly be serious. How can that possibly make sense?".to_string(),
                    success_dialogue: Some("ClockExplanation".to_string()), // Go back to the room's main dialogue
                    ..Default::default()
                },
            ],
            ..Default::default()
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
                    success_dialogue: Some("ClockNotTrauma".to_string()), // Go back to the room's main dialogue
                    ..Default::default()
                },

                DialogueOption {
                    description: "Did I have to do... that... during The War? Do I have repressed memories I need to process?".to_string(),
                    success_dialogue: Some("ClockRepression".to_string()), // Go back to the room's main dialogue
                    ..Default::default()
                },
            ],
            ..Default::default()
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
                    success_dialogue: Some("".to_string()), 
                    ..Default::default()
                },
            ],
            ..Default::default()
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
                    success_dialogue: Some("".to_string()), 
                    ..Default::default()
                },
            ],
            ..Default::default()
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
                    success_dialogue: Some("".to_string()),
                    ..Default::default()
                },
            ],
            ..Default::default()
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
                    success_dialogue: Some("".to_string()), 
                    ..Default::default()
                },

                DialogueOption {
                    description: "".to_string(),
                    success_dialogue: Some("".to_string()), 
                    ..Default::default()
                },

                DialogueOption {
                    description: "If this is what communism has come to, shave my mustache and call me Milton Friedman. (Leave in disgust)".to_string(),
                    success_dialogue: Some("Start".to_string()), 
                    ..Default::default()
                },
            ],
            ..Default::default()
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
                    success_dialogue: Some("Start".to_string()), // Go back to the room's main dialogue
                    ..Default::default()
                },
            ],
            xp_reward: Some(10),
            ..Default::default()
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
                    success_dialogue: Some("Start".to_string()), // Go back to the room's main dialogue
                    ..Default::default()
                },
            ],
            ..Default::default()
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
                    success_dialogue: Some("ClockInterior".to_string()), 
                    item_to_pickup: Some("Sukhoi SU-25 Grach model".to_string()),
                    ..Default::default()
                },
                DialogueOption {
                    description: "It's only a model (Return)".to_string(),
                    success_dialogue: Some("ClockInterior".to_string()), // Go back to the room's main dialogue
                    ..Default::default()
                },
            ],
            xp_reward: Some(5),
            ..Default::default()
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
                    success_dialogue: Some("ClockInterior".to_string()), // Go back to the room's main dialogue
                    item_to_pickup: Some("Toy Plane".to_string()),
                    ..Default::default()
                },

                DialogueOption {
                    description: "(Return)".to_string(),
                    success_dialogue: Some("ClockInterior".to_string()), // Go back to the room's main dialogue
                    ..Default::default()
                },
            ],
            ..Default::default()
        }
    );

    vestibule_dialogues.insert(
        "VestibuleMailboxes".to_string(),
        Dialogue{
            speaker: "Mailboxes".to_string(),
            intro: "Four rows of square, wooden mailboxes sit above a dusty bar mounted to the wall.".to_string(),
            options: vec![
                DialogueOption {
                    description: "Examine the mailboxes".to_string(),
                    success_dialogue: Some("MailboxExamination".to_string()),
                    ..Default::default()
                },
                DialogueOption {
                    description: "Look at the first row".to_string(),
                    success_dialogue: Some("MailboxFirstRow".to_string()),
                    ..Default::default()
                },
                DialogueOption {
                    description: "Look at the second row".to_string(),
                    success_dialogue: Some("MailboxSecondRow".to_string()),
                    ..Default::default()
                },
                DialogueOption {
                    description: "Look at the third row".to_string(),
                    success_dialogue: Some("MailboxThirdRow".to_string()),
                    ..Default::default()
                },
                DialogueOption {
                    description: "Look at the fourth row".to_string(),
                    success_dialogue: Some("MailboxFourthRow".to_string()),
                    ..Default::default()
                },
                DialogueOption {
                    description: "Exit".to_string(),
                    success_dialogue: Some("Start".to_string()),
                    ..Default::default()
                },
            ],
            passive_check: vec![
                PassiveCheck {
                    skill: "dossier".to_string(),
                    target: 8,
                    success_text: Some("The first mailbox in the fourth row, number 400, just reads 'Administrator.' This one's yours.".to_string()),
                    failure_text: None,
                    speaker: Some("Dossier".to_string())
                },
            ],
            ..Default::default()
        }
    );

    vestibule_dialogues.insert(
        "MailboxFirstRow".to_string(),
        Dialogue {
            speaker: "Mailboxes".to_string(),
            intro: "".to_string(),
            options: vec![
                DialogueOption {
                    description: "Open 106: Kutuzov".to_string(),
                    success_dialogue: Some("Mailbox106".to_string()),
                    ..Default::default()
                },
                DialogueOption {
                    description: "Open 108: ".to_string(),
                    success_dialogue: Some("Mailbox108".to_string()),
                    ..Default::default()
                },
                DialogueOption {
                    description: "Return".to_string(),
                    success_dialogue: Some("VestibuleMailboxes".to_string()),
                    ..Default::default()
                },
            ],
            ..Default::default()
        }
    );

    vestibule_dialogues.insert(
        "MailboxSecondRow".to_string(),
        Dialogue {
            speaker: "Mailboxes".to_string(),
            intro: "".to_string(),
            options: vec![
                DialogueOption {
                    description: "Open 201: ".to_string(),
                    success_dialogue: Some("Mailbox201".to_string()),
                    ..Default::default()
                },
                DialogueOption {
                    description: "Open 205: ".to_string(),
                    success_dialogue: Some("Mailbox205".to_string()),
                    ..Default::default()
                },
                DialogueOption {
                    description: "Open 206: ".to_string(),
                    success_dialogue: Some("Mailbox206".to_string()),
                    ..Default::default()
                },
                DialogueOption {
                    description: "Open 208: ".to_string(),
                    success_dialogue: Some("Mailbox208".to_string()),
                    ..Default::default()
                },
                DialogueOption {
                    description: "Return".to_string(),
                    success_dialogue: Some("VestibuleMailboxes".to_string()),
                    ..Default::default()
                },
            ],
            ..Default::default()
        }
    );

    vestibule_dialogues.insert(
        "MailboxThirdRow".to_string(),
        Dialogue {
            speaker: "Mailboxes".to_string(),
            intro: "".to_string(),
            options: vec![
                DialogueOption {
                    description: "Open 301: ".to_string(),
                    success_dialogue: Some("Mailbox301".to_string()),
                    ..Default::default()
                },
                DialogueOption {
                    description: "Open 303: ".to_string(),
                    success_dialogue: Some("Mailbox303".to_string()),
                    ..Default::default()
                },
                DialogueOption {
                    description: "Open 304: ".to_string(),
                    success_dialogue: Some("Mailbox304".to_string()),
                    ..Default::default()
                },
                DialogueOption {
                    description: "Open 305: ".to_string(),
                    success_dialogue: Some("Mailbox305".to_string()),
                    ..Default::default()
                },
                DialogueOption {
                    description: "Open 307: ".to_string(),
                    success_dialogue: Some("Mailbox307".to_string()),
                    ..Default::default()
                },
                DialogueOption {
                    description: "Return".to_string(),
                    success_dialogue: Some("VestibuleMailboxes".to_string()),
                    ..Default::default()
                },
            ],
            ..Default::default()
        }
    );

    vestibule_dialogues.insert(
        "MailboxFourthRow".to_string(),
        Dialogue {
            speaker: "Mailboxes".to_string(),
            intro: "The lone nameplate reads: Administrator. It's clean, free of dust, and undamaged.".to_string(),
            options: vec![
                DialogueOption {
                    description: "Open 400: Administrator".to_string(),
                    success_dialogue: Some("Mailbox400".to_string()),
                    ..Default::default()
                },
                DialogueOption {
                    description: "Return".to_string(),
                    success_dialogue: Some("VestibuleMailboxes".to_string()),
                    ..Default::default()
                },

            ],
            ..Default::default()
        }
    );

    vestibule_dialogues.insert(
        "MailboxExamination".to_string(),
        Dialogue{
            speaker: "Mailboxes".to_string(),
            intro: "Most have no nameplate at all. Maybe a dozen or so apartments are actually assigned. Most of those are empty too.".to_string(),
            options: vec![
                DialogueOption {
                    description: "".to_string(),
                    success_dialogue: Some("".to_string()),
                    ..Default::default()
                },
            ],
            passive_check: vec![
                PassiveCheck {
                    skill: "robot".to_string(),
                    target: 0,
                    success_text: Some("Good. The people here are dutiful. They collect their mail on time.".to_string()),
                    failure_text: None,
                    speaker: Some("Robot".to_string())
                },

                PassiveCheck {
                    skill: "delusion".to_string(),
                    target: 10,
                    success_text: Some("Unlikely. These mailboxes are sad. Lonely. Touch-starved.".to_string()),
                    failure_text: None,
                    speaker: Some("Delusion".to_string())
                },
            ],
            ..Default::default()
        }
    );

    vestibule_dialogues.insert(
        "Mailbox106".to_string(),
        Dialogue{
            speaker: "Mailboxes".to_string(),
            intro: "This mailbox's cover is hanging off its hinge. The nameplate, Kutuzov, is slightly bent.".to_string(),
            options: vec![
                DialogueOption {
                    description: "".to_string(),
                    success_dialogue: Some("Mailbox106Part2".to_string()),
                    ..Default::default()
                },
            ],
            passive_check: vec![
                PassiveCheck {
                    skill: "gunsmoke".to_string(),
                    target: 10,
                    success_text: Some("Psh, what a pussy. A real man could knock the cover clean off. Establish dominance. Punch his mailbox *harder*.".to_string()),
                    failure_text: Some("Oh yeah, this Kutuzov fellow is a four-star badass. It takes a real man to punch his own mailbox.".to_string()),
                    speaker: Some("Gunsmoke".to_string()),
                },
                PassiveCheck {
                    skill: "pathology".to_string(),
                    target: 10,
                    success_text: Some("This 'Kutuzov' suffers from a condition in which he gives human qualities to inanimate objects. I'm sure you can't relate".to_string()),
                    failure_text: Some("It takes a real man to punch *someone else's* mailbox. Punching your own is just pathetic.".to_string()),
                    speaker: Some("Pathology".to_string()),
                },
                PassiveCheck {
                    skill: "transcendence".to_string(),
                    target: 10,
                    success_text: Some("This is *not* the New Soviet Man. Or Woman.".to_string()),
                    failure_text: Some("Why are we assuming Kutuzov is a man? Women can annihilate state property too.".to_string()),
                    speaker: Some("Transcendence".to_string()),
                },
            ],
            ..Default::default()
        }
    );

    vestibule_dialogues.insert(
        "Mailbox106Part2".to_string(),
        Dialogue{
            speaker: "Mailboxes".to_string(),
            intro: "Ahem. As I was saying, the box is stuffed with official notices and bills. One of them sticks out: an empty, ripped-open court notice.".to_string(),
            options: vec![
                DialogueOption {
                    description: "Punch it *harder*.".to_string(),
                    //gunsmoke check, failure you lose health, break your wrist
                    success_dialogue: Some("".to_string()),
                    ..Default::default()
                }, // this option should become available if the gunsmoke 
                // or should it?
                DialogueOption {
                    description: "I'm not doing that.".to_string(),
                    success_dialogue: Some("MailboxFirstRow".to_string()),
                    ..Default::default()
                },
            ],
            passive_check: vec![
                PassiveCheck {
                    skill: "dossier".to_string(),
                    target: 10,
                    success_text: Some("These envelopes carry three kinds of notices: Fines, Summons, and Divorces. Take your bets.".to_string()),
                    failure_text: None,
                    speaker: Some("Dossier".to_string()),
                },   
            ],
            ..Default::default()
        }
    );

    //kutuzov is loud and violent, nobody likes him, 108 remains only because they can't leave or somehow don't midn
    //106 is beaten up, slammed, punched

    vestibule_dialogues.insert(
        "Mailbox400".to_string(),
        Dialogue{
            speaker: "Mailboxes".to_string(),
            intro: "There's just one envelope, crisp and clean, lying on the bottom. The writing on the front reads 'Apologies'.".to_string(),
            options: vec![
                DialogueOption {
                    description: "Whose writing?. (Dossier 10)".to_string(),
                    challenge_attribute: Some("dossier".to_string()),
                    challenge_number: Some(10),
                    success_dialogue: Some("Mailbox400LetterSuccess".to_string()),
                    failure_dialogue: Some("Mailbox400LetterFailure".to_string()),
                    ..Default::default()
                },
                DialogueOption {
                    description: "Take the letter".to_string(),
                    success_dialogue: Some("MailboxFourthRow".to_string()),
                    item_to_pickup: Some("Administrator's letter".to_string()),
                    ..Default::default()
                },
            ],
            ..Default::default()
        }
    );

    vestibule_dialogues.insert(
        "Mailbox400LetterSuccess".to_string(),
        Dialogue{
            speaker: "Crisp Letter".to_string(),
            intro: "Neat, disciplined, orderly handwriting. Professional. From a hand that kills.".to_string(),
            options: vec![
                DialogueOption {
                    description: "The plot thickens!".to_string(),
                    success_dialogue: Some("Mailbox400LetterPen".to_string()),
                    ..Default::default()
                },
                DialogueOption {
                    description: "Cut the drama. What else can I tell?".to_string(),
                    success_dialogue: Some("Mailbox400LetterPen".to_string()),
                    ..Default::default()
                },
            ],
            ..Default::default()
        }
    );

    vestibule_dialogues.insert(
        "Mailbox400LetterFailure".to_string(),
        Dialogue{
            speaker: "Crisp Letter".to_string(),
            intro: "This is really nice handwriting. Precise. It just screams 'competence'. Whoever wrote this was clearly...".to_string(),
            options: vec![
                DialogueOption {
                    description: "A politician.".to_string(),
                    success_dialogue: Some("Mailbox400LetterPen".to_string()),
                    ..Default::default()
                },
                DialogueOption {
                    description: "A scientist.".to_string(),
                    success_dialogue: Some("Mailbox400LetterPen".to_string()),
                    ..Default::default()
                },
                DialogueOption {
                    description: "A bureaucrat.".to_string(),
                    success_dialogue: Some("Mailbox400LetterPen".to_string()),
                    ..Default::default()
                },
                DialogueOption {
                    description: "An artist.".to_string(),
                    success_dialogue: Some("Mailbox400LetterPen".to_string()),
                    ..Default::default()
                },
                DialogueOption {
                    description: "How should I know?".to_string(),
                    success_dialogue: Some("Mailbox400LetterPen".to_string()),
                    ..Default::default()
                },
            ],
            ..Default::default()
        }
    );

    vestibule_dialogues.insert(
        "Mailbox400LetterPen".to_string(),
        Dialogue{
            speaker: "Crisp Letter".to_string(),
            intro: "The pen almost runs out of ink just as he finishes: the dark, confident line sputters out. It left a curling imprint on the paper, a ghostly line where no ink filled in.".to_string(),
            options: vec![
                DialogueOption {
                    description: "Continue".to_string(),
                    success_dialogue: Some("Mailbox400LetterPenQuestions".to_string()),
                    ..Default::default()
                },
            ],
            passive_check: vec![
                PassiveCheck {
                    skill: "checkmate".to_string(),
                    target: 10,
                    success_text: Some("He was in a rush, but wanted to hide it. Someone with a hand this meticulous would have switched to a new cartridge otherwise.".to_string()),
                    failure_text: Some("It means he was using his pen a lot. Makes sense for someone with really nice handwriting to use a pen a lot.".to_string()),
                    speaker: Some("Checkmate".to_string())
                },
            ] ,
            ..Default::default()
        }
    );

    vestibule_dialogues.insert(
        "Mailbox400LetterPenQuestions".to_string(),
        Dialogue{
            speaker: "Crisp Letter".to_string(),
            intro: "The letter waits in the mailbox.".to_string(),
            options: vec![
                DialogueOption {
                    description: "Anything else? At all? (Oldtime Religion 8)".to_string(),
                    challenge_attribute: Some("oldtime_religion".to_string()),
                    challenge_number: Some(8),
                    success_dialogue: Some("Mailbox400LetterAnythingSuccess".to_string()),
                    failure_dialogue: Some("Mailbox400LetterAnythingFailure".to_string()),
                    ..Default::default()
                },
                DialogueOption {
                    description: "Shake the envelope.".to_string(),
                    success_dialogue: Some("Mailbox400LetterListen".to_string()),
                    ..Default::default()
                },
                DialogueOption {
                    description: "Sniff the envelope.".to_string(),
                    success_dialogue: Some("Mailbox400LetterSmell".to_string()),
                    ..Default::default()
                },
                DialogueOption {
                    description: "Taste the envelope.".to_string(),
                    success_dialogue: Some("Mailbox400LetterTaste".to_string()),
                    ..Default::default()
                },
                DialogueOption {
                    description: "Enough of this.".to_string(),
                    success_dialogue: Some("Mailbox400".to_string()),
                    ..Default::default()
                },
            ],
            ..Default::default()
        }
    );

    vestibule_dialogues.insert(
        "Mailbox400LetterAnythingSuccess".to_string(),
        Dialogue{
            speaker: "Oldtime Religion".to_string(),
            intro: "He was religious.".to_string(),
            options: vec![
                DialogueOption {
                    description: "Egads! The opiate of the masses? In *my* socialist republic?".to_string(),
                    success_dialogue: Some("LetterReligious".to_string()),
                    ..Default::default()
                },
                DialogueOption {
                    description: "Seriously? How could I possibly know that?".to_string(),
                    success_dialogue: Some("LetterReligious".to_string()),
                    ..Default::default()
                },
            ],
            ..Default::default()
        }
    );

    vestibule_dialogues.insert(
        "Mailbox400LetterListen".to_string(),
        Dialogue{
            speaker: "Crisp Letter".to_string(),
            intro: "The envelope is thin, without overt bulging. But the dull jangle of metal sounds inside. A key, probably.".to_string(),
            options: vec![
                DialogueOption {
                    description: "But I already have a key.".to_string(),
                    success_dialogue: Some("LetterKeyAlready".to_string()),
                    ..Default::default()
                },
            ],
            ..Default::default()
        }
    );

    vestibule_dialogues.insert(
        "Mailbox400LetterSmell".to_string(),
        Dialogue{
            speaker: "Crisp Letter".to_string(),
            intro: "It smells like stale sweat and candle smoke. It probably sat on someone's desk for a long time. He might have been saving it.".to_string(),
            options: vec![
                DialogueOption {
                    description: "Return".to_string(),
                    success_dialogue: Some("Mailbox400LetterPenQuestions".to_string()),
                    ..Default::default()
                },
            ],
            ..Default::default()
        }
    );

    vestibule_dialogues.insert(
        "Mailbox400LetterTaste".to_string(),
        Dialogue{
            speaker: "Crisp Letter".to_string(),
            intro: "It tastes like paper. I don't know what you expected.".to_string(),
            options: vec![
                DialogueOption {
                    description: "Return".to_string(),
                    success_dialogue: Some("Mailbox400LetterPenQuestions".to_string()),
                    ..Default::default()
                },
            ],
            ..Default::default()
        }
    );

    vestibule_dialogues.insert(
        "LetterReligious".to_string(),
        Dialogue{
            speaker: "Oldtime Religion".to_string(),
            intro: "It's more likely than you think. You can see it in how the 'g' curls: a decadent, calligraphic flourish that comes out of the handwriting methods taught before the Revolution. Some reactionary groups, especially those aligned with the Orthodox church, pick it up from copying church documents.".to_string(),
            options: vec![
                DialogueOption {
                    description: "Case closed!".to_string(),
                    success_dialogue: Some("".to_string()),
                    ..Default::default()
                }, // what case, you're not a detective
                DialogueOption {
                    description: "But, to be clear, calligraphy doesn't actually imply anything about the writer's beliefs regarding God, the church, or anything else, right?".to_string(),
                    success_dialogue: Some("".to_string()),
                    ..Default::default()
                },
            ],
            ..Default::default()
        }
    );

    vestibule_dialogues.insert(
        "".to_string(),
        Dialogue{
            speaker: "".to_string(),
            intro: "".to_string(),
            options: vec![
                DialogueOption {
                    description: "".to_string(),
                    success_dialogue: Some("".to_string()),
                    ..Default::default()
                },
            ],
            ..Default::default()
        }
    );



    //mess with people's mail, most empty
    // box numbers
    // your own, a letter from the last guy, a key too??? but not the key to the front door



    //in the admin room, a calendar? learn about time, historical events, and the last guy's personality


    // default-enabled version, nice and compact
    // remember to make is_hidden false if this is an entry point!!
    vestibule_dialogues.insert(
        "".to_string(),
        Dialogue{
            speaker: "".to_string(),
            intro: "".to_string(),
            options: vec![
                DialogueOption {
                    description: "".to_string(),
                    success_dialogue: Some("".to_string()),
                    ..Default::default()
                },
            ],
            ..Default::default()
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
                    visible_when: None,
                    flags: None,
                },
            ],
            passive_check: vec![],
            xp_reward: None,
            is_hidden: true,
            time: Some(1),
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
                    visible_when: None,
                    flags: None,
                },
                DialogueOption {
                    description: "Try to open the stuck door (Dossier challenge).".to_string(),
                    challenge_attribute: Some("dossier".to_string()),
                    challenge_number: Some(12),
                    success_dialogue: Some("Garden".to_string()),
                    failure_dialogue: Some("FailedDoor".to_string()),
                    item_to_pickup: None,
                    visible_when: None,
                    flags: None,
                },
                DialogueOption {
                    description: "Go back to the vestibule.".to_string(),
                    challenge_attribute: None,
                    challenge_number: None,
                    success_dialogue: Some("Vestibule".to_string()),
                    failure_dialogue: None,
                    item_to_pickup: None,
                    visible_when: None,
                    flags: None,
                },
            ],
            passive_check: vec![],
            xp_reward: None,
            is_hidden: false,
            time: Some(1),
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
                    visible_when: None,
                    flags: None,
                },
            ],
            passive_check: vec![],
            xp_reward: None,
            is_hidden: false,
            time: Some(1),
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
                    visible_when: None,
                    flags: None,
                },
            ],
            passive_check: vec![],
            xp_reward: None,
            is_hidden: false,
            time: Some(1),
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
                    visible_when: None,
                    flags: None,
                },
                DialogueOption {
                    description: "Go back to the first floor.".to_string(),
                    challenge_attribute: None,
                    challenge_number: None,
                    success_dialogue: Some("FirstFloor".to_string()),
                    failure_dialogue: None,
                    item_to_pickup: None,
                    visible_when: None,
                    flags: None,
                },
            ],
            passive_check: vec![],
            xp_reward: None,
            is_hidden: false,
            time: Some(1)
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
                    visible_when: None,
                    flags: None,
                },
            ],
            passive_check: vec![],
            xp_reward: None,
            is_hidden: false,
            time: Some(1),
        },
    );

    let outdoors_dialogues = HashMap::new();

    let second_floor_dialogues = HashMap::new();

    let third_floor_dialogues = HashMap::new();

    let fourth_floor_dialogues = HashMap::new();

    let rooftop_garden_dialogues = HashMap::new();

    let admin_room_dialogues = HashMap::new();

    // Define Locations

    locations.insert(
        "Outdoors".to_string(),
        Location {
            name: "Outdoors".to_string(),
            dialogues: outdoors_dialogues,
            exits: vec!["Vestibule".to_string(),]
        }
    );

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
            exits: vec!["Vestibule".to_string(), "SecondFloor".to_string(), "ThirdFloor".to_string(), "FourthFloor".to_string(),],
        },
    );

    locations.insert(
        "SecondFloor".to_string(),
        Location {
            name: "The Second Floor".to_string(),
            dialogues: second_floor_dialogues,
            exits: vec!["FirstFloor".to_string(), "ThirdFloor".to_string(), "FourthFloor".to_string(),],
        },
    );

    locations.insert(
        "ThirdFloor".to_string(),
        Location {
            name: "The Third Floor".to_string(),
            dialogues: third_floor_dialogues,
            exits: vec!["FirstFloor".to_string(), "SecondFloor".to_string(), "FourthFloor".to_string(),],
        },
    );

    locations.insert(
        "FourthFloor".to_string(),
        Location {
            name: "The Fourth Floor".to_string(),
            dialogues: fourth_floor_dialogues,
            exits: vec!["FirstFloor".to_string(), "SecondFloor".to_string(), "ThirdFloor".to_string(), "RooftopGarden".to_string(), "AdministratorRoom".to_string(),],
        },
    );

    locations.insert(
        "AdministratorRoom".to_string(),
        Location {
            name: "Administrator's Room".to_string(),
            dialogues: admin_room_dialogues,
            exits: vec!["FourthFloor".to_string()],
        },
    );

    locations.insert(
        "RooftopGarden".to_string(),
        Location {
            name: "Rooftop Garden".to_string(),
            dialogues: rooftop_garden_dialogues,
            exits: vec!["FourthFloor".to_string()],
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

    locations
}


