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

#![allow(unused_imports)]
#![allow(dead_code)]

#[macro_use]
extern crate rocket;

mod verification;
mod processus;
mod pages;
mod parallele;

use std::collections::HashMap;
use std::sync::{ Arc, Mutex, RwLock };
use rocket::State;
use rocket::fs::FileServer;
use rocket::response::stream::{ EventStream, Event };
use rocket::tokio::time::{ self, Duration };
use serde::Serialize;
use chrono::{ DateTime, Utc };

use sqlx::{Encode, Row};
use crate::rocket::futures::TryStreamExt;

const FICHIER_BDD: &str = "fichier.db";

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
	liste: Arc<RwLock<HashMap<String, Mutex<Session>>>>,		// Liste de sessions
	pool: r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>		// Pool de connexion SQLite
//	pool: sqlx::Pool<sqlx::Postgres>							// Pool de connexion PostgreSQL
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

//			SELECT pg_sleep(10);	postgres 5433

//			let mut resultat = String::from("[]");
//			let mut rows = sqlx::query("SELECT json_agg(alias)::text as donnees FROM (select clef, nom from utilisateurs) alias").fetch(&session.pool.clone());
//			while let row = rows.try_next().await 
//			{
//				match row
//				{
//					Ok(option_pg_row) => {
//						match option_pg_row
//						{
//							Some(ligne) => 
//							{
//								resultat = ligne.try_get("donnees").unwrap();
//							},
//							None => break
//
//						}
//					},
//					Err(erreur) => println!("[requete] Erreur {}", erreur )
//				}
//			}

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

// #[launch]
//async fn rocket() -> _
//{
//	let bdd_connexion= sqlx::PgPool::connect("postgresql://postgres@localhost:5432/postgres").await;
//	match bdd_connexion

//	let bdd_connexion = r2d2_sqlite::SqliteConnectionManager::file(FICHIER_BDD);
//	match r2d2::Pool::builder().build(bdd_connexion)
//	{
//		Ok(bdd_cnx) => 
//		{
//			processus::test();
//			parallele::test();
//			// La fonction rocket retourne la valeur de retour de :
//			rocket::build()
//			.mount( "/api/envoi", routes![envoi] )
//			.mount( "/api/reception", routes![reception] )
//			.mount( "/", FileServer::from( "www" ) )
//			.manage( Sessions { liste: Arc::new(RwLock::new(HashMap::new())), pool: bdd_cnx } )
//		}
//		Err(erreur) => 
//		{
//			panic!( "[rocket] Impossible de lancer le serveur : {}", erreur );
//		}
//	}
//}

fn main()
{
	/* let args: Vec<String> = std::env::args().collect();
	println!( "Arguments : {:?} 0={}", args, &args[0] );
	let mut toto = 5;

	{	
		let mut calcul_racine = Cache::nouveau( |nombre: u32|
		{
			toto = toto + 1;
			nombre * nombre + toto
		});

		// println!( "toto = {}", toto );

		println!( "Resultat ={}", calcul_racine.valeur(3) );
		println!( "Resultat ={}", calcul_racine.valeur(2) );
		println!( "Resultat ={}", calcul_racine.valeur(3) );
	}

	let objet: Option<String> = None;	// Some(String::from("hello world"));
	if let Some(obj) = objet	// Le if objet != NULL :o)
	{
		println!( "objet = {}", obj );
	}
	else
	{
		println!( "objet est null !" );
	}
	
	let age: Result<u8, _> = "34".parse();
	if let Ok(valeur_age) = age
	{
		println!( "Parse ok = {}", valeur_age );
	}

	let mut pile = Vec::new();

	pile.push(1);
	pile.push(2);
	pile.push(3);

	while let Some(donnee_du_haut) = pile.pop()
	{
		println!("{}", donnee_du_haut);
	}

	let v = vec!['a', 'b', 'c'];
	for (indice, valeur) in v.iter().enumerate()
	{
		println!("{} est à l'indice {}", valeur, indice);
	}

	processus::test();

	// let mut toto = 456;
	let mut toto: String = "Hello".to_string();

	// Si move, alors le toto apres l'appel à test = 456, sinon il est egal à 1456
	// Avec un String, si move alors la variable n'est plus disponible ensuite !
	let mut test = /*move*/ |nombre: u32|
	{
		//toto = toto + 1000;
		toto = "World".to_string();
		println!("nombre = {}, toto = {}", nombre, toto);
	};

	test(132);

	println!("toto = {}", toto);
	*/

	// SQLite
	/* let bdd_connexion = r2d2_sqlite::SqliteConnectionManager::file(FICHIER_BDD);
	match r2d2::Pool::builder().build(bdd_connexion)
	{
		Ok(bdd_cnx) => 
		{
			let mut date = chrono::Local::now();
			println!("DEBUT {} SIZE {}", date.format( "%H:%M:%S" ), bdd_cnx.max_size());
			let valeur = rocket::tokio::runtime::Runtime::new().unwrap().block_on(list_fonction_asynchone(bdd_cnx));
			date = chrono::Local::now();
			println!( "FIN {} Valeur = {}", date.format( "%H:%M:%S" ), valeur );
		}
		Err(erreur) => 
		{
			panic!( "Impossible de se connecter à la base de données : {}", erreur );
		}
	} */

	// PostgreSQL
	// let bdd_connexion = rocket::tokio::runtime::Runtime::new().unwrap().block_on(sqlx::PgPool::connect("postgresql://postgres@localhost:5433/postgres"));
	let bdd_connexion = rocket::tokio::runtime::Runtime::new().unwrap().block_on(sqlx::postgres::PgPoolOptions::new().max_connections(5).connect("postgresql://postgres@localhost:5433/postgres"));
	match bdd_connexion
	{
		Ok(bdd_connexion) => 
		{
			let mut date = chrono::Local::now();
			println!("DEBUT {} SIZE {}", date.format( "%H:%M:%S" ), bdd_connexion.size() );
			let valeur = rocket::tokio::runtime::Runtime::new().unwrap().block_on(list_fonction_asynchone(bdd_connexion));
			date = chrono::Local::now();
			println!( "FIN {} Valeur = {}", date.format( "%H:%M:%S" ), valeur );
		}
		Err(erreur) => 
		{
			panic!( "Impossible de se connecter à la base de données : {}", erreur );
		}
	}
}

