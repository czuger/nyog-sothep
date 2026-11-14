use crate::db::{GameDocument, LogBatch, LogEntry, Store};
use crate::domain::turn::{
    ClueKind, GameState, MoveOptions, Outcome, ProfessorPrompt, Prompt, RouteStage, RuleSet, Side,
};
use crate::domain::{
    CounterKind, CounterPlace, InvestigatorId, InvestigatorStatus, Item, LocationId, Score,
};
use crate::error::AppError;
use crate::service::token_tilts::{TiltedToken, TokenTilts};

/// What the investigators know of the professor's position.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProfessorSighting {
    /// Never seen.
    Unknown,
    /// Last seen there, on that turn.
    LastSeen {
        /// Where.
        at: LocationId,
        /// When.
        turn: u32,
    },
    /// In plain sight (after the summoning, or at the end of the game).
    Visible {
        /// Where he stands.
        at: LocationId,
    },
}

/// An investigator as the page shows them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvestigatorView {
    /// Position in playing order.
    pub index: usize,
    /// Which counter.
    pub id: InvestigatorId,
    /// Sanity points.
    pub san: i32,
    /// The objects carried.
    pub items: Vec<Item>,
    /// Dice rolled on Table 2 or 3.
    pub event_dice: u8,
    /// Knows the banishment spell.
    pub spell: bool,
    /// Where they are.
    pub location: LocationId,
    /// In the game, mad or dead.
    pub status: InvestigatorStatus,
    /// Their moves, when the prompt asks for them.
    pub moves: Option<MoveOptions>,
    /// French notes on lasting effects (lost turns, broken car…).
    pub notes: Vec<String>,
}

/// One line of the notebook.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotebookLine {
    /// The turn the clue was learned.
    pub turn: u32,
    /// The clue, in French.
    pub text: String,
}

/// A counter placed on the board.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CounterView {
    /// Its city.
    pub city: LocationId,
    /// Its kind.
    pub kind: CounterKind,
    /// Still hidden from the investigators (only shown to the professor, or at the end).
    pub hidden: bool,
}

/// One investigator's final score.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScoreLine {
    /// The investigator.
    pub id: InvestigatorId,
    /// Their score, line by line.
    pub score: Score,
}

/// The game as the human may see it. Playing the investigators, never the professor's position,
/// hand, hidden counters, route sheet or Nyog'Sothep's city while they are secret; playing the
/// professor, everything.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameView {
    /// The side the human plays.
    pub human: Side,
    /// The rules the game is played with.
    pub rules: RuleSet,
    /// Game id.
    pub game_id: i64,
    /// Secret part of the game's private URL.
    pub token: String,
    /// Bumped on every change.
    pub version: i64,
    /// The round.
    pub turn: u32,
    /// The decision the human must take.
    pub prompt: Option<Prompt>,
    /// The result, once over.
    pub outcome: Option<Outcome>,
    /// The professor's hit points (the investigators dealt the damage).
    pub professor_hp: i32,
    /// What is known of his position.
    pub professor: ProfessorSighting,
    /// The four investigators.
    pub investigators: Vec<InvestigatorView>,
    /// Counters on the board that the human may see.
    pub counters: Vec<CounterView>,
    /// The professor's hand, when the human may see it.
    pub hand: Vec<CounterKind>,
    /// The professor may summon Nyog'Sothep now (shown to the professor only).
    pub can_summon: bool,
    /// Cities destroyed by Nyog'Sothep.
    pub destroyed: Vec<LocationId>,
    /// Where Nyog'Sothep stands, once summoned.
    pub nyog_location: Option<LocationId>,
    /// Nyog'Sothep's appearance city, once revealed.
    pub nyog_city: Option<LocationId>,
    /// The professor must announce his movements at sea.
    pub sea_announcements: bool,
    /// The investigators' notebook, oldest first.
    pub notebook: Vec<NotebookLine>,
    /// The game log, newest first.
    pub log: Vec<LogEntry>,
    /// The professor's route sheet, shown to the professor, or at the end.
    pub route_sheet: Vec<RouteStage>,
    /// The investigators' scores, at the end only.
    pub scores: Vec<ScoreLine>,
    /// The rotations the tokens are drawn with.
    pub tilts: TokenTilts,
}

impl GameView {
    /// Loads the view of a game.
    ///
    /// # Errors
    /// `GameNotFound` for a wrong id or token, or a database error.
    pub async fn load(store: &Store, game_id: i64, token: &str) -> Result<GameView, AppError> {
        let token = token.to_string();
        store
            .run(move |store| {
                let database = store.database();
                let game =
                    GameDocument::find(database, game_id, &token)?.ok_or(AppError::GameNotFound)?;
                let log = LogBatch::newest_first(database, game_id)?;
                Ok(GameView::of(&game, log))
            })
            .await
    }

