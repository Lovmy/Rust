// Creation d'un Web Worker
const worker = new Worker("../../js/worker/web_worker.js");

function sendMessageToWorker()
{
	worker.postMessage( "hello" );
}

function askWorkerRecurringTask()
{
	worker.postMessage( "recurring" );
}

worker.addEventListener("message", function(messageEvent)
{
	document.getElementById( "divWorker" ).innerHTML = messageEvent.data;
});

// Creation d'un service worker
window.addEventListener( "load", () => 
{
	if ("serviceWorker" in navigator)
	{
		navigator.serviceWorker.register( "../../js/worker/service_worker.js" ).then(serviceWorker =>
		{
			console.log( "[load] Service Worker enregistre : ", serviceWorker );
			console.log( "[load] Scope du Service Worker : ", serviceWorker.scope );
		}).catch(error =>
		{
			console.error( "[load] Erreur a l'enregistrement du Service Worker: ", error );
		});
	}
	else
		console.log( "[load] Service Worker non supporte." );
})
