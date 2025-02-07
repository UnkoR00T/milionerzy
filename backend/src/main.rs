use tower_http::cors::{CorsLayer, Any};
use axum::{routing::get, Router};
use axum::http::Method;
use socketioxide::{SocketIo};
use tokio_postgres::{NoTls, Client};
use std::{sync::Arc};
use tokio::sync::Mutex;
use socketioxide::extract::{AckSender, Data, SocketRef};
use tokio::net::TcpListener;
use tracing::{error, info, span};
use tracing_subscriber::FmtSubscriber;
use serde::{Deserialize, Serialize};

// Define application state

#[derive(Serialize, Deserialize)]
struct ReturnStartGame {
    questionId: i32,
    question: Question,
}

#[derive(Serialize, Deserialize)]
struct AnswerQuestionBody {
    gameid: String,
    selected: i32
}

#[derive(Serialize, Deserialize)]
struct AddQuestion {
    gameId: String,
    question: String,
    answerA: String,
    answerB: String,
    answerC: String,
    answerD: String,
    correctAnswer: i32
}

#[derive(Serialize, Deserialize)]
struct UseResque { gameId: String, index: i32 }

#[derive(Serialize, Deserialize)]
struct RemoveQuestion { gameId: String, id: i32 }

#[derive(Serialize, Deserialize)]
struct AnswerReturn { selected: i32, correct: i32 }
#[derive(Serialize, Deserialize)]
#[derive(Clone)]
struct Question {
    id: i32,
    gameId: String,
    question: String,
    answerA: String,
    answerB: String,
    answerC: String,
    answerD: String,
    correctAnswer: i32,
}
struct AppState {
    db:Client,
    questions: Mutex<Vec<Question>>, // Stores questions
    selected: Mutex<i32>,         // Stores selected int
    cur_question: Mutex<i32>, // Stores current question
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;
    // Set up PostgreSQL connection
    info!("Connecting to postgres server");
    let (client, connection) = tokio_postgres::connect(
        "host=postgres user=admin password=abc123 dbname=milionerzy",
        NoTls,
    )
        .await
        .expect("Failed to connect to database");

