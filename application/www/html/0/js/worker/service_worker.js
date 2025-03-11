const VERSION = "0.0.1a";
const FICHIERS_EN_CACHE = [
	"main.js",
	"web_worker.js",
	"service_worker.js",
	"../ajax.js",
	"../event.js",
	"../langue.js",
	"../popup.js",
	"../tableau.js",
	"../verification.js",
	"../../css/couleurs.css",
	"../../css/popup.css",
	"../../css/style.css",
	"../../css/tableau.css",
	"../../images/192.png",
	"../../images/512.png",
	"../../langue/fr/index.html",
	"../../wasm/web_assembly.wasm",
	"../../../../favicon.ico",
	"../../../../index.html"
];

self.addEventListener("install", event => {
	console.log( "[install] Service Worker installing.");
	event.waitUntil ( caches.open( VERSION ).then( cache => cache.addAll( FICHIERS_EN_CACHE ) ) );
});

self.addEventListener("activate", event => {
	console.log("[activate] Service Worker activating.");
	// Suppression du cache obsolete
	event.waitUntil ( caches
		.keys()
		.then(keys => keys.filter(key => key !== VERSION))
		.then(keys =>
		Promise.all ( keys.map(key => {
			console.log(`[activate] Deleting cache ${key}`);
			return caches.delete(key);
		}))
	));
});

self.addEventListener("fetch", event => {
	console.log("[fetch] ");
	// Stratégie Cache-First
	event.respondWith( caches
		.match( event.request)								// On vérifie si la requête a déjà été mise en cache
		.then( cached => cached || fetch(event.request))	// sinon on requête le réseau
		.then( response => cache(event.request, response)	// on met à jour le cache
		.then(() => response)								// et on résout la promesse avec l'objet Response
	));
});

function cache(request, response)
{
	if (response.type === "error" || response.type === "opaque")
	{
		return Promise.resolve(); // do not put in cache network errors
	}
	return caches.open(VERSION).then(cache => cache.put(request, response.clone()));
}
