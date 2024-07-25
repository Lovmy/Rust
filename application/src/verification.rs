#![allow(unused_assignments)]
#![allow(dead_code)]

use regex::Regex;

fn test_expression_reguliere( texte: &String, expression_reguliere: String, longueur: usize ) -> bool
{
    let mut resultat = false;
    let regex = Regex::new(&expression_reguliere).unwrap();
    if texte.len() > 0
	{
		if texte.len() > longueur
        {
			resultat = false;
        }
        else
		{
            resultat = regex.is_match(texte);
		}
	}
	else
    {
        resultat = false;
    }
    resultat
}

pub fn test_caracteres_autorises( texte: &String, longueur: usize) -> bool
{
    let expression_reguliere = ["^[A-Za-z0-9&-_|=(°)àáâãäåçèéêëìíîïðòóôõöùúûüýÿ?,.':!@ /]{3,", &longueur.to_string(), "}$"].concat();
    test_expression_reguliere( &texte, expression_reguliere, longueur )
}

pub fn test_email( texte: &String, longueur: usize ) -> bool
{
    let expression_reguliere = "^[^\\W][a-zA-Z0-9_]+(\\.[a-zA-Z0-9_]+)*@[a-zA-Z0-9_]+(\\.[a-zA-Z0-9_]+)*\\.[a-zA-Z]{2,4}$".to_string();
    test_expression_reguliere( &texte, expression_reguliere, longueur )
}

pub fn detecter_email( texte: &String ) -> bool
{
	let mut resultat = false;
	for _trouve in Regex::new(r"([a-zA-Z0-9._-]+@[a-zA-Z0-9._-]+\.[a-zA-Z0-9._-]+)").unwrap().find_iter(texte)
	{
		resultat = true;
	}
	resultat
}

pub fn detecter_numero_telephone( texte: &String ) -> bool
{
	let mut resultat = false;
	for _trouve in Regex::new(r"(?:(?:\+|00)33|0)\s*[1-9](?:[\s.-]*\d{2}){4}").unwrap().find_iter(texte)
	{
		println!("Trouve {:#?}", _trouve.as_str());
		resultat = true;
	}
	resultat
}

pub fn test_date( texte: &String ) -> bool
{
	let expression_reguliere = r"^([0-2]\d|[3][0-1])/([0]\d|[1][0-2])/([2][01]|[1][6-9])\d{2}(\s([0-1]\d|[2][0-3])(:[0-5]\d){1,2})?$".to_string();
    test_expression_reguliere( &texte, expression_reguliere, 19 )
}

pub fn test_nombre( texte: &String ) -> bool
{
	let mut resultat = true;
	if texte.len() > 0
	{
		for caractere in texte.chars()
		{
			if !caractere.is_numeric() && caractere != '.'
			{
				resultat = false;
			}
		}
	}
	else
	{
		resultat = false;
	}
	resultat
}

#[test]
fn _test_caracteres_autorises()
{
	assert_eq!(test_caracteres_autorises( &"abcdef".to_string(), 10 ), true);
	assert_eq!(test_caracteres_autorises( &"abc_def".to_string(), 10 ), true);
	assert_eq!(test_caracteres_autorises( &"abc def".to_string(), 10 ), true);
	assert_eq!(test_caracteres_autorises( &"abc-def".to_string(), 10 ), true);
	assert_eq!(test_caracteres_autorises( &"abc/def".to_string(), 10 ), true);
	assert_eq!(test_caracteres_autorises( &"abc\\def".to_string(), 10 ), true);
	assert_eq!(test_caracteres_autorises( &"abc#def".to_string(), 10 ), false);
	assert_eq!(test_caracteres_autorises( &"abcdefghijklnm".to_string(), 10 ), false);
}

#[test]
fn _test_email()
{
	assert_eq!(test_email( &"toto@toto.com".to_string(), 20 ), true);
	assert_eq!(test_email( &"totoAttoto.com".to_string(), 20 ), false);
	assert_eq!(test_email( &"toto@toto".to_string(), 20 ), false);
	assert_eq!(test_email( &"totototo.com".to_string(), 20 ), false);
	assert_eq!(test_email( &"toto@totocom".to_string(), 20 ), false);
}

#[test]
fn _detecter_email()
{
	assert_eq!(detecter_email( &"Un mail toto@toto.com dans ce texte mail = zozo@zozocom.".to_string() ), true );
	assert_eq!(detecter_email( &"Pas de mail dans ce texte.".to_string() ), false );
}

#[test]
fn _detecter_numero_telephone()
{
	assert_eq!(detecter_numero_telephone( &"Un numéro +33 6 45 78 92 12 dans ce texte".to_string() ), true );
	assert_eq!(detecter_numero_telephone( &"Un numéro 01 23 45 67 89 dans ce texte".to_string() ), true );
	assert_eq!(detecter_numero_telephone( &"Un numéro 0123456789 dans ce texte".to_string() ), true );
	assert_eq!(detecter_numero_telephone( &"Un numéro +33 06 78 54 11 20 dans ce texte.".to_string() ), true );
	assert_eq!(detecter_numero_telephone( &"Pas de numéro +33 dans ce texte.".to_string() ), false );
}

#[test]
fn _test_date()
{
	assert_eq!(test_date( &"31/12/2023 23:59".to_string() ), true);
	assert_eq!(test_date( &"30/12/2002".to_string() ), true);
	assert_eq!(test_date( &"12/01/1998 13:30".to_string() ), true);
	assert_eq!(test_date( &"28/01/2002 22:35:00".to_string() ), true);
	assert_eq!(test_date( &"12/31/2023 00:00".to_string() ), false);
	assert_eq!(test_date( &"31/12/23 00:00".to_string() ), false);
	assert_eq!(test_date( &"1/12/2023 01:01".to_string() ), false);
	assert_eq!(test_date( &"30/13/2002".to_string() ), false);
	assert_eq!(test_date( &"12/01/1998 24:30".to_string() ), false);
	assert_eq!(test_date( &"28/01/2002 22:35:64".to_string() ), false);
}

#[test]
fn _test_nombre()
{
	assert_eq!(test_nombre( &"123".to_string() ), true);
	assert_eq!(test_nombre( &"123.45".to_string() ), true);
	assert_eq!(test_nombre( &"123,45".to_string() ), false);
	assert_eq!(test_nombre( &"123e2".to_string() ), false);
}
