function boite_dialogue( titre, contenu, ajout )
{
	var html  = "<div>";
	    html += "<header class=\"enteteDiv\">";
	    html += "<center><b>"+titre+"</b></center>";
	    html += "</header>";
	    html += "<p>"+contenu+"</p>";
	    html += "<footer>";
	    html += "<center>";
	    html += "<a href=\"javascript:boite_dialogue_reponse(0)\" class=\"lien\">Fermer</a>";
	    if ( ajout )
	    	html += "&nbsp;&nbsp;"+ajout;
	    html += "</center>";
	    html += "</footer>";
	    html += "</div>";
	document.getElementById("popup").innerHTML = html;
	location.href = "#popup";
}

function fermer_boite_dialogue()
{
	boite_dialogue_reponse(0);	
}

function boite_dialogue_reponse( code )
{
	switch( code )
	{
		case 0:
			// actionEntree = null;
			location.href = "#";
			break;
		default:
			
			break;
	}
}
