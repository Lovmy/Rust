const separateur_entete = "|";

function json2tableau_utilisateurs( objet_json, entete )
{
	var html = "<table class=\"listeTABLE\" border=\"1\">";
	if ( typeof entete !== 'undefined')
	{
		html += "<tr class=\"listeTR\">";
		const colonne = entete.split(separateur_entete);
		for (var i = 0; i < colonne.length; i++)
		{
			html += "<th class=\"listeTH\">"+colonne[i]+"</th>";
		}
		html += "</tr>";
	}
	for (var i = 0; i < objet_json.length; i++)
	{
		let objet = objet_json[i];
		html += "<tr class=\"listeTR\">";
		html += "<td class=\"listeTD\">"+objet.clef+"</td>";
		html += "<td class=\"listeTD\">"+objet.nom+"</td>";
		html += "<td class=\"listeTD\"><a href=\"javascript:click_bouton_libelle( '"+objet.clef+"' )\" class=\"lien\">libelle</a></td>";
		html += "</tr>";
	}
	html += "</table>";
	return ( html );
}