    // Spawn the connection in a separate task
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Database connection error: {}", e);
        }
    });
    info!("Connacted to postgres server");

    info!("Initializing application state");
    let state = Arc::new(AppState {
        db: client,
        questions: Mutex::new(vec![]),
        selected: Mutex::new(0),
        cur_question: Mutex::new(0),
    });

    let state_clone = state.clone();
    // Set up Socket.io
    info!("Setting up socket");
    let (layer, io) = SocketIo::new_layer();
    io.ns("/", move |s: SocketRef| {
        info!("User connacted to socket");
        let state = Arc::clone(&state_clone);
        s.on("connectConsumer", |s: SocketRef, Data(gameid): Data<String>, ack: AckSender| {
           if gameid.starts_with("PRZEGIBEK2025") {
               info!("User registered");
               s.join(gameid);
               ack.send(&true).ok();
           } else{
               ack.send(&false).ok();
           }
        });

        s.on("testConnection", move |s: SocketRef, Data(gameid): Data<String>| async move {
            info!("Testing connection");
           if gameid.starts_with("PRZEGIBEK2025") {
               s.within(gameid).emit("testConnection", &()).await.unwrap();
           }
        });
        let state_for_startgame = Arc::clone(&state);
        s.on("startGame", move |s: SocketRef, Data(gameid): Data<String>, ack: AckSender| async move {
            info!("Starting game");
            {
                let mut selected = state_for_startgame.selected.lock().await;
                *selected = 0;
                let mut curquestion = state_for_startgame.cur_question.lock().await;
                *curquestion = 0;
            }
            let client = &state_for_startgame.db;
            let rows = client.query(
                "SELECT id, gameid, question, answera, answerb, answerc, answerd, correctanswer FROM questions WHERE gameid LIKE $1::TEXT",
                &[&gameid]
            ).await;

            if let Ok(rows) = rows {
                let mut questions = state_for_startgame.questions.lock().await;
                *questions = rows.iter().map(|row| {
                    Question {
                        id: row.get(0),
                        gameId: row.get(1),
                        question: row.get(2),
                        answerA: row.get(3),
                        answerB: row.get(4),
                        answerC: row.get(5),
                        answerD: row.get(6),
                        correctAnswer: row.get(7), // Placeholder for correct answer
                    }
                }).collect();
                let cur_question = state_for_startgame.cur_question.lock().await;
                ack.send(&*questions).ok();
                if let Some(question) = questions.get(*cur_question as usize) {
                    s.to(gameid).emit("startGame", &ReturnStartGame{questionId: *cur_question, question: question.clone() }).await.unwrap();
                }else{
                    panic!("Idk")
                }
            }
        });
        let state_for_answerQuestion = Arc::clone(&state);
        s.on("answerQuestion", move |s: SocketRef, Data(answer): Data<AnswerQuestionBody>, ack: AckSender| async move {
            info!("Answering question: {}", answer.selected);
            let mut selected = state_for_answerQuestion.selected.lock().await;
            let cur_question = state_for_answerQuestion.cur_question.lock().await;
            let questions = state_for_answerQuestion.questions.lock().await;
            if *selected != answer.selected {
                *selected = answer.selected;
                s.to(answer.gameid.clone()).emit("answerQuestionSelect", &answer.selected).await.unwrap();
            }else{
                s.to(answer.gameid.clone()).emit("answerQuestionFinal", &AnswerReturn{selected: *selected, correct: questions.get(*cur_question as usize).unwrap().correctAnswer }).await.unwrap();
                *selected = 0;
            }
            ack.send(&true).ok();
        });
        let state_for_changeScreen = Arc::clone(&state);
        s.on("changeScreen", move |s: SocketRef, Data(gameid): Data<String>, ack: AckSender | async move {
            let selected = state_for_changeScreen.selected.lock().await;
            if *selected == 0 {
                let questions = state_for_changeScreen.questions.lock().await;
                let cur_question = state_for_changeScreen.cur_question.lock().await;

                s.to(gameid).emit("changeScreen", &ReturnStartGame{
                    questionId: *cur_question,
                    question: questions.get(*cur_question as usize).unwrap().clone(),
                }).await.unwrap();
                ack.send(&true).ok();
            }
        });
        let state_for_newQuestion = Arc::clone(&state);
        s.on("nextQuestion", move |s: SocketRef, Data(gameid): Data<String>, ack: AckSender | async move{
            let mut cur_question = state_for_newQuestion.cur_question.lock().await;
            *cur_question += 1;
            let questions = state_for_newQuestion.questions.lock().await;
            if let Some(question) = questions.get(*cur_question as usize) {
                s.to(gameid).emit("changeScreen", &ReturnStartGame{questionId: *cur_question, question: question.clone() }).await.unwrap();
            }else{
                panic!("Idk")
            }
            ack.send(&true).ok();
        });
        //
        // Panel
        //
        let state_for_getquestions = Arc::clone(&state);
        s.on("getQuestions", move |_: SocketRef, Data(gameid): Data<String>, ack: AckSender| async move {
            {
                let mut selected = state_for_getquestions.selected.lock().await;
                *selected = 0;
                let mut curquestion = state_for_getquestions.cur_question.lock().await;
                *curquestion = 0;
            }
            let client = &state_for_getquestions.db;
            let rows = client.query(
                "SELECT id, gameid, question, answera, answerb, answerc, answerd, correctanswer FROM questions WHERE gameid LIKE $1::TEXT",
                &[&gameid]
            ).await;

            if let Ok(rows) = rows {
                let mut questions = state_for_getquestions.questions.lock().await;
                *questions = rows.iter().map(|row| {
                    Question {
                        id: row.get(0),
                        gameId: row.get(1),
                        question: row.get(2),
                        answerA: row.get(3),
                        answerB: row.get(4),
                        answerC: row.get(5),
                        answerD: row.get(6),
                        correctAnswer: row.get(7), // Placeholder for correct answer
                    }
                }).collect();
                ack.send(&*questions).ok();
            }
        });
        let state_for_addquestion = Arc::clone(&state);
        s.on("addQuestion", move |s: SocketRef, Data(question): Data<AddQuestion>, ack: AckSender| async move {
            let client = &state_for_addquestion.db;
            if let Err(error) = client.execute("INSERT INTO questions (gameid, question, answera, answerb, answerc, answerd, correctanswer) VALUES ($1::TEXT, $2::TEXT, $3::TEXT, $4::TEXT, $5::TEXT, $6::TEXT, $7::INTEGER)"
            , &[&question.gameId, &question.question, &question.answerA, &question.answerB, &question.answerC, &question.answerD, &question.correctAnswer]).await {
                error!("{}", error.to_string());
                ack.send(&false).ok();
            } else {
                info!("New question for {} was created", &question.gameId);
                ack.send(&true).ok();
            }
        });
        let state_for_deletequestion = Arc::clone(&state);
        s.on("deleteQuestion", move |s: SocketRef, Data(question): Data<RemoveQuestion>, ack: AckSender| async move {
            let client = &state_for_deletequestion.db;

            if let Err(error) = client.execute("DELETE FROM questions WHERE gameid = $1::TEXT AND id = $2::INTEGER", &[&question.gameId, &question.id]).await {
                error!("{}", error.to_string());
                ack.send(&false).ok();
            }else{
                info!("Question {}:{} was deleted", &question.gameId, &question.id);
                ack.send(&true).ok();
            }
        });
        s.on("useResque", move |s: SocketRef, Data(data): Data<UseResque>, ack: AckSender| async move {
            s.to(data.gameId).emit("useResque", &data.index).await.unwrap();
            ack.send(&true).ok();
        });
        // let state_for_getquestions = Arc::clone(&state);
        // s.on("get_questions", move |s: SocketRef, ack: AckSender| async {
        //     let questions = state_for_getquestions.questions.lock().await;
        //     ack.send(&*questions).ok();
        //     s.emit("questions", &*questions).ok();
        // });
    });

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers(Any);

    // Set up Axum router
    info!("Setting up Axum router");
    let app = Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .layer(cors)
        .layer(layer)
        .with_state(state);

    // Start the server
    info!("Starting server on 0.0.0.0:8080");
    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
