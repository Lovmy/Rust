function get_langue()
{
	var langue = "en";
	if (navigator.browserLanguage)
		var language = navigator.browserLanguage;
	else
		var language = navigator.language;
	if (language.indexOf('fr') > -1 || language.indexOf('FR') > -1)
		langue="fr";
	return( langue );
}

//Detection des parametres sur une URL (IE ne supporte pas le "new URL()")
function isIE()
{
	var ua = window.navigator.userAgent;
	var msie = ua.indexOf('MSIE ');
	if ( msie > 0 )
		return( true );
	var trident = ua.indexOf('Trident/');
	if ( trident > 0 )
		return( true );
	var edge = ua.indexOf('Edge/');
	if ( edge > 0 )
		return( true );
	return false;
}

function get_parametre( nom )
{
	if( isIE() )
	{
		var key = false, res = {}, itm = null;
		// get the query string without the ?
		var qs = location.search.substring(1);
		// check for the key as an argument
		if (arguments.length > 0 && arguments[0].length > 1)
			key = arguments[0];
		// make a regex pattern to grab key/value
		var pattern = /([^&=]+)=([^&]*)/g;
		// loop the items in the query string, either find a match to the argument, or build an object with key/value pairs
		while (itm = pattern.exec(qs))
		{
			if (key !== false && decodeURIComponent(itm[1]) === key)
				return decodeURIComponent(itm[2]);
			else
			{
				if (key === false)
					res[decodeURIComponent(itm[1])] = decodeURIComponent(itm[2]);
			}
		}
		return key === false ? res : null;
	}
	else
	{
		var parsed_url = new URL(window.location.href);
		return( parsed_url.searchParams.get(nom) );
	}
}
