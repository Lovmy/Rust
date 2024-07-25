use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

pub fn test()
{
    let thread_secondaire = thread::spawn( move ||
    {
        for instant in 1..21
        {
            println!( "Instant {} depuis le thread secondaire", instant) ;
            thread::sleep( Duration::from_millis(15) );
        }
    });

    for instant in 1..21
    {
        println!("Instant {} depuis le thread principal", instant );
        thread::sleep( Duration::from_millis(10) );
    }

    thread_secondaire.join().unwrap();
    println!( "*** Fin des Threads classiques ! ***" );

    // Acces concurrent a une variable
    rc_usage();
    arc_usage();
    mutex_usage();

    // Methodes de threading
    channel_usage();
    channel2_usage();
}

fn rc_usage()
{
    let ma_valeur  : Rc<String> = Rc::new("Paris".to_string());

    println!("[RC] longueur de la chaîne : {}", ma_valeur.len());

    // Erreur volontaire de compilation : Rc immutable.
    //ma_valeur.push_str(" est la capitale de la France");

    let reference_1 : Rc<String> = ma_valeur.clone();
    println!("[RC] Compteur de références (2) : {}", Rc::strong_count(&ma_valeur));

    let reference_2 : Rc<String> = ma_valeur.clone();
    println!("[RC] Compteur de références (3) : {}", Rc::strong_count(&ma_valeur));

    let reference_3 : Rc<String> = ma_valeur.clone();
    println!("[RC] Compteur de références (4) : {}", Rc::strong_count(&ma_valeur));

    {
        let _reference_4 : Rc<String> = ma_valeur.clone();
        println!("[RC] Compteur de références (5) : {}", Rc::strong_count(&ma_valeur));
    }
    println!("[RC] Compteur de références (4, reference_4 supprimée) : {}", Rc::strong_count(&ma_valeur));

    println!("[RC] Référence 0 (ma_valeur) : {}", ma_valeur);
 
    drop(ma_valeur);

    // Les references sont garde meme si leur source est detruite !

    println!("[RC] Référence 1 : {}", reference_1);
    println!("[RC] Référence 2 : {}", reference_2);
    println!("[RC] Référence 3 : {}", reference_3);
}

fn arc_usage()
{
    // Vecteur d'entiers.
    let mut vecteur : Vec<i64> = vec![];
    for ii in 0..1000
    {
        vecteur.push(ii);
    }
     
    // Trois références sur le vecteur d'entiers.
    let vecteur_reference_1 : Arc<Vec<i64>> = Arc::new(vecteur);
    let vecteur_reference_2 : Arc<Vec<i64>> = vecteur_reference_1.clone();
    let vecteur_reference_3 : Arc<Vec<i64>> = vecteur_reference_1.clone();
     
    // Vecteur qui contient les trois références.
    let mut vecteur_references : Vec<Arc<Vec<i64>>> = vec![];
    vecteur_references.push(vecteur_reference_1);
    vecteur_references.push(vecteur_reference_2);
    vecteur_references.push(vecteur_reference_3);

     // Lancement d'un thread par référence.
    let mut threads = vec![];
    let mut index : i64 = 1;
    for reference in vecteur_references
    {
        threads.push(thread::spawn(move ||
        {
            let id = thread::current().id();
            println!("[ARC] Longueur du vecteur d'entiers 'vecteur' via la référence {} du thread {:?}: {}", index, id, reference.len());
        }));
        index = index + 1;
    }
     
    // On attend la fin de tous les threads.
    for thread in threads
    {
        let _ret = thread.join();
    }
}

fn mutex_usage()
{
    let compteur = Arc::new(Mutex::new(0));

    let mut threads = vec![];

    for _ in 0..10
    {
        let compteur_reference = Arc::clone(&compteur);
        let thread_courant = thread::spawn( move ||
        {
            let mut num = compteur_reference.lock().unwrap();
            let id = thread::current().id();
            *num += 1;
            println!("[MUTEX] Compteur à {} incrémenté par {:?}.", num, id);
        });
        threads.push(thread_courant);
    }

    for thread in threads
    {
        thread.join().unwrap();
    }

    println!("[MUTEX] Situation courante du compteur : {}.", *compteur.lock().unwrap());
}

fn channel_usage()
{
    let (tx, rx) = channel();

    // Thread qui envoie.
    thread::spawn(move||
    {
        let id = thread::current().id();
        println!("[CHANNEL] A - Je suis le thread qui envoie : {:?}.", id);
        let valeur = String::from("Coucou, ça va ?");
        tx.send(valeur).unwrap();
        // println!("{}.", valeur); // Test erreur de compilation.
    });

    // Thread qui reçoit, par ailleurs thread principal.
    let id = thread::current().id();
    println!("[CHANNEL] B - Je suis le thread qui reçoit : {:?}.", id);
    let message = rx.recv().unwrap();
    println!("[CHANNEL] Message reçu par B envoyé par A : {}", message);
}

fn channel2_usage()
{
    let (tx, rx) = channel();

    // Thread qui envoie.
    thread::spawn(move||
    {
        let  vecteur = vec![ 
            String::from("Coucou"),
            String::from(", "),
            String::from("ça "),
            String::from("va "),
            String::from("?")
        ];

        let id = thread::current().id();
        println!("[CHANNEL2] A - Je suis le thread qui envoie : {:?}.", id);

        let intervalle = Duration::from_secs(2);

        for mot in vecteur
        {
            tx.send(mot).unwrap();
            thread::sleep(intervalle); // Deux secondes avant le prochain mot envoyé.
        }
    });

    // Thread qui reçoit, par ailleurs thread principal.
    let id = thread::current().id();
    println!("[CHANNEL2] B - Je suis le thread qui reçoit : {:?}.", id);
    
    for mot_recu in rx
    {
        println!("[CHANNEL2] 😻 Mot reçu par B envoyé par A : {}", mot_recu);
    }
}