    /// Builds the view from the stored game, keeping only what the human may see.
    pub fn of(game: &GameDocument, log: Vec<LogEntry>) -> GameView {
        let state = &game.state;
        let mut tilts = TokenTilts::of_documents(&game.tilts);
        let finished = state.is_finished();
        let secrets = finished || state.human == Side::Professor;
        let revealed = secrets || state.nyog.is_summoned();
        if !revealed {
            tilts.forget(TiltedToken::Professor);
        }
        GameView {
            human: state.human,
            rules: state.rules,
            game_id: game.id,
            token: game.token.clone(),
            version: game.version,
            turn: state.turn,
            prompt: state.prompt.clone(),
            outcome: state.outcome,
            professor_hp: state.professor.hp,
            professor: sighting(state, revealed),
            investigators: investigators(state),
            counters: counters(state, secrets),
            hand: match secrets {
                true => state.counters.hand(),
                false => Vec::new(),
            },
            can_summon: state.human == Side::Professor && state.can_summon(),
            destroyed: state.destroyed.clone(),
            nyog_location: state.nyog.location,
            nyog_city: revealed.then_some(state.nyog.city),
            sea_announcements: state.sea_announcements,
            notebook: state
                .clues
                .iter()
                .map(|clue| NotebookLine {
                    turn: clue.turn,
                    text: clue.kind.describe(),
                })
                .collect(),
            log,
            route_sheet: if secrets {
                state.route_sheet.clone()
            } else {
                Vec::new()
            },
            scores: if finished { scores(state) } else { Vec::new() },
            tilts,
        }
    }

    /// Whether the game is over.
    pub fn is_finished(&self) -> bool {
        self.outcome.is_some()
    }

    /// The professor's position when he must name it among other cities: the page names it for him.
    pub fn own_city_to_name(&self) -> Option<LocationId> {
        match (&self.prompt, &self.professor) {
            (
                Some(Prompt::Professor(ProfessorPrompt::NameCities { .. })),
                ProfessorSighting::Visible { at },
            ) => Some(*at),
            _ => None,
        }
    }
}

fn counters(state: &GameState, secrets: bool) -> Vec<CounterView> {
    state
        .counters
        .placed()
        .map(|(city, counter)| CounterView {
            city,
            kind: counter.kind,
            hidden: !matches!(
                counter.place,
                CounterPlace::Placed {
                    discovered: true,
                    ..
                }
            ),
        })
        .filter(|counter| secrets || !counter.hidden)
        .collect()
}

fn sighting(state: &GameState, revealed: bool) -> ProfessorSighting {
    if revealed {
        return ProfessorSighting::Visible {
            at: state.professor.location,
        };
    }
    last_sighting(state)
        .map(|(at, turn)| ProfessorSighting::LastSeen { at, turn })
        .unwrap_or(ProfessorSighting::Unknown)
}

/// Where and on which turn the investigators last spotted the professor.
pub(crate) fn last_sighting(state: &GameState) -> Option<(LocationId, u32)> {
    state.clues.iter().rev().find_map(|clue| match clue.kind {
        ClueKind::Spotted { at } => Some((at, clue.turn)),
        _ => None,
    })
}

fn investigators(state: &GameState) -> Vec<InvestigatorView> {
    let replaying = match state.prompt {
        Some(Prompt::PlayAgain { investigator }) => Some(investigator),
        _ => None,
    };
    state
        .investigators
        .iter()
        .enumerate()
        .map(|(index, investigator)| {
            let moves = match (&state.prompt, replaying) {
                (Some(Prompt::Moves), _) => MoveOptions::of(state, index, false),
                (_, Some(replayer)) if replayer == index => MoveOptions::of(state, index, true),
                _ => None,
            };
            InvestigatorView {
                index,
                id: investigator.id,
                san: investigator.san,
                items: investigator.items(),
                event_dice: investigator.event_dice(),
                spell: investigator.spell,
                location: investigator.location,
                status: investigator.status,
                moves,
                notes: notes(investigator),
            }
        })
        .collect()
}

fn notes(investigator: &crate::domain::Investigator) -> Vec<String> {
    let mut notes = Vec::new();
    if !investigator.is_active() {
        return notes;
    }
    match investigator.skip_turns {
        0 => {}
        1 => notes.push("perd son prochain tour".to_string()),
        turns => notes.push(format!("perd ses {turns} prochains tours")),
    }
    if investigator.car_broken {
        notes.push("voiture en panne : ne peut que rester ou prendre la mer".to_string());
    }
    if let Some(city) = investigator.forbidden_city {
        notes.push(format!(
            "interdit{} de séjour à {city}",
            investigator.id.agreement()
        ));
    }
    if investigator.free_border {
        notes.push("passera la frontière sans contrôle".to_string());
    }
    if let Some(city) = investigator.blocked_crossing {
        notes.push(format!("ne peut pas franchir la frontière vers {city}"));
    }
    notes
}

fn scores(state: &GameState) -> Vec<ScoreLine> {
    state
        .investigators
        .iter()
        .map(|investigator| ScoreLine {
            id: investigator.id,
            score: Score::of(investigator, state.destroyed.len()),
        })
        .collect()
}
