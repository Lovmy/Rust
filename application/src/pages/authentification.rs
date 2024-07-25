use std::{collections::HashMap, sync::{Arc, Mutex, RwLock}};
use serde::Deserialize;
use uuid::Uuid;

use crate::Session;

pub fn verification( body: &str, session_liste: Arc<RwLock<HashMap<String, Mutex<Session>>>>, session_id: &str ) -> Session
{
	#[derive(Deserialize)]
	struct Formulaire
	{
		nom: String,
		prenom: String
	}
	
	impl Default for Formulaire
	{
		fn default() -> Formulaire
		{
			Formulaire
			{
				nom: "".to_string(),
				prenom: "".to_string()
			}
		}
	}
	
	let mut session_utilisateur = Session { ..Default::default() };
	let formulaire: Formulaire = serde_json::from_str(&body).unwrap_or( Formulaire { ..Default::default() } );

	let ok_nom = crate::verification::test_caracteres_autorises( &formulaire.nom, 255 );
	println!( "[verification] ok_nom = {}", ok_nom );

	println!( "[verification] Formulaire nom = {}", formulaire.nom );
	println!( "[verification] Formulaire prenom = {}", formulaire.prenom );

	// Porte du RwLock, un drop est fait sur l'accolade fermante. Lecture de la liste des sessions.
	{
		match session_liste.read()
		{
			Ok(liste) => 
			{
				if liste.contains_key(&session_id.to_string())
				{
					let mutex_session = liste.get(&session_id.to_string());
					if let Some(ms) = mutex_session 
					{
						let mutex_guard = ms.lock().unwrap();
						// Session de l'utilisateur trouve
						session_utilisateur.session_id = mutex_guard.session_id.to_string();
						session_utilisateur.utilisateur = mutex_guard.utilisateur.to_string();
					}
				}
			}
			Err(erreur) => 
			{
				println!( "[verification] Erreur = {}", erreur );
			}
		}
	}

	if session_utilisateur.utilisateur.len() > 0
	{
		println!( "[verification] Session connue, session_id = {}", session_utilisateur.session_id );
		println!( "[verification] Session connue, utilisateur = {}", session_utilisateur.utilisateur );
	}
	else
	{
		// Ecriture dans la liste des sessions
		let mutex_session = session_liste.write();
		match mutex_session
		{
			Ok(mut ms) => 
			{
				session_utilisateur.session_id = Uuid::new_v4().to_string();
				session_utilisateur.utilisateur = formulaire.nom;
				ms.entry(session_utilisateur.session_id.to_string()).or_insert_with(|| Mutex::new(session_utilisateur.clone()));

				println!( "[verification] Nouvelle session, session_id = {}", session_utilisateur.session_id );
				println!( "[verification] Nouvelle session, utilisateur = {}", session_utilisateur.utilisateur );
			}
			Err(erreur) => 
			{
				println!( "[verification] Erreur session = {}", erreur );
			}
		}
	}
	session_utilisateur
}

pub fn liste( session_liste: Arc<RwLock<HashMap<String, Mutex<Session>>>> )
{
	match session_liste.read()
	{
		Ok(liste) => 
		{
			for (cle, valeur) in liste.iter()
			{
				let mutex_session = valeur.lock().expect("[liste] Could not lock mutex");
				println!("[liste] {} : utilisateur {}", cle, mutex_session.utilisateur);
			}
		}
		Err(erreur) => 
		{
			println!( "[liste] Erreur = {}", erreur );
		}
	}
}
