var reception;
function recepteur( lien, fonction )
{
	this.source = null;
	this.start = function ()
	{
		this.source = new EventSource(lien);
		this.source.onmessage = function(event)
		{
			var f = fonction+'( \"'+event.data.replace(/["]/g, "\\\"")+'\" )';
			eval ( f );
		}
		/* this.source.addEventListener("message", function (event)
		{
			var f = fonction+'( \"'+event.data.replace(/["]/g, "\\\"")+'\" )';
			eval ( f );
		}); */
		this.source.onerror = function ()
		{
			console.log( "Erreur dans la reception des evenements." );
			this.source.close();
		};
	};
	this.stop = function()
	{
		this.source.close();
	}
}
			
function initialisation_event(lien, fonction)
{
	reception = new recepteur(lien, fonction);
	/* window.onload = function()
	{ */
		reception.start();
	// };
	window.onbeforeunload = function()
	{
		reception.stop();
	}
}
