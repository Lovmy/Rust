function test_expression_reguliere( texte, expression_reguliere, longueur )
{
	var regex = RegExp(expression_reguliere);
	if ( texte != null && texte.length > 0 )
	{
		if ( texte.length > longueur )
			return( false );
		else
		{
			if ( regex.test(texte) )
				return( true );
			else
				return( false );
		}
	}
	else
		return( false );
}

function test_caracteres_autorises(texte, longueur)
{
	return test_expression_reguliere( texte, "^[A-Za-z0-9&\\-_|=(°)àáâãäåçèéêëìíîïðòóôõöùúûüýÿ?,.':!@\\ \\/\\\\]{3,"+longueur+"}$", longueur );
}

function test_email( texte, longueur )
{
	return test_expression_reguliere( texte, "^[^\\W][a-zA-Z0-9_]+(\\.[a-zA-Z0-9_]+)*\\@[a-zA-Z0-9_]+(\\.[a-zA-Z0-9_]+)*\\.[a-zA-Z]{2,4}$", longueur );
}

function detecter_email( texte )
{
	var regex = /([a-zA-Z0-9._-]+@[a-zA-Z0-9._-]+.[a-zA-Z0-9._-]+)/gi;
	return regex.test( texte );
}

function detecter_numero_telephone( texte )
{
	var regex = /(?:(?:\+|00)33|0)\s*[1-9](?:[\s.-]*\d{2}){4}/gi;
	return regex.test( texte );
}

function test_date( texte )
{
	return test_expression_reguliere( texte, "^([0-2]\\d|[3][0-1])\\/([0]\\d|[1][0-2])\\/([2][01]|[1][6-9])\\d{2}(\\s([0-1]\\d|[2][0-3])(:[0-5]\\d){1,2})?$", 19 );
}

function test_nombre( texte )
{
	if ( texte != null && texte.length > 0 )
	{
		texte = texte.replace('e', '_');
		if ( isNaN(texte) )
			return( false );
		else
			return( true );
	}
	else
		return( false );
}
