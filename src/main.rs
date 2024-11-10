use rocket::{self, get, post, put, delete, routes, State, serde::{json::Json, Serialize, Deserialize}};

use std::sync::Mutex;
//Struct for workout
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Workout {
    id: String,
    name: String,
    sets: String,
    reps: String,
}

type WorkoutList = Mutex<Vec<Workout>>;

#[get("/workouts")]
async fn get_workouts(state: &State<WorkoutList>) -> Json<Vec<Workout>> {
    let workouts = state.lock().unwrap();
    Json(workouts.clone())
}

#[get("/workouts/<id>")]
async fn get_workout(id: String, state: &State<WorkoutList>) -> Option<Json<Workout>> {
    let workouts = state.lock().unwrap();
    workouts.iter().find(|&w| w.id == id).map(|w| Json(w.clone()))
}

#[post("/workouts", format = "json", data = "<workout>")]
async fn create_workout(workout: Json<Workout>, state: &State<WorkoutList>) -> String {
    let mut workouts = state.lock().unwrap();
    workouts.push(workout.into_inner());
    "Workout added".to_string()
}

#[put("/workouts/<id>", format = "json", data = "<workout>")]
async fn update_workout(id: String, workout: Json<Workout>, state: &State<WorkoutList>) -> Option<String> {
    let mut workouts = state.lock().unwrap();
    if let Some(existing_workout) = workouts.iter_mut().find(|w| w.id == id) {
        *existing_workout = workout.into_inner();
        Some("Workout updated".to_string())
    } else {
        None
    }
}

#[delete("/workouts/<id>")]
async fn delete_workout(id: String, state: &State<WorkoutList>) -> Option<String> {
    let mut workouts = state.lock().unwrap();
    if workouts.iter().any(|w| w.id == id) {
        workouts.retain(|w| w.id != id);
        Some("Workout deleted".to_string())
    } else {
        None
    }
}

#[rocket::main]
async fn main() {
    let list_of_workouts = WorkoutList::new(vec![
        Workout { id: "1".to_string(), name: "Push-ups".to_string(), sets: "3".to_string(), reps: "12".to_string() },
        Workout { id: "2".to_string(), name: "Pull-ups".to_string(), sets: "3".to_string(), reps: "12".to_string() },
    ]);
    
    rocket::build()
        .manage(list_of_workouts)
        .mount("/", routes![get_workouts, get_workout, create_workout, update_workout, delete_workout])
        .launch()
        .await
        .unwrap();
}