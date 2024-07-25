"use strict";

let session_id = 0;

function get_ajax()
{
	let objet_ajax;
	try
	{
		objet_ajax = new ActiveXObject('Msxml2.XMLHTTP');
	}
	catch (e)
	{
		try
		{
			objet_ajax = new ActiveXObject('Microsoft.XMLHTTP');
		}
		catch (e2)
		{
			try
			{
				objet_ajax = new XMLHttpRequest();
			}
			catch (e3)
			{
				objet_ajax = false;
			}
		}
	}
	return( objet_ajax );
}

function set_session_id( valeur )
{
	ecrire_local_storage( "session_id", valeur );
}

function ajouter_parametre( page, parametre )
{
	// if ( page.indexOf("?") != -1 )
	//	page += "&";			
	// else
	//	page += "?";			
	page += "/"+parametre;
	return( page );
}

function requete_ajax_json( page, fonction, donnees )
{
	let objet_ajax = null;
	let doc = null;
	let methode = "GET";
	if ( donnees != null )
	{
		methode = "POST";
		// donnees = encodeURI(donnees);	// A tester...
	}
	if ( page == null || page == "" )
		page = document.location.href;
	// page = ajouter_parametre( page, "token="+token );
	// page = ajouter_parametre( page, "toto="+(Math.random()+""+Date.now()).replace('.','') );
	page = ajouter_parametre( page, lire_local_storage("session_id") );
	page = ajouter_parametre( page, (Math.random()+""+Date.now()).replace('.','') );
	page = encodeURI(page);
	if ( objet_ajax = get_ajax() )
	{
		objet_ajax.onreadystatechange = function()
		{
			if ( objet_ajax.readyState == 4 )
			{
				if ( objet_ajax.status == 200 )
				{
					try
					{
						doc = objet_ajax.responseText;
						if ( doc != null )
						{
							var reg = new RegExp("\"", "g" );
							doc = doc.replace( reg, "\\\"");
							doc = doc.replace( /\n/g, "" );
							var f = fonction+`( "`+doc+`" )`;
							eval ( f );
						}
					}
					catch(erreur)
					{
						var f = fonction+`( '{ \"etat\" : \"1\", \"erreur\" : \"`+erreur+`\" }' )`;
						eval ( f );
					}
				}
				else
				{
					if( objet_ajax.status == 0 )
					{
						var f = fonction+`( '{ \"etat\" : \"2\", \"erreur\" : \"`+objet_ajax.status+`\" }' )`;
						eval ( f );
					}
					else
					{
						var f = fonction+`( '{ \"etat\" : \"3\", \"erreur\" : \"`+objet_ajax.status+`\" }' )`;
						eval ( f );
					}
				}
			}
		}
		objet_ajax.open( methode, page );
		objet_ajax.setRequestHeader("Content-Type","application/x-www-form-urlencoded");
		objet_ajax.send( donnees );
	}
}

function ecrire_cookie( nom, valeur )
{
	var jours = 256;
	var fin = "";
	if (jours)
	{
		var date = new Date();
		date.setTime(date.getTime()+(jours*24*60*60*1000));
		fin = "; expires="+date.toGMTString();
	}
	document.cookie = nom+"="+valeur+fin+"; path=/";
}

function lire_cookie( nom )
{
	var nameEQ = nom + "=";
	var ca = document.cookie.split(';');
	for(var i=0;i < ca.length;i++)
	{
		var c = ca[i];
		while (c.charAt(0)==' ') c = c.substring(1,c.length);
		if (c.indexOf(nameEQ) == 0)
			return c.substring(nameEQ.length,c.length);
	}
}

function efface_cookie( nom )
{
	ecrire_cookie( nom, "", -1 );
}

function ecrire_local_storage( nom, valeur )
{
	localStorage.setItem( nom, valeur );
}

function lire_local_storage( nom )
{
	var donnees_client = localStorage.getItem( nom );
	if ( donnees_client != null )
		return( donnees_client );
	return null;
}

function efface_local_storage( nom )
{
	localStorage.removeItem( nom );
}

function ecrire_session_storage( nom, valeur )
{
	sessionStorage.setItem( nom, valeur );
}

function lire_session_storage( nom )
{
	var donnees_client = sessionStorage.getItem( nom );
	if ( donnees_client != null )
		return( donnees_client );
	return null;
}

function efface_session_storage( nom )
{
	sessionStorage.removeItem( nom );
}

function replace_all( recherche, remplacement, chaine_a_modifier )
{
	if ( chaine_a_modifier != null )
	{
		// var chaine = chaineAModifier.replace(/[.*+?^${}()|[]\]/g, \$&);
		let chaine = chaine_a_modifier;
		let re = new RegExp( recherche, 'g' );
		return chaine.replace( re, remplacement );
	}
	else
		return( "" );
}
