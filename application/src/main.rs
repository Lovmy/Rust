//	- le_project
//	|
//	|- lib.rs			<- le fichier principal si c'est une librairie, mod un_fichier, mod module1
//	|- main.rs			<- le fichier principal si c'est un binaire
//	|- un_fichier.rs
//	|- module1
//		|
//		|- mod.rs		<- obligatoire on y met les pub mod file & module2
//		|- file.rs
//		|- module2
//			|
//			|- mod.rs	<- obligatoire on y met les pub mod file1
//			|- file1.rs

// SQLite voir comment envoyer ces parametres lors de la connexion
// pragma journal_mode = WAL;
// pragma synchronous = normal;
// pragma temp_store = memory;
// pragma mmap_size = 30000000000;
// pragma auto_vacuum = incremental;	-- once on first DB create
// pragma incremental_vacuum;			-- regularily

#[macro_use]
extern crate rocket;

mod verification;
mod processus;
mod pages;

use std::collections::HashMap;
use std::sync::{ Arc, Mutex, RwLock };
use rocket::State;
use rocket::fs::{ FileServer, relative };
use rocket::response::stream::{ EventStream, Event };
use rocket::tokio::time::{ self, Duration };
use serde::Serialize;
use chrono::{ DateTime, Utc };

const FICHIER_BDD: &str = "/home/serge/sqlite/fichier.db";

#[derive(Clone)]
struct Session
{
	session_id: String,
    utilisateur: String
}

impl Default for Session
{
    fn default() -> Session
	{
        Session
		{
			session_id: "".to_string(),
            utilisateur: "".to_string()
        }
    }
}

struct Sessions
{
	liste: Arc<RwLock<HashMap<String, Mutex<Session>>>>,	// Liste de sessions
	pool: r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>	// Pool de connexion SQL
}

pub fn requete( pool: r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>, contenu: &str ) -> Option<String>
{
	match pool.get()
	{
		Ok(client) => 
		{
			match client.query_row( contenu, [], |row| row.get::<usize, String>(0) )
			{
				Ok(donnee) => 
				{
					return Some(donnee);
				}
				Err(erreur) => 
				{
					println!( "[requete] Erreur requete : {}", erreur );
					return None;
				}
			}
		}
		Err(erreur) => 
		{
			println!( "[requete] Erreur connexion = {}", erreur );
			return None;
		}
	}
}

// Ajax.js envoi par defaut une variable de session et un nombre aleatoire pour eviter le cache.
#[post("/<fonction>/<session_id>/<random>", data = "<body>")]
async fn envoi( session: &State<Sessions>, fonction: &str, session_id: &str, random: &str, body: &str ) -> String
{
	println!( "[envoi] fonction:{} session_id:{} random:{}", fonction, session_id, random );
	let session_liste = Arc::clone(&session.liste);

	// Reponse commune
	#[derive(Serialize)]
	struct Reponse
	{
		etat: usize,
		erreur: usize,
		sessionid: String,
		donnees : serde_json::Value
	}

	let mut reponse = Reponse
	{
		etat: 0,
		erreur: 0,
        sessionid: "".to_string(),
		donnees : serde_json::from_str("[]").unwrap()
    };

	match fonction
	{
		"utilisateur" =>
		{
			pages::authentification::liste(session_liste.clone());

			let s: Session = pages::authentification::verification( body, session_liste, session_id );
			let resultat = pages::utilisateurs::liste( session.pool.clone() );
			reponse.sessionid = s.session_id;
			reponse.donnees = serde_json::from_str(resultat.as_str()).unwrap();	// Texte vers JSON
		}
		_=>
		{
			reponse.erreur = 1;
		}
	}

	serde_json::to_string(&reponse).unwrap()									// JSON vers texte
}

#[get("/donnees/<session_id>")]
fn reception(session: &State<Sessions>, session_id: &str) -> EventStream![]
{
	println!( "[reception] session_id:{}", session_id );
	let _session_liste = Arc::clone(&session.liste);
	EventStream!
	{
		let mut interval = time::interval(Duration::from_secs(1));				// Regler l'intervalle entre deux envois
		loop
		{
			let maintenant: DateTime<Utc> = Utc::now();
			yield Event::data(format!("{{ \"date\": \"{}\" }}", maintenant.format("%d/%m/%Y %H:%M:%S")));
			interval.tick().await;
		}
	}
}

#[launch]
fn rocket() -> _
{
	let bdd_connexion = r2d2_sqlite::SqliteConnectionManager::file(FICHIER_BDD);
    match r2d2::Pool::builder().build(bdd_connexion)
	{
		Ok(bdd_cnx) => 
		{
			processus::test();
			// La fonction rocket retourne la valeur de retour de :
			rocket::build()
			.mount( "/api/envoi", routes![envoi] )
			.mount( "/api/reception", routes![reception] )
			.mount( "/", FileServer::from( relative!( "www" ) ) )
			.manage( Sessions { liste: Arc::new(RwLock::new(HashMap::new())), pool: bdd_cnx } )
		}
		Err(erreur) => 
		{
			panic!( "[rocket] Impossible de lancer le serveur : {}", erreur );
		}
	}
}
