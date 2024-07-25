async function addition(a, b)
{
	let resultat = 0;
	let reponse = await fetch('../../wasm/web_assembly.wasm');
	let bytes = await reponse.arrayBuffer();
	let { instance } = await WebAssembly.instantiate(bytes, { });
	resultat = instance.exports.addition(a, b);
	return resultat;
 }

self.addEventListener( "message", function(messageEvent)
{
	let a = parseInt(2);
    let b = parseInt(3);

	if ( messageEvent.data === "hello" )
	{
		addition(a, b).then(res =>
		{
			self.postMessage( "Resultat="+res );
		})
		.catch(e =>
		{
			console.log('On a eu un problème : ' + e.message);
		});
		self.postMessage( "Attente..." );
	}

	if ( messageEvent.data === "recurring" )
	{
		for ( let i = 0; i < 10; i++ )
		{
			setTimeout(() => self.postMessage(new Date()), i * 1000);
		}
	}
});
