/* Qwik Service Worker */
const appBundles=[["q-1504dcfa.js",[7]],["q-1d4e3858.js",[]],["q-21361a47.js",[7]],["q-3e577dc2.js",[7],["ceU05TscGYE","N39ca0w8E8Y"]],["q-4834acc3.js",[7,21],["nd8yk3KO22c"]],["q-4b916990.js",[29]],["q-4bf12bfa.js",[7,12,29],["bHjzplUVBNo","x04JC5xeP1U","zH94hIe0Ick"]],["q-5bbf9964.js",[]],["q-64d3db07.js",[29]],["q-6605d321.js",[5,8,29]],["q-73461fc9.js",[7],["VkLNXphUh5s"]],["q-74c9cd41.js",[5,29]],["q-84340f44.js",[29]],["q-8c033291.js",[7]],["q-914a6c6a.js",[5,7,8,11,29],["2XxI5peAEOQ"]],["q-91f13216.js",[7]],["q-99fe9dc0.js",[7,21],["xYL1qOwPyDI"]],["q-a2d630bc.js",[7,8,29],["GmEZMGhx0kI"]],["q-b35e44bd.js",[7,21],["hA9UPaY8sNQ","mYsiJcA4IBc","skxgNVWVOT8","uVE5iM9H73c"]],["q-b7f61013.js",[7],["6LYztwGzxAA"]],["q-b8750d52.js",[5,7,8,9,29],["Hd8r0YexMcY"]],["q-cb129d7f.js",[7],["3sccYCDd1Z0","hO3b5j0m2ZI"]],["q-cb7336ca.js",[7,21],["dznIGAlrcag","OW4nu0I1yZ8","p4UiTwsJc7c","V0Y6u0VD1eY"]],["q-d2a1530f.js",[7]],["q-d3eac568.js",[7]],["q-d9dd92b1.js",[7,21],["zrbrqoaqXSY"]],["q-dc2d4886.js",[7]],["q-ea375946.js",[5,7,8,9,11,12,29],["LQzPP0B5Rtk"]],["q-ed292abc.js",[7,21],["AaAlzKH0KlQ","z1nvHyEppoI"]],["q-f0b8da33.js",[7],["0TpDEaIm2Eg","ajMyuRH1aws","CwHr1soCFpo","DVMPoyljbY8","i4frY0Pt5lk"]]];
const libraryBundleIds=[24];
const linkBundles=[[/^\/qwik-test\/$/,[13,10,2,16]],[/^\/qwik-test\/flower\/?$/,[13,10,26,22]],[/^\/qwik-test\/react\/?$/,[13,10,0,29]]];
const m="QwikBuild",k=new Set,g=new Map,u=[],E=(e,n)=>n.filter(s=>!e.some(i=>s.endsWith(i[0]))),q=(e,n)=>!!n&&!v(e)&&!v(n),v=e=>{const n=e.headers.get("Cache-Control")||"";return n.includes("no-cache")||n.includes("max-age=0")},C=(e,n)=>e.some(s=>n.endsWith("/"+s[0])),U=(e,n)=>e.find(s=>s[0]===n),b=(e,n)=>n.map(s=>e[s]?e[s][0]:null),W=(e,n)=>n.map(s=>e.get(s)).filter(s=>s!=null),p=e=>{const n=new Map;for(const s of e){const i=s[2];if(i)for(const c of i)n.set(c,s[0])}return n},A=(e,n,s,i)=>new Promise((c,h)=>{const t=i.url,r=s.get(t);if(r)r.push([c,h]);else{const o=l=>{const a=s.get(t);if(a){s.delete(t);for(const[d]of a)d(l.clone())}else c(l.clone())},f=l=>{const a=s.get(t);if(a){s.delete(t);for(const[d,L]of a)L(l)}else h(l)};s.set(t,[[c,h]]),e.match(t).then(l=>{if(q(i,l))o(l);else return n(i).then(async a=>{a.ok&&await e.put(t,a.clone()),o(a)})}).catch(l=>e.match(t).then(a=>{a?o(a):f(l)}))}}),y=(e,n,s,i,c,h=!1)=>{const t=()=>{for(;u.length>0&&g.size<6;){const o=u.shift(),f=new Request(o);k.has(o)?t():(k.add(o),A(n,s,g,f).finally(t))}},r=o=>{try{const f=U(e,o);if(f){const l=b(e,f[1]),a=new URL(o,i).href,d=u.indexOf(a);d>-1?h&&(u.splice(d,1),u.unshift(a)):h?u.unshift(a):u.push(a),l.forEach(r)}}catch(f){console.error(f)}};Array.isArray(c)&&c.forEach(r),t()},w=(e,n,s,i,c,h,t)=>{try{y(e,i,c,h,b(e,n))}catch(r){console.error(r)}for(const r of t)try{for(const o of s){const[f,l]=o;if(f.test(r)){y(e,i,c,h,b(e,l));break}}}catch(o){console.error(o)}},B=(e,n,s,i)=>{try{const c=i.href.split("/"),h=c[c.length-1];c[c.length-1]="";const t=new URL(c.join("/"));y(e,n,s,t,[h],!0)}catch(c){console.error(c)}},N=(e,n,s,i)=>{const c=e.fetch.bind(e),h=p(n);e.addEventListener("fetch",t=>{const r=t.request;if(r.method==="GET"){const o=new URL(r.url);C(n,o.pathname)&&t.respondWith(e.caches.open(m).then(f=>(B(n,f,c,o),A(f,c,g,r))))}}),e.addEventListener("message",async({data:t})=>{if(t.type==="qprefetch"&&typeof t.base=="string"){const r=await e.caches.open(m),o=new URL(t.base,e.origin);Array.isArray(t.links)&&w(n,s,i,r,c,o,t.links),Array.isArray(t.bundles)&&y(n,r,c,o,t.bundles),Array.isArray(t.symbols)&&y(n,r,c,o,W(h,t.symbols))}}),e.addEventListener("activate",async()=>{try{const t=await e.caches.open(m),o=(await t.keys()).map(l=>l.url),f=E(n,o);await Promise.all(f.map(l=>t.delete(l)))}catch(t){console.error(t)}})},x=()=>{typeof self<"u"&&typeof appBundles<"u"&&N(self,appBundles,libraryBundleIds,linkBundles)};x();addEventListener("install",()=>self.skipWaiting());addEventListener("activate",()=>self.clients.claim());
