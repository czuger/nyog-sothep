//! The game page: one screen per prompt, and never a secret of the professor in the HTML.

use askama::Template;
use sqlx::SqlitePool;

use nyog_sothep::db::{self, EventRecord, GameRecord, save_state};
use nyog_sothep::domain::investigator_ai::InvestigatorsAi;
use nyog_sothep::domain::professor_ai::PetersenAi;
use nyog_sothep::domain::turn::{
    Engine, GameState, Outcome, Phase, ProfessorMoveOptions, ProfessorPrompt, Prompt, Side,
};
use nyog_sothep::domain::{CounterKind, FightChoice, InvestigatorId, LocationId, RandomDice};
use nyog_sothep::service::GameView;
use nyog_sothep::web::{GamePage, Screen};

const IDS: [InvestigatorId; 4] = [
    InvestigatorId::Poirot,
    InvestigatorId::Hercule,
    InvestigatorId::Sandy,
    InvestigatorId::Lemon,
];

/// A fresh game whose professor holds the Flying Horror and has hidden Ghouls in Lynn.
fn secret_state() -> GameState {
    let (mut state, _) = Engine::new_game(
        &IDS,
        Side::Investigators,
        &mut RandomDice::seeded(5),
        &mut PetersenAi::seeded(5),
        &mut InvestigatorsAi::seeded(5),
    );
    for kind in state.counters.hand() {
        state.counters.discard(kind);
    }
    assert!(state.counters.draw(CounterKind::FlyingHorror));
    assert!(state.counters.draw(CounterKind::Ghouls));
    assert!(state.counters.place(CounterKind::Ghouls, LocationId::Lynn));
    state
}

async fn render(state: &GameState) -> (Screen, String) {
    let pool: SqlitePool = db::connect("sqlite::memory:").await.unwrap();
    let mut connection = pool.acquire().await.unwrap();
    let game_id = GameRecord::insert(&mut connection, "secret-token", state)
        .await
        .unwrap();
    let mut game = GameRecord::load(&mut connection, game_id, "secret-token")
        .await
        .unwrap()
        .unwrap();
    save_state(&mut connection, &mut game, state).await.unwrap();
    let log = EventRecord::newest_first(&mut connection, game_id)
        .await
        .unwrap();
    let view = GameView::of(&game, state, log);
    let page = GamePage::new(view, None);
    (page.screen, page.render().unwrap())
}

#[tokio::test]
async fn the_page_hides_the_professors_secrets() {
    let state = secret_state();
    let (screen, html) = render(&state).await;
    assert_eq!(screen, Screen::Moves);
    assert!(
        !html.contains("token-ghouls.png"),
        "a hidden counter leaked"
    );
    assert!(!html.contains("token-flying_horror.png") && !html.contains("Horreur volante"));
    assert!(!html.contains("Le professeur Petersen à") && !html.contains("last-seen"));
    assert!(!html.contains("devait apparaître") && !html.contains("Feuille de route"));
    assert!(!html.contains("token-nyog-sothep.png"));
    assert!(html.contains("position inconnue"));
}

