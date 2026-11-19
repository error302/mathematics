// AXIOM Mathematics Academy — Service Worker (Section 23.2)
const CACHE_NAME = 'axiom-shell-v3-challenges';
const SHELL_ASSETS = [
  '/',
  '/index.html',
  '/manifest.webmanifest',
  '/icon.svg',
  '/logo.png',
  '/axiom-certificate-specimen.html',
];

self.addEventListener('install', (event) => {
  event.waitUntil(
    caches.open(CACHE_NAME).then((cache) => {
      return cache.addAll(SHELL_ASSETS);
    })
  );
  // Do not replace an active learning session. A waiting update activates after tabs close.
});

self.addEventListener('activate', (event) => {
  event.waitUntil(
    caches.keys().then((keys) => {
      return Promise.all(
        keys.filter((key) => key.startsWith('axiom-shell-') && key !== CACHE_NAME).map((key) => caches.delete(key))
      );
    })
  );
});

self.addEventListener('fetch', (event) => {
  const url = new URL(event.request.url);

  if (event.request.method !== 'GET' || url.origin !== self.location.origin) return;
  if (url.pathname.startsWith('/api/') || url.pathname.startsWith('/auth/')) return;
  const isStaticAsset = SHELL_ASSETS.includes(url.pathname) || /^\/assets\/[^/]+-[A-Za-z0-9_-]+\.(js|css|wasm|woff2?|png|svg)$/.test(url.pathname);
  if (event.request.mode === 'navigate' && url.pathname !== '/axiom-certificate-specimen.html') {
    if (url.pathname !== '/' && url.pathname !== '/index.html') return;
    event.respondWith(fetch(event.request).then(response => {
      if (response.ok) { const clone = response.clone(); event.waitUntil(caches.open(CACHE_NAME).then(cache => cache.put('/index.html', clone))); }
      return response;
    }).catch(() => caches.match('/index.html')));
    return;
  }
  // An allowlist prevents authenticated exports, private pages, and arbitrary responses entering this cache.
  if (!isStaticAsset) return;

  // Cache-first with network fallback for static shell assets
  event.respondWith(
    caches.match(event.request).then((cachedResponse) => {
      if (cachedResponse) {
        return cachedResponse;
      }
      return fetch(event.request).then((networkResponse) => {
        if (networkResponse && networkResponse.status === 200 && networkResponse.type === 'basic') {
          const responseToCache = networkResponse.clone();
          event.waitUntil(caches.open(CACHE_NAME).then((cache) => {
            return cache.put(event.request, responseToCache);
          }));
        }
        return networkResponse;
      });
    })
  );
});
