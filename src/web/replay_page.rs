use std::collections::BTreeMap;

use askama::Template;
use serde::Serialize;

use crate::domain::map::location;
use crate::domain::{CounterKind, InvestigatorId, LocationId};
use crate::error::AppError;
use crate::service::ReplayView;
use crate::web::board_view::BoardView;

/// The replay page: the map, and the recorded events that `replay.js` plays on it.
#[derive(Template)]
#[template(path = "replay.html")]
pub struct ReplayPage {
    /// The replayed game.
    pub view: ReplayView,
    /// The map, without tokens: the script draws them.
    pub board: BoardView,
    /// The data the script reads, as JSON safe to embed in a `<script>` element.
    pub data: String,
}

/// What `replay.js` reads: the French names it writes and the steps it plays.
#[derive(Serialize)]
struct ReplayData<'a> {
    places: BTreeMap<&'static str, PlaceData>,
    investigators: BTreeMap<&'static str, InvestigatorData>,
    counters: BTreeMap<&'static str, &'static str>,
    steps: &'a [crate::domain::turn::ReplayStep],
}

#[derive(Serialize)]
struct PlaceData {
    name: &'static str,
    at: String,
    x: u32,
    y: u32,
    sea: bool,
}

#[derive(Serialize)]
struct InvestigatorData {
    name: &'static str,
    agreement: &'static str,
}

impl ReplayPage {
    /// Builds the page of a replay.
    ///
    /// # Errors
    /// `Template` if the steps cannot be written as JSON.
    pub fn new(view: ReplayView) -> Result<ReplayPage, AppError> {
        let data = ReplayData {
            places: LocationId::ALL
                .into_iter()
                .map(|id| {
                    let info = location(id);
                    let place = PlaceData {
                        name: info.name,
                        at: id.at_phrase(),
                        x: info.x,
                        y: info.y,
                        sea: id.is_sea(),
                    };
                    (id.code(), place)
                })
                .collect(),
            investigators: InvestigatorId::ALL
                .into_iter()
                .map(|id| {
                    let names = InvestigatorData {
                        name: id.name(),
                        agreement: id.agreement(),
                    };
                    (id.code(), names)
                })
                .collect(),
            counters: CounterKind::ALL
                .into_iter()
                .map(|kind| (kind.code(), kind.name()))
                .collect(),
            steps: &view.steps,
        };
        let json = serde_json::to_string(&data)
            .map_err(|error| AppError::Template(askama::Error::custom(error)))?;
        Ok(ReplayPage {
            // `<` only occurs inside JSON strings, where its escape keeps `</script>` out.
            data: json.replace('<', "\\u003c"),
            view,
            board: BoardView::bare(),
        })
    }

    /// The game's private URL.
    pub fn game_path(&self) -> String {
        format!("/games/{}/{}", self.view.game_id, self.view.token)
    }

    /// The outcome's French text, as the human read it.
    pub fn outcome_text(&self) -> &'static str {
        self.view.outcome.text(self.view.human)
    }

    /// Whether the human won.
    pub fn human_won(&self) -> bool {
        self.view.outcome.won_by(self.view.human)
    }
}
