var Ze=Object.defineProperty;var xe=(t,e,l)=>e in t?Ze(t,e,{enumerable:!0,configurable:!0,writable:!0,value:l}):t[e]=l;var U=(t,e,l)=>(xe(t,typeof e!="symbol"?e+"":e,l),l);(function(){const e=document.createElement("link").relList;if(e&&e.supports&&e.supports("modulepreload"))return;for(const r of document.querySelectorAll('link[rel="modulepreload"]'))n(r);new MutationObserver(r=>{for(const s of r)if(s.type==="childList")for(const o of s.addedNodes)o.tagName==="LINK"&&o.rel==="modulepreload"&&n(o)}).observe(document,{childList:!0,subtree:!0});function l(r){const s={};return r.integrity&&(s.integrity=r.integrity),r.referrerpolicy&&(s.referrerPolicy=r.referrerpolicy),r.crossorigin==="use-credentials"?s.credentials="include":r.crossorigin==="anonymous"?s.credentials="omit":s.credentials="same-origin",s}function n(r){if(r.ep)return;r.ep=!0;const s=l(r);fetch(r.href,s)}})();function k(){}function fe(t,e){for(const l in e)t[l]=e[l];return t}function et(t){return!!t&&(typeof t=="object"||typeof t=="function")&&typeof t.then=="function"}function Ge(t){return t()}function pe(){return Object.create(null)}function le(t){t.forEach(Ge)}function Je(t){return typeof t=="function"}function z(t,e){return t!=t?e==e:t!==e||t&&typeof t=="object"||typeof t=="function"}let se;function O(t,e){return se||(se=document.createElement("a")),se.href=e,t===se.href}function tt(t){return Object.keys(t).length===0}function Ve(t,...e){if(t==null)return k;const l=t.subscribe(...e);return l.unsubscribe?()=>l.unsubscribe():l}function lt(t){let e;return Ve(t,l=>e=l)(),e}function ie(t,e,l){t.$$.on_destroy.push(Ve(e,l))}function Y(t,e,l,n){if(t){const r=Ke(t,e,l,n);return t[0](r)}}function Ke(t,e,l,n){return t[1]&&n?fe(l.ctx.slice(),t[1](n(e))):l.ctx}function Q(t,e,l,n){if(t[2]&&n){const r=t[2](n(l));if(e.dirty===void 0)return r;if(typeof r=="object"){const s=[],o=Math.max(e.dirty.length,r.length);for(let i=0;i<o;i+=1)s[i]=e.dirty[i]|r[i];return s}return e.dirty|r}return e.dirty}function X(t,e,l,n,r,s){if(r){const o=Ke(e,l,n,s);t.p(o,r)}}function Z(t){if(t.ctx.length>32){const e=[],l=t.ctx.length/32;for(let n=0;n<l;n++)e[n]=-1;return e}return-1}function ve(t){const e={};for(const l in t)l[0]!=="$"&&(e[l]=t[l]);return e}function nt(t){const e={};for(const l in t)e[l]=!0;return e}function d(t,e){t.appendChild(e)}function R(t,e,l){t.insertBefore(e,l||null)}function C(t){t.parentNode&&t.parentNode.removeChild(t)}function he(t,e){for(let l=0;l<t.length;l+=1)t[l]&&t[l].d(e)}function _(t){return document.createElement(t)}function I(t){return document.createTextNode(t)}function q(){return I(" ")}function Ye(){return I("")}function de(t,e,l,n){return t.addEventListener(e,l,n),()=>t.removeEventListener(e,l,n)}function u(t,e,l){l==null?t.removeAttribute(e):t.getAttribute(e)!==l&&t.setAttribute(e,l)}function st(t){return Array.from(t.childNodes)}function rt(t,e,l,n){l===null?t.style.removeProperty(e):t.style.setProperty(e,l,n?"important":"")}function ke(t,e,l){t.classList[l?"add":"remove"](e)}let ne;function D(t){ne=t}function me(){if(!ne)throw new Error("Function called outside component initialization");return ne}function ot(t,e){return me().$$.context.set(t,e),e}function it(t){return me().$$.context.get(t)}function at(t,e){const l=t.$$.callbacks[e.type];l&&l.slice().forEach(n=>n.call(this,e))}const te=[],ye=[],re=[],we=[],ct=Promise.resolve();let _e=!1;function ut(){_e||(_e=!0,ct.then(be))}function ge(t){re.push(t)}const ce=new Set;let x=0;function be(){if(x!==0)return;const t=ne;do{try{for(;x<te.length;){const e=te[x];x++,D(e),ft(e.$$)}}catch(e){throw te.length=0,x=0,e}for(D(null),te.length=0,x=0;ye.length;)ye.pop()();for(let e=0;e<re.length;e+=1){const l=re[e];ce.has(l)||(ce.add(l),l())}re.length=0}while(te.length);for(;we.length;)we.pop()();_e=!1,ce.clear(),D(t)}function ft(t){if(t.fragment!==null){t.update(),le(t.before_update);const e=t.dirty;t.dirty=[-1],t.fragment&&t.fragment.p(t.ctx,e),t.after_update.forEach(ge)}}const oe=new Set;let K;function J(){K={r:0,c:[],p:K}}function V(){K.r||le(K.c),K=K.p}function h(t,e){t&&t.i&&(oe.delete(t),t.i(e))}function S(t,e,l,n){if(t&&t.o){if(oe.has(t))return;oe.add(t),K.c.push(()=>{oe.delete(t),n&&(l&&t.d(1),n())}),t.o(e)}else n&&n()}function Se(t,e){const l=e.token={};function n(r,s,o,i){if(e.token!==l)return;e.resolved=i;let c=e.ctx;o!==void 0&&(c=c.slice(),c[o]=i);const a=r&&(e.current=r)(c);let g=!1;e.block&&(e.blocks?e.blocks.forEach((p,y)=>{y!==s&&p&&(J(),S(p,1,1,()=>{e.blocks[y]===p&&(e.blocks[y]=null)}),V())}):e.block.d(1),a.c(),h(a,1),a.m(e.mount(),e.anchor),g=!0),e.block=a,e.blocks&&(e.blocks[s]=a),g&&be()}if(et(t)){const r=me();if(t.then(s=>{D(r),n(e.then,1,e.value,s),D(null)},s=>{if(D(r),n(e.catch,2,e.error,s),D(null),!e.hasCatch)throw s}),e.current!==e.pending)return n(e.pending,0),!0}else{if(e.current!==e.then)return n(e.then,1,e.value,t),!0;e.resolved=t}}function dt(t,e,l){const n=e.slice(),{resolved:r}=t;t.current===t.then&&(n[t.value]=r),t.current===t.catch&&(n[t.error]=r),t.block.p(n,l)}function G(t){t&&t.c()}function M(t,e,l,n){const{fragment:r,after_update:s}=t.$$;r&&r.m(e,l),n||ge(()=>{const o=t.$$.on_mount.map(Ge).filter(Je);t.$$.on_destroy?t.$$.on_destroy.push(...o):le(o),t.$$.on_mount=[]}),s.forEach(ge)}function T(t,e){const l=t.$$;l.fragment!==null&&(le(l.on_destroy),l.fragment&&l.fragment.d(e),l.on_destroy=l.fragment=null,l.ctx=[])}function _t(t,e){t.$$.dirty[0]===-1&&(te.push(t),ut(),t.$$.dirty.fill(0)),t.$$.dirty[e/31|0]|=1<<e%31}function A(t,e,l,n,r,s,o,i=[-1]){const c=ne;D(t);const a=t.$$={fragment:null,ctx:[],props:s,update:k,not_equal:r,bound:pe(),on_mount:[],on_destroy:[],on_disconnect:[],before_update:[],after_update:[],context:new Map(e.context||(c?c.$$.context:[])),callbacks:pe(),dirty:i,skip_bound:!1,root:e.target||c.$$.root};o&&o(a.root);let g=!1;if(a.ctx=l?l(t,e.props||{},(p,y,...b)=>{const P=b.length?b[0]:y;return a.ctx&&r(a.ctx[p],a.ctx[p]=P)&&(!a.skip_bound&&a.bound[p]&&a.bound[p](P),g&&_t(t,p)),y}):[],a.update(),g=!0,le(a.before_update),a.fragment=n?n(a.ctx):!1,e.target){if(e.hydrate){const p=st(e.target);a.fragment&&a.fragment.l(p),p.forEach(C)}else a.fragment&&a.fragment.c();e.intro&&h(t.$$.fragment),M(t,e.target,e.anchor,e.customElement),be()}D(c)}class W{$destroy(){T(this,1),this.$destroy=k}$on(e,l){if(!Je(l))return k;const n=this.$$.callbacks[e]||(this.$$.callbacks[e]=[]);return n.push(l),()=>{const r=n.indexOf(l);r!==-1&&n.splice(r,1)}}$set(e){this.$$set&&!tt(e)&&(this.$$.skip_bound=!0,this.$$set(e),this.$$.skip_bound=!1)}}const ee=[];function Qe(t,e=k){let l;const n=new Set;function r(i){if(z(t,i)&&(t=i,l)){const c=!ee.length;for(const a of n)a[1](),ee.push(a,t);if(c){for(let a=0;a<ee.length;a+=2)ee[a][0](ee[a+1]);ee.length=0}}}function s(i){r(i(t))}function o(i,c=k){const a=[i,c];return n.add(a),n.size===1&&(l=e(r)||k),i(t),()=>{n.delete(a),n.size===0&&(l(),l=null)}}return{set:r,update:s,subscribe:o}}const ue={};function Le(t){return t==="local"?localStorage:sessionStorage}function Xe(t,e,l){const n=(l==null?void 0:l.serializer)??JSON,r=(l==null?void 0:l.storage)??"local",s=typeof window<"u"&&typeof document<"u";function o(i,c){s&&Le(r).setItem(i,n.stringify(c))}if(!ue[t]){const i=Qe(e,g=>{const p=s?Le(r).getItem(t):null;if(p&&g(n.parse(p)),s){const y=b=>{b.key===t&&g(b.newValue?n.parse(b.newValue):null)};return window.addEventListener("storage",y),()=>window.removeEventListener("storage",y)}}),{subscribe:c,set:a}=i;ue[t]={set(g){o(t,g),a(g)},update(g){const p=g(lt(i));o(t,p),a(p)},subscribe:c}}return ue[t]}Xe("storePrefersDarkScheme",!1);Xe("storeLightSwitch",void 0);const gt=t=>({}),Pe=t=>({}),ht=t=>({}),$e=t=>({}),mt=t=>({}),Ce=t=>({}),bt=t=>({}),Re=t=>({}),pt=t=>({}),Ee=t=>({}),vt=t=>({}),je=t=>({});function qe(t){let e,l,n;const r=t[18].header,s=Y(r,t,t[17],je);return{c(){e=_("header"),s&&s.c(),u(e,"id","shell-header"),u(e,"class",l="flex-none "+t[7])},m(o,i){R(o,e,i),s&&s.m(e,null),n=!0},p(o,i){s&&s.p&&(!n||i&131072)&&X(s,r,o,o[17],n?Q(r,o[17],i,vt):Z(o[17]),je),(!n||i&128&&l!==(l="flex-none "+o[7]))&&u(e,"class",l)},i(o){n||(h(s,o),n=!0)},o(o){S(s,o),n=!1},d(o){o&&C(e),s&&s.d(o)}}}function He(t){let e,l;const n=t[18].sidebarLeft,r=Y(n,t,t[17],Ee);return{c(){e=_("aside"),r&&r.c(),u(e,"id","sidebar-left"),u(e,"class",t[6])},m(s,o){R(s,e,o),r&&r.m(e,null),l=!0},p(s,o){r&&r.p&&(!l||o&131072)&&X(r,n,s,s[17],l?Q(n,s[17],o,pt):Z(s[17]),Ee),(!l||o&64)&&u(e,"class",s[6])},i(s){l||(h(r,s),l=!0)},o(s){S(r,s),l=!1},d(s){s&&C(e),r&&r.d(s)}}}function Fe(t){let e,l,n;const r=t[18].pageHeader,s=Y(r,t,t[17],Re),o=s||kt();return{c(){e=_("header"),o&&o.c(),u(e,"id","page-header"),u(e,"class",l="flex-none "+t[4])},m(i,c){R(i,e,c),o&&o.m(e,null),n=!0},p(i,c){s&&s.p&&(!n||c&131072)&&X(s,r,i,i[17],n?Q(r,i[17],c,bt):Z(i[17]),Re),(!n||c&16&&l!==(l="flex-none "+i[4]))&&u(e,"class",l)},i(i){n||(h(o,i),n=!0)},o(i){S(o,i),n=!1},d(i){i&&C(e),o&&o.d(i)}}}function kt(t){let e;return{c(){e=I("(slot:header)")},m(l,n){R(l,e,n)},d(l){l&&C(e)}}}function Ne(t){let e,l,n;const r=t[18].pageFooter,s=Y(r,t,t[17],Ce),o=s||yt();return{c(){e=_("footer"),o&&o.c(),u(e,"id","page-footer"),u(e,"class",l="flex-none "+t[2])},m(i,c){R(i,e,c),o&&o.m(e,null),n=!0},p(i,c){s&&s.p&&(!n||c&131072)&&X(s,r,i,i[17],n?Q(r,i[17],c,mt):Z(i[17]),Ce),(!n||c&4&&l!==(l="flex-none "+i[2]))&&u(e,"class",l)},i(i){n||(h(o,i),n=!0)},o(i){S(o,i),n=!1},d(i){i&&C(e),o&&o.d(i)}}}function yt(t){let e;return{c(){e=I("(slot:footer)")},m(l,n){R(l,e,n)},d(l){l&&C(e)}}}function Oe(t){let e,l;const n=t[18].sidebarRight,r=Y(n,t,t[17],$e);return{c(){e=_("aside"),r&&r.c(),u(e,"id","sidebar-right"),u(e,"class",t[5])},m(s,o){R(s,e,o),r&&r.m(e,null),l=!0},p(s,o){r&&r.p&&(!l||o&131072)&&X(r,n,s,s[17],l?Q(n,s[17],o,ht):Z(s[17]),$e),(!l||o&32)&&u(e,"class",s[5])},i(s){l||(h(r,s),l=!0)},o(s){S(r,s),l=!1},d(s){s&&C(e),r&&r.d(s)}}}function ze(t){let e,l,n;const r=t[18].footer,s=Y(r,t,t[17],Pe);return{c(){e=_("footer"),s&&s.c(),u(e,"id","shell-footer"),u(e,"class",l="flex-none "+t[1])},m(o,i){R(o,e,i),s&&s.m(e,null),n=!0},p(o,i){s&&s.p&&(!n||i&131072)&&X(s,r,o,o[17],n?Q(r,o[17],i,gt):Z(o[17]),Pe),(!n||i&2&&l!==(l="flex-none "+o[1]))&&u(e,"class",l)},i(o){n||(h(s,o),n=!0)},o(o){S(s,o),n=!1},d(o){o&&C(e),s&&s.d(o)}}}function wt(t){let e,l,n,r,s,o,i,c,a,g,p,y,b,P,H,w=t[9].header&&qe(t),E=t[9].sidebarLeft&&He(t),L=t[9].pageHeader&&Fe(t);const N=t[18].default,$=Y(N,t,t[17],null);let m=t[9].pageFooter&&Ne(t),j=t[9].sidebarRight&&Oe(t),f=t[9].footer&&ze(t);return{c(){e=_("div"),w&&w.c(),l=q(),n=_("div"),E&&E.c(),r=q(),s=_("div"),L&&L.c(),o=q(),i=_("main"),$&&$.c(),a=q(),m&&m.c(),p=q(),j&&j.c(),y=q(),f&&f.c(),u(i,"id","page-content"),u(i,"class",c="flex-auto "+t[3]),u(s,"id","page"),u(s,"class",g=t[0]+" "+Be),u(n,"class","flex-auto "+Lt),u(e,"id","appShell"),u(e,"class",t[8]),u(e,"data-testid","app-shell")},m(v,F){R(v,e,F),w&&w.m(e,null),d(e,l),d(e,n),E&&E.m(n,null),d(n,r),d(n,s),L&&L.m(s,null),d(s,o),d(s,i),$&&$.m(i,null),d(s,a),m&&m.m(s,null),d(n,p),j&&j.m(n,null),d(e,y),f&&f.m(e,null),b=!0,P||(H=de(s,"scroll",t[19]),P=!0)},p(v,[F]){v[9].header?w?(w.p(v,F),F&512&&h(w,1)):(w=qe(v),w.c(),h(w,1),w.m(e,l)):w&&(J(),S(w,1,1,()=>{w=null}),V()),v[9].sidebarLeft?E?(E.p(v,F),F&512&&h(E,1)):(E=He(v),E.c(),h(E,1),E.m(n,r)):E&&(J(),S(E,1,1,()=>{E=null}),V()),v[9].pageHeader?L?(L.p(v,F),F&512&&h(L,1)):(L=Fe(v),L.c(),h(L,1),L.m(s,o)):L&&(J(),S(L,1,1,()=>{L=null}),V()),$&&$.p&&(!b||F&131072)&&X($,N,v,v[17],b?Q(N,v[17],F,null):Z(v[17]),null),(!b||F&8&&c!==(c="flex-auto "+v[3]))&&u(i,"class",c),v[9].pageFooter?m?(m.p(v,F),F&512&&h(m,1)):(m=Ne(v),m.c(),h(m,1),m.m(s,null)):m&&(J(),S(m,1,1,()=>{m=null}),V()),(!b||F&1&&g!==(g=v[0]+" "+Be))&&u(s,"class",g),v[9].sidebarRight?j?(j.p(v,F),F&512&&h(j,1)):(j=Oe(v),j.c(),h(j,1),j.m(n,null)):j&&(J(),S(j,1,1,()=>{j=null}),V()),v[9].footer?f?(f.p(v,F),F&512&&h(f,1)):(f=ze(v),f.c(),h(f,1),f.m(e,null)):f&&(J(),S(f,1,1,()=>{f=null}),V()),(!b||F&256)&&u(e,"class",v[8])},i(v){b||(h(w),h(E),h(L),h($,v),h(m),h(j),h(f),b=!0)},o(v){S(w),S(E),S(L),S($,v),S(m),S(j),S(f),b=!1},d(v){v&&C(e),w&&w.d(),E&&E.d(),L&&L.d(),$&&$.d(v),m&&m.d(),j&&j.d(),f&&f.d(),P=!1,H()}}}const St="w-full h-full flex flex-col overflow-hidden",Lt="w-full h-full flex overflow-hidden",Be="flex-1 overflow-x-hidden overflow-y-auto flex flex-col",Pt="flex-none overflow-x-hidden overflow-y-auto",$t="flex-none overflow-x-hidden overflow-y-auto";function Ct(t,e,l){let n,r,s,o,i,c,a,g,{$$slots:p={},$$scope:y}=e;const b=nt(p);let{regionPage:P=""}=e,{slotHeader:H="z-10"}=e,{slotSidebarLeft:w="w-auto"}=e,{slotSidebarRight:E="w-auto"}=e,{slotPageHeader:L=""}=e,{slotPageContent:N=""}=e,{slotPageFooter:$=""}=e,{slotFooter:m=""}=e;function j(f){at.call(this,t,f)}return t.$$set=f=>{l(20,e=fe(fe({},e),ve(f))),"regionPage"in f&&l(0,P=f.regionPage),"slotHeader"in f&&l(10,H=f.slotHeader),"slotSidebarLeft"in f&&l(11,w=f.slotSidebarLeft),"slotSidebarRight"in f&&l(12,E=f.slotSidebarRight),"slotPageHeader"in f&&l(13,L=f.slotPageHeader),"slotPageContent"in f&&l(14,N=f.slotPageContent),"slotPageFooter"in f&&l(15,$=f.slotPageFooter),"slotFooter"in f&&l(16,m=f.slotFooter),"$$scope"in f&&l(17,y=f.$$scope)},t.$$.update=()=>{l(8,n=`${St} ${e.class??""}`),t.$$.dirty&1024&&l(7,r=`${H}`),t.$$.dirty&2048&&l(6,s=`${Pt} ${w}`),t.$$.dirty&4096&&l(5,o=`${$t} ${E}`),t.$$.dirty&8192&&l(4,i=`${L}`),t.$$.dirty&16384&&l(3,c=`${N}`),t.$$.dirty&32768&&l(2,a=`${$}`),t.$$.dirty&65536&&l(1,g=`${m}`)},e=ve(e),[P,g,a,c,i,o,s,r,n,b,H,w,E,L,N,$,m,y,p,j]}class Rt extends W{constructor(e){super(),A(this,e,Ct,wt,z,{regionPage:0,slotHeader:10,slotSidebarLeft:11,slotSidebarRight:12,slotPageHeader:13,slotPageContent:14,slotPageFooter:15,slotFooter:16})}}function Me(t,e,l){const n=t.slice();return n[3]=e[l],n}function Te(t){let e,l,n=t[3].round+"",r,s,o,i=t[3].nos+"",c,a,g,p=t[3].eles+"",y,b,P,H=t[3].value+"",w,E;return{c(){e=_("tr"),l=_("td"),r=I(n),s=q(),o=_("td"),c=I(i),a=q(),g=_("td"),y=I(p),b=q(),P=_("td"),w=I(H),E=q(),u(l,"class","svelte-1prmq1x"),u(o,"class","svelte-1prmq1x"),u(g,"class","svelte-1prmq1x"),u(P,"class","svelte-1prmq1x"),u(e,"class","svelte-1prmq1x"),ke(e,"lose",t[3].eles=="Win"),ke(e,"win",t[3].nos=="Win")},m(L,N){R(L,e,N),d(e,l),d(l,r),d(e,s),d(e,o),d(o,c),d(e,a),d(e,g),d(g,y),d(e,b),d(e,P),d(P,w),d(e,E)},p:k,d(L){L&&C(e)}}}function Et(t){let e,l,n,r,s,o,i,c,a,g,p,y,b,P,H,w,E,L,N=t[0],$=[];for(let m=0;m<N.length;m+=1)$[m]=Te(Me(t,N,m));return{c(){e=_("div"),l=_("h2"),l.textContent="Score history",n=q(),r=_("hr"),s=q(),o=_("div"),i=_("table"),c=_("thead"),c.innerHTML=`<tr class="svelte-1prmq1x"><th class="svelte-1prmq1x">Round</th> 
                    <th class="svelte-1prmq1x">Nós</th> 
                    <th class="svelte-1prmq1x">Eles</th> 
                    <th class="svelte-1prmq1x">Value</th></tr>`,a=q(),g=_("tbody");for(let m=0;m<$.length;m+=1)$[m].c();p=q(),y=_("tfoot"),b=_("tr"),P=_("td"),P.textContent="Score",H=q(),w=_("td"),w.textContent=`${t[1]}`,E=q(),L=_("td"),L.textContent=`${t[2]}`,u(l,"class","p-2"),u(P,"class","font-bold svelte-1prmq1x"),u(w,"class","font-bold  svelte-1prmq1x"),u(L,"class","font-bold svelte-1prmq1x"),u(b,"class","svelte-1prmq1x"),u(i,"class","table svelte-1prmq1x"),u(o,"class","table-container"),u(e,"class","card pt-2 text-center")},m(m,j){R(m,e,j),d(e,l),d(e,n),d(e,r),d(e,s),d(e,o),d(o,i),d(i,c),d(i,a),d(i,g);for(let f=0;f<$.length;f+=1)$[f].m(g,null);d(i,p),d(i,y),d(y,b),d(b,P),d(b,H),d(b,w),d(b,E),d(b,L)},p(m,[j]){if(j&1){N=m[0];let f;for(f=0;f<N.length;f+=1){const v=Me(m,N,f);$[f]?$[f].p(v,j):($[f]=Te(v),$[f].c(),$[f].m(g,null))}for(;f<$.length;f+=1)$[f].d(1);$.length=N.length}},i:k,o:k,d(m){m&&C(e),he($,m)}}}function jt(t){let e=[{round:1,nos:"Win",eles:"",value:1},{round:2,nos:"",eles:"Win",value:6},{round:3,nos:"Win",eles:"",value:3}],l=e.reduce((r,s)=>r+(s.nos=="Win"?s.value:0),0),n=e.reduce((r,s)=>r+(s.eles=="Win"?s.value:0),0);return[e,l,n]}class qt extends W{constructor(e){super(),A(this,e,jt,Et,z,{})}}function Ht(t){let e;return{c(){e=_("div"),e.innerHTML=`<header class="card-header text-center pb-4"><h3>Game log</h3></header> 
    <hr/> 
	<div class="p-4">(body)</div>`,u(e,"class","card")},m(l,n){R(l,e,n)},p:k,i:k,o:k,d(l){l&&C(e)}}}class Ft extends W{constructor(e){super(),A(this,e,null,Ht,z,{})}}function Nt(t){let e,l,n,r,s;return l=new qt({}),r=new Ft({}),{c(){e=_("div"),G(l.$$.fragment),n=q(),G(r.$$.fragment),u(e,"class","flex flex-col justify-between gambiarra svelte-1hc6umh")},m(o,i){R(o,e,i),M(l,e,null),d(e,n),M(r,e,null),s=!0},p:k,i(o){s||(h(l.$$.fragment,o),h(r.$$.fragment,o),s=!0)},o(o){S(l.$$.fragment,o),S(r.$$.fragment,o),s=!1},d(o){o&&C(e),T(l),T(r)}}}class Ot extends W{constructor(e){super(),A(this,e,null,Nt,z,{})}}function zt(t){let e,l,n,r,s,o;return{c(){e=_("div"),l=_("button"),l.textContent="Iniciar jogo",n=q(),r=_("button"),r.textContent="Delete room",u(l,"class","btn bg-primary-500"),u(r,"class","btn bg-primary-500"),u(e,"class","flex flex-col gambiarra gap-2 svelte-z1fb2r")},m(i,c){R(i,e,c),d(e,l),d(e,n),d(e,r),s||(o=[de(l,"click",t[0]),de(r,"click",Mt)],s=!0)},p:k,i:k,o:k,d(i){i&&C(e),s=!1,le(o)}}}function Bt(t){var e="; "+document.cookie,l=e.split("; "+t+"=");if(l.length==2)return l.pop().split(";").shift()}async function Mt(){await fetch(window.location.href,{method:"DELETE"}).then(t=>t.status==204?window.location.replace(`${window.location.origin}/lobby`):console.log("Não foi possivel deletar a sala, resposta não do servidor foi "+t.status))}function Tt(t){const e=it("ws");function l(){const n=Bt("uuid"),r=location.pathname.split("/")[2],s={action:"StartGame",user:n,room:r};e.send(JSON.stringify(s))}return[l]}class At extends W{constructor(e){super(),A(this,e,Tt,zt,z,{})}}class Wt{constructor(){U(this,"map",new Map);U(this,"initialized");U(this,"set_utils_cards",e=>{const l=fetch(this.url_background_svg()).then(r=>r.blob()).then(r=>{this.map.set("bg",URL.createObjectURL(r))}),n=fetch(this.url_card_loading()).then(r=>r.blob()).then(r=>{this.map.set("loading",URL.createObjectURL(r))});e.push(l),e.push(n)});U(this,"url_card_svg",(e,l)=>`https://cdn.jsdelivr.net/gh/kaiofake/cardsSvg/${e}${l}.svg`);U(this,"url_background_svg",()=>"https://cdn.jsdelivr.net/gh/kaiofake/cardsSvg/backgroundcard.svg");U(this,"url_card_loading",()=>"https://cdn.jsdelivr.net/gh/kaiofake/cardsSvg/loadingcard.svg");U(this,"get_card_svg",e=>this.map.get(`${e.number}${e.suit}`));U(this,"get_utils_svg",e=>this.map.get(e));this.initialized=new Promise(e=>{const l=[];this.set_utils_cards(l);for(let n=0;n<4;n++)for(let r=1;r<11;r++){const s=fetch(this.url_card_svg(r,n)).then(o=>o.blob()).then(o=>{this.map.set(`${r}${n}`,URL.createObjectURL(o))});l.push(s)}Promise.all(l).then(()=>e())})}}const ae=Qe(new Wt);function Ae(t,e,l){const n=t.slice();return n[3]=e[l],n}function We(t){let e;return{c(){e=_("li"),e.textContent="Your cards will be here"},m(l,n){R(l,e,n)},p:k,d(l){l&&C(e)}}}function Ue(t){let e,l,n;return{c(){e=_("div"),l=_("img"),O(l.src,n=t[0]?t[2].get_card_svg(t[3]):t[2].get_utils_svg("loading"))||u(l,"src",n),u(l,"alt","10"),u(e,"class","truco_card svelte-uy0z68")},m(r,s){R(r,e,s),d(e,l)},p(r,s){s&5&&!O(l.src,n=r[0]?r[2].get_card_svg(r[3]):r[2].get_utils_svg("loading"))&&u(l,"src",n)},d(r){r&&C(e)}}}function Ie(t){let e;return{c(){e=_("button"),e.textContent="Ask for truco",u(e,"class","btn variant-filled-primary")},m(l,n){R(l,e,n)},d(l){l&&C(e)}}}function Ut(t){let e,l,n=t[0],r=[];for(let i=0;i<n.length;i+=1)r[i]=Ue(Ae(t,n,i));let s=null;n.length||(s=We());let o=t[1]&&Ie();return{c(){e=_("div");for(let i=0;i<r.length;i+=1)r[i].c();s&&s.c(),l=q(),o&&o.c(),u(e,"class","flex justify-center")},m(i,c){R(i,e,c);for(let a=0;a<r.length;a+=1)r[a].m(e,null);s&&s.m(e,null),d(e,l),o&&o.m(e,null)},p(i,[c]){if(c&5){n=i[0];let a;for(a=0;a<n.length;a+=1){const g=Ae(i,n,a);r[a]?r[a].p(g,c):(r[a]=Ue(g),r[a].c(),r[a].m(e,l))}for(;a<r.length;a+=1)r[a].d(1);r.length=n.length,!n.length&&s?s.p(i,c):n.length?s&&(s.d(1),s=null):(s=We(),s.c(),s.m(e,l))}i[1]?o||(o=Ie(),o.c(),o.m(e,null)):o&&(o.d(1),o=null)},i:k,o:k,d(i){i&&C(e),he(r,i),s&&s.d(),o&&o.d()}}}function It(t,e,l){let n;ie(t,ae,o=>l(2,n=o));let{player_hand:r=[]}=e,{is_allowed_to_ask_truco:s=!1}=e;return t.$$set=o=>{"player_hand"in o&&l(0,r=o.player_hand),"is_allowed_to_ask_truco"in o&&l(1,s=o.is_allowed_to_ask_truco)},[r,s,n]}class Dt extends W{constructor(e){super(),A(this,e,It,Ut,z,{player_hand:0,is_allowed_to_ask_truco:1})}}function Gt(t){let e;return{c(){e=_("h2"),e.textContent="The game hasn't started yet"},m(l,n){R(l,e,n)},p:k,d(l){l&&C(e)}}}function Jt(t){let e,l,n,r,s,o,i;return{c(){e=_("div"),l=_("img"),r=q(),s=_("div"),o=_("img"),O(l.src,n=t[1].get_utils_svg("bg"))||u(l,"src",n),u(l,"alt","10"),u(e,"class","truco_card cardback svelte-x0jw19"),O(o.src,i=t[0]?t[1].get_card_svg(t[0]):t[1].get_utils_svg("loading"))||u(o,"src",i),u(o,"alt","10"),u(s,"class","truco_card manilha svelte-x0jw19")},m(c,a){R(c,e,a),d(e,l),R(c,r,a),R(c,s,a),d(s,o)},p(c,a){a&2&&!O(l.src,n=c[1].get_utils_svg("bg"))&&u(l,"src",n),a&3&&!O(o.src,i=c[0]?c[1].get_card_svg(c[0]):c[1].get_utils_svg("loading"))&&u(o,"src",i)},d(c){c&&C(e),c&&C(r),c&&C(s)}}}function Vt(t){let e;function l(s,o){return s[0]?Jt:Gt}let n=l(t),r=n(t);return{c(){e=_("div"),r.c(),u(e,"class","table_game flex items-center justify-center svelte-x0jw19")},m(s,o){R(s,e,o),r.m(e,null)},p(s,[o]){n===(n=l(s))&&r?r.p(s,o):(r.d(1),r=n(s),r&&(r.c(),r.m(e,null)))},i:k,o:k,d(s){s&&C(e),r.d()}}}function Kt(t,e,l){let n;ie(t,ae,s=>l(1,n=s));let{table_manilha:r=null}=e;return t.$$set=s=>{"table_manilha"in s&&l(0,r=s.table_manilha)},[r,n]}class Yt extends W{constructor(e){super(),A(this,e,Kt,Vt,z,{table_manilha:0})}}function Qt(t){let e,l,n,r,s,o,i,c,a,g,p,y,b;return{c(){e=_("div"),l=_("div"),n=_("span"),r=_("img"),o=q(),i=_("span"),c=_("img"),g=q(),p=_("span"),y=_("img"),O(r.src,s=t[0].get_utils_svg("bg"))||u(r,"src",s),u(r,"alt","10"),u(n,"class","truco_card cardback svelte-4ardud"),O(c.src,a=t[0].get_utils_svg("bg"))||u(c,"src",a),u(c,"alt","10"),u(i,"class","truco_card cardback svelte-4ardud"),O(y.src,b=t[0].get_utils_svg("bg"))||u(y,"src",b),u(y,"alt","10"),u(p,"class","truco_card cardback svelte-4ardud"),u(l,"class","flex justify-center gap-1 relative cards_container svelte-4ardud"),u(e,"class","absolute "+t[1]+" svelte-4ardud"),rt(e,"--rot_degree",t[2])},m(P,H){R(P,e,H),d(e,l),d(l,n),d(n,r),d(l,o),d(l,i),d(i,c),d(l,g),d(l,p),d(p,y)},p(P,[H]){H&1&&!O(r.src,s=P[0].get_utils_svg("bg"))&&u(r,"src",s),H&1&&!O(c.src,a=P[0].get_utils_svg("bg"))&&u(c,"src",a),H&1&&!O(y.src,b=P[0].get_utils_svg("bg"))&&u(y,"src",b)},i:k,o:k,d(P){P&&C(e)}}}function Xt(t,e,l){let n;ie(t,ae,a=>l(0,n=a));let{initials:r="P1"}=e,{name:s="(empty)"}=e,{position:o=0}=e,i="player_position"+o,c=o*90+"deg";return t.$$set=a=>{"initials"in a&&l(3,r=a.initials),"name"in a&&l(4,s=a.name),"position"in a&&l(5,o=a.position)},[n,i,c,r,s,o]}class Zt extends W{constructor(e){super(),A(this,e,Xt,Qt,z,{initials:3,name:4,position:5})}}function xt(t,e,l){const n=t.slice();return n[1]=e[l],n[3]=l,n}function De(t){let e,l,n={length:4},r=[];for(let s=0;s<n.length;s+=1)r[s]=el(xt(t,n,s));return{c(){for(let s=0;s<r.length;s+=1)r[s].c();e=Ye()},m(s,o){for(let i=0;i<r.length;i+=1)r[i].m(s,o);R(s,e,o),l=!0},i(s){if(!l){for(let o=0;o<n.length;o+=1)h(r[o]);l=!0}},o(s){r=r.filter(Boolean);for(let o=0;o<r.length;o+=1)S(r[o]);l=!1},d(s){he(r,s),s&&C(e)}}}function el(t){let e,l;return e=new Zt({props:{initials:"P"+t[3],position:t[3]}}),{c(){G(e.$$.fragment)},m(n,r){M(e,n,r),l=!0},p:k,i(n){l||(h(e.$$.fragment,n),l=!0)},o(n){S(e.$$.fragment,n),l=!1},d(n){T(e,n)}}}function tl(t){var o;let e,l,n,r,s=t[0]&&De(t);return n=new Yt({props:{table_manilha:(o=t[0])==null?void 0:o.round_data.manilha}}),{c(){e=_("div"),s&&s.c(),l=q(),G(n.$$.fragment),u(e,"class","grid place-items-center relative svelte-edndqx"),u(e,"id","game_body")},m(i,c){R(i,e,c),s&&s.m(e,null),d(e,l),M(n,e,null),r=!0},p(i,[c]){var g;i[0]?s?c&1&&h(s,1):(s=De(i),s.c(),h(s,1),s.m(e,l)):s&&(J(),S(s,1,1,()=>{s=null}),V());const a={};c&1&&(a.table_manilha=(g=i[0])==null?void 0:g.round_data.manilha),n.$set(a)},i(i){r||(h(s),h(n.$$.fragment,i),r=!0)},o(i){S(s),S(n.$$.fragment,i),r=!1},d(i){i&&C(e),s&&s.d(),T(n)}}}function ll(t,e,l){let{setup:n=null}=e;return t.$$set=r=>{"setup"in r&&l(0,n=r.setup)},[n]}class nl extends W{constructor(e){super(),A(this,e,ll,tl,z,{setup:0})}}let B=null;function sl(){rl();const{location:t}=window,l=`${t.protocol.startsWith("https")?"wss":"ws"}://${t.host}${t.pathname}/ws`;return B=new WebSocket(l),B.onopen=()=>{console.log("Eu entrei")},B.onclose=()=>{B=null},B}function rl(){B&&(B.close(),B=null)}function ol(t){return{c:k,m:k,p:k,i:k,o:k,d:k}}function il(t){var s,o,i,c;let e,l,n,r;return e=new nl({props:{setup:t[0]}}),n=new Dt({props:{player_hand:(o=(s=t[0])==null?void 0:s.user_data)==null?void 0:o.hand,is_allowed_to_ask_truco:(c=(i=t[0])==null?void 0:i.user_data)==null?void 0:c.is_allowed_to_truco}}),{c(){G(e.$$.fragment),l=q(),G(n.$$.fragment)},m(a,g){M(e,a,g),R(a,l,g),M(n,a,g),r=!0},p(a,g){var b,P,H,w;const p={};g&1&&(p.setup=a[0]),e.$set(p);const y={};g&1&&(y.player_hand=(P=(b=a[0])==null?void 0:b.user_data)==null?void 0:P.hand),g&1&&(y.is_allowed_to_ask_truco=(w=(H=a[0])==null?void 0:H.user_data)==null?void 0:w.is_allowed_to_truco),n.$set(y)},i(a){r||(h(e.$$.fragment,a),h(n.$$.fragment,a),r=!0)},o(a){S(e.$$.fragment,a),S(n.$$.fragment,a),r=!1},d(a){T(e,a),a&&C(l),T(n,a)}}}function al(t){let e;return{c(){e=I("...fetching cards")},m(l,n){R(l,e,n)},p:k,i:k,o:k,d(l){l&&C(e)}}}function cl(t){let e,l,n,r={ctx:t,current:null,token:null,hasCatch:!1,pending:al,then:il,catch:ol,value:3,blocks:[,,,]};return Se(l=t[1].initialized,r),{c(){e=Ye(),r.block.c()},m(s,o){R(s,e,o),r.block.m(s,r.anchor=o),r.mount=()=>e.parentNode,r.anchor=e,n=!0},p(s,o){t=s,r.ctx=t,o&2&&l!==(l=t[1].initialized)&&Se(l,r)||dt(r,t,o)},i(s){n||(h(r.block),n=!0)},o(s){for(let o=0;o<3;o+=1){const i=r.blocks[o];S(i)}n=!1},d(s){s&&C(e),r.block.d(s),r.token=null,r=null}}}function ul(t){let e,l;return e=new At({}),{c(){G(e.$$.fragment)},m(n,r){M(e,n,r),l=!0},i(n){l||(h(e.$$.fragment,n),l=!0)},o(n){S(e.$$.fragment,n),l=!1},d(n){T(e,n)}}}function fl(t){let e,l;return e=new Ot({}),{c(){G(e.$$.fragment)},m(n,r){M(e,n,r),l=!0},i(n){l||(h(e.$$.fragment,n),l=!0)},o(n){S(e.$$.fragment,n),l=!1},d(n){T(e,n)}}}function dl(t){let e,l;return e=new Rt({props:{$$slots:{sidebarRight:[fl],sidebarLeft:[ul],default:[cl]},$$scope:{ctx:t}}}),{c(){G(e.$$.fragment)},m(n,r){M(e,n,r),l=!0},p(n,[r]){const s={};r&19&&(s.$$scope={dirty:r,ctx:n}),e.$set(s)},i(n){l||(h(e.$$.fragment,n),l=!0)},o(n){S(e.$$.fragment,n),l=!1},d(n){T(e,n)}}}function _l(t,e,l){let n;ie(t,ae,s=>l(1,n=s));let r=null;return sl(),B.onmessage=s=>{let o=JSON.parse(s.data);switch(o.msg_type){case"Notification":console.log(o);break;case"Redirect":window.location.replace(`${window.location.origin}/${o.redirect}`);break;case"GameNotification":switch(o.action){case"RoundStartState":l(0,r=o);break;default:console.log(`Error:${[...o]}`)}default:console.log(`Error:${[...o]}`)}},ot("ws",B),[r,n]}class gl extends W{constructor(e){super(),A(this,e,_l,dl,z,{})}}new gl({target:document.getElementById("app")});
