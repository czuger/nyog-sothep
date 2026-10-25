use sqlx::SqlitePool;

use crate::db::{EventRecord, GameRecord, load_state};
use crate::domain::turn::{ClueKind, GameState, MoveOptions, Outcome, Prompt, RouteStage, Side};
use crate::domain::{
    CounterKind, CounterPlace, InvestigatorId, InvestigatorStatus, LocationId, Score,
};
use crate::error::AppError;

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
    /// Holds a weapon.
    pub weapon: bool,
    /// Holds a protective sign.
    pub sign: bool,
    /// Holds a medallion.
    pub medallion: bool,
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
    pub log: Vec<EventRecord>,
    /// The professor's route sheet, shown to the professor, or at the end.
    pub route_sheet: Vec<RouteStage>,
    /// The investigators' scores, at the end only.
    pub scores: Vec<ScoreLine>,
}

impl GameView {
    /// Loads the view of a game.
    ///
    /// # Errors
    /// `GameNotFound` for a wrong id or token.
    pub async fn load(pool: &SqlitePool, game_id: i64, token: &str) -> Result<GameView, AppError> {
        let mut connection = pool.acquire().await?;
        let game = GameRecord::load(&mut connection, game_id, token)
            .await?
            .ok_or(AppError::GameNotFound)?;
        let state = load_state(&mut connection, &game).await?;
        let log = EventRecord::newest_first(&mut connection, game_id).await?;
        Ok(GameView::of(&game, &state, log))
    }

    /// Builds the view from the full state, keeping only what the human may see.
    pub fn of(game: &GameRecord, state: &GameState, log: Vec<EventRecord>) -> GameView {
        let finished = state.is_finished();
        let secrets = finished || state.human == Side::Professor;
        let revealed = secrets || state.nyog.is_summoned();
        GameView {
            human: state.human,
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
        }
    }

    /// Whether the game is over.
    pub fn is_finished(&self) -> bool {
        self.outcome.is_some()
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
    state
        .clues
        .iter()
        .rev()
        .find_map(|clue| match clue.kind {
            ClueKind::Spotted { at } => Some(ProfessorSighting::LastSeen {
                at,
                turn: clue.turn,
            }),
            _ => None,
        })
        .unwrap_or(ProfessorSighting::Unknown)
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
                weapon: investigator.weapon,
                sign: investigator.sign,
                medallion: investigator.medallion,
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