async fn list_fonction_asynchone(pool: sqlx::Pool<sqlx::Postgres>) -> u32
// async fn list_fonction_asynchone(pool: r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>) -> u32
{
	// let resultat1 = fonction_asynchone(pool.clone()).await;
	// let resultat2 = fonction_asynchone(pool.clone()).await;

	let tache1 = fonction_asynchone(pool.clone());
	let tache2 = fonction_asynchone(pool.clone());
	println!( "FIN APPELS" );
	let (resultat1, resultat2) = rocket::tokio::join!(tache1, tache2);

	resultat1 + resultat2
}

async fn fonction_asynchone(pool: sqlx::Pool<sqlx::Postgres>) -> u32
{
	println!( "FONCTION ASYNCHRONE" );
	let rows = sqlx::query("SELECT pg_sleep(2) || 'toto' as resultat").map(|row: sqlx::postgres::PgRow| 
	{
		let resultat: String = row.get("resultat");
		resultat
	}).fetch_all(&pool).await;
	println!( "MILIEU FONCTION ASYNCHRONE" );
	if let Ok(mut resultat) = rows 
	{
		println!( "Taille avant {}", resultat.len());
		println!( "Resultat: {:?}", resultat.pop());
		println!( "Taille apres {}", resultat.len());
	}
	println!( "FIN FONCTION ASYNCHRONE" );
	42
}

/* async fn fonction_asynchone(pool: std::option::Option<r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>>) -> u32
{
	println!( "FONCTION ASYNCHRONE" );

	// time::sleep(Duration::from_secs(5)).await; // Simuler un délai réseau
	
	let requete = "WITH RECURSIVE r(i) AS ( VALUES(0) UNION ALL SELECT i FROM r LIMIT 10000000 ) SELECT i FROM r WHERE i = 1";
	match pool
	{
		Some(client) => 
		{
			println!( "A" );
			match client.query_row( requete, [], |row| row.get::<usize, String>(0) )
			{
				Ok(_donnee) => 
				{
					println!( "OK REQUETE" );
				}
				Err(erreur) => 
				{
					println!( "{}", erreur );
				}
			}
		}
		None => 
		{
			println!( "NONE" );
		}
	}

	42
} */

// Toutes les fermetures implémentent au moins un des traits suivants : Fn (emprunt), FnMut(emprunte des valeurs de manière mutable) ou FnOnce (consomme)
struct Cache<T>	// T = La fonction anonyme
where
	T: FnMut(u32) -> u32,
{
	calcul: T,
	valeur: HashMap<u32, u32>,
}

impl<T> Cache<T>
where
	T: FnMut(u32) -> u32	// La fermeture a un paramètre de type u32 et renvoie un u32, le trait lié que nous précisons est Fn (u32) -> u32
{							// On precise que le type générique T est une fermeture en utilisant le trait Fn
	fn nouveau(calcul: T) -> Cache<T>
	{
		Cache
		{
			calcul,
			valeur: HashMap::new(),
		}
	}

	fn valeur(&mut self, arg: u32) -> u32	// self = La structure, presence de self = Methode et non fonction.
	{
		match self.valeur.get(&arg)
		{
			Some(v) => 
			{	
				println!( "*** Trouve en cache" );
				*v
			},
			None =>
			{
				println!( "*** Calculé" );
				let v = (self.calcul)(arg);
				self.valeur.insert(arg, v);
				v
			},
		}
	}
}

pub fn rechercher<'a>(recherche: &str, contenu: &'a str) -> Vec<&'a str>
{
//	let mut resultats = Vec::new();
//	for ligne in contenu.lines()
//	{
//		if ligne.contains(recherche)
//		{
//			resultats.push(ligne);
//		}
//	}
//	resultats

	contenu.lines().filter(|ligne| ligne.contains(recherche)).collect()
}	