#[tokio::test]
async fn moves_are_offered_on_the_map_and_in_the_panel() {
    let (screen, html) = render(&secret_state()).await;
    assert_eq!(screen, Screen::Moves);
    assert!(html.contains(r#"data-mover="move_0""#));
    assert!(html.contains(r#"data-targets="nantucket_sound""#));
    assert!(html.contains(r#"<select name="move_3" data-move-select>"#));
    assert!(html.contains(r#"value="nantucket_sound">Embarquer : Nantucket Sound"#));
    assert!(html.contains(r#"value="psychoanalyst""#));
    assert!(html.contains("/moves"));
}

#[tokio::test]
async fn discovered_counters_and_sightings_are_shown() {
    let mut state = secret_state();
    state.counters.set_place_at(
        LocationId::Lynn,
        nyog_sothep::domain::CounterPlace::Placed {
            city: LocationId::Lynn,
            discovered: true,
        },
    );
    state.clues.push(nyog_sothep::domain::turn::Clue {
        turn: 1,
        kind: nyog_sothep::domain::turn::ClueKind::Spotted {
            at: LocationId::Boston,
        },
    });
    let (_, html) = render(&state).await;
    assert!(html.contains("token-ghouls.png"));
    assert!(html.contains("a été vu à Boston au tour 1"));
    assert!(html.contains("Le professeur est repéré à Boston."));
}

#[tokio::test]
async fn each_prompt_has_its_screen() {
    let mut state = secret_state();
    state.prompt = Some(Prompt::Table { investigator: 1 });
    let (screen, html) = render(&state).await;
    assert_eq!(screen, Screen::Table);
    assert!(html.contains(r#"name="table" value="3""#) && html.contains("Hercule"));

    state.prompt = Some(Prompt::Fight {
        investigator: 2,
        options: vec![FightChoice::Weapon, FightChoice::Hide],
    });
    let (screen, html) = render(&state).await;
    assert_eq!(screen, Screen::Fight);
    assert!(html.contains(r#"name="choice" value="weapon""#) && html.contains(r#"value="hide""#));

    state.prompt = Some(Prompt::Cities {
        investigator: 0,
        question: nyog_sothep::domain::ProfessorQuestion::NyogSothepInTwoCities { optional: true },
    });
    let (screen, html) = render(&state).await;
    assert_eq!(screen, Screen::Cities);
    assert!(
        html.contains(r#"<input type="checkbox" name="city" value="boston" form="screen-form">"#)
    );
    assert!(html.contains(r#"name="skip""#));
}

#[tokio::test]
async fn the_end_reveals_everything() {
    let mut state = secret_state();
    state.outcome = Some(Outcome::ProfessorKilled);
    state.phase = Phase::Finished;
    state.prompt = None;
    let (screen, html) = render(&state).await;
    assert_eq!(screen, Screen::Finished);
    assert!(html.contains("Victoire"));
    assert!(html.contains("Feuille de route"));
    assert!(html.contains("devait apparaître"));
    assert!(
        html.contains("token-ghouls.png"),
        "hidden counters are shown at the end"
    );
    assert!(html.contains("Classement des investigateurs"));
}

#[tokio::test]
async fn the_professor_sees_his_secrets() {
    let mut state = secret_state();
    state.human = Side::Professor;
    let at = state.professor.location;
    state.prompt = Some(Prompt::Professor(ProfessorPrompt::Movement {
        options: ProfessorMoveOptions {
            steps: vec![at],
            flying: vec![LocationId::Boston],
        },
    }));
    let (screen, html) = render(&state).await;
    assert_eq!(screen, Screen::ProfessorMove);
    assert!(html.contains("token-ghouls.png"), "his hidden counter");
    assert!(html.contains("token hidden"));
    assert!(html.contains("Votre main") && html.contains("token-flying_horror.png"));
    assert!(html.contains(r#"data-mover="destination""#));
    assert!(html.contains(r#"data-targets="boston""#));
    assert!(html.contains("Horreur volante : Boston"));
    assert!(html.contains("Feuille de route") && html.contains("Carnet des investigateurs"));
    assert!(html.contains(&format!("vous êtes {}", at.at_phrase())));
    assert!(html.contains("/move"));
}

#[tokio::test]
async fn the_professors_prompts_have_their_screens() {
    let mut state = secret_state();
    state.human = Side::Professor;
    state.prompt = Some(Prompt::Professor(ProfessorPrompt::Relocate {
        options: vec![LocationId::Lynn, LocationId::MassachusettsBay],
        pushed_back: false,
    }));
    let (screen, html) = render(&state).await;
    assert_eq!(screen, Screen::Retreat);
    assert!(html.contains(r#"<input type="radio" name="city" value="massachusetts_bay""#));
    assert!(!html.contains(r#"value="boston" form="screen-form""#));
    assert!(html.contains(r#"<select name="city" data-pick-select>"#));
    assert!(html.contains(r#"<option value="massachusetts_bay">"#));

    state.prompt = Some(Prompt::Professor(ProfessorPrompt::NyogStep {
        options: vec![LocationId::Dedham],
    }));
    let (screen, html) = render(&state).await;
    assert_eq!(screen, Screen::NyogStep);
    assert!(html.contains(r#"<option value="dedham">Dedham</option>"#));
    assert!(html.contains("/location"));

    state.prompt = Some(Prompt::Professor(ProfessorPrompt::Table1 {
        options: vec![CounterKind::Storm, CounterKind::Fanatics],
    }));
    let (screen, html) = render(&state).await;
    assert_eq!(screen, Screen::Table1);
    assert!(html.contains(r#"name="counter" value="storm""#) && html.contains("/counter"));

    state.prompt = Some(Prompt::Professor(ProfessorPrompt::Setup));
    let (screen, html) = render(&state).await;
    assert_eq!(screen, Screen::Setup);
    assert!(html.contains(r#"<select name="nyog_city">"#));
    assert!(html.contains(r#"<input type="radio" name="city" value="boston" form="screen-form">"#));
}

#[tokio::test]
async fn the_professors_own_city_is_named_for_him() {
    let mut state = secret_state();
    state.human = Side::Professor;
    state.professor.location = LocationId::Pascoag;
    state.prompt = Some(Prompt::Professor(ProfessorPrompt::NameCities { count: 3 }));
    let (screen, html) = render(&state).await;
    assert_eq!(screen, Screen::Names);
    assert!(html.contains(r#"<input type="hidden" name="city" value="pascoag">"#));
    assert!(!html.contains(r#"value="pascoag" form="screen-form""#));
    assert!(
        html.contains(r#"<input type="checkbox" name="city" value="boston" form="screen-form">"#)
    );
    assert!(html.contains(r#"data-pick-limit="2""#));
    assert!(html.contains("Cochez les 2 autres villes"));
}
