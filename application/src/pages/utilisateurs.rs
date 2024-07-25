pub fn liste( pool: r2d2::Pool<r2d2_sqlite::SqliteConnectionManager> ) -> String
{
	match crate::requete( pool, "SELECT json_group_array(json_object( 'clef', clef, 'nom', nom ) ) as toto FROM utilisateurs" )
	{
		Some( resultat ) =>
		{
			resultat
		}
		None =>
		{
			"[]".to_string()
		}
	}
}
