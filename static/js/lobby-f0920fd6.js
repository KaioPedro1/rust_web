(function(){const e=document.createElement("link").relList;if(e&&e.supports&&e.supports("modulepreload"))return;for(const o of document.querySelectorAll('link[rel="modulepreload"]'))n(o);new MutationObserver(o=>{for(const s of o)if(s.type==="childList")for(const r of s.addedNodes)r.tagName==="LINK"&&r.rel==="modulepreload"&&n(r)}).observe(document,{childList:!0,subtree:!0});function l(o){const s={};return o.integrity&&(s.integrity=o.integrity),o.referrerpolicy&&(s.referrerPolicy=o.referrerpolicy),o.crossorigin==="use-credentials"?s.credentials="include":o.crossorigin==="anonymous"?s.credentials="omit":s.credentials="same-origin",s}function n(o){if(o.ep)return;o.ep=!0;const s=l(o);fetch(o.href,s)}})();function R(){}function x(t,e){for(const l in e)t[l]=e[l];return t}function $e(t){return t()}function ye(){return Object.create(null)}function $(t){t.forEach($e)}function et(t){return typeof t=="function"}function J(t,e){return t!=t?e==e:t!==e||t&&typeof t=="object"||typeof t=="function"}function ut(t){return Object.keys(t).length===0}function tt(t,...e){if(t==null)return R;const l=t.subscribe(...e);return l.unsubscribe?()=>l.unsubscribe():l}function dt(t){let e;return tt(t,l=>e=l)(),e}function ve(t,e,l){t.$$.on_destroy.push(tt(e,l))}function B(t,e,l,n){if(t){const o=lt(t,e,l,n);return t[0](o)}}function lt(t,e,l,n){return t[1]&&n?x(l.ctx.slice(),t[1](n(e))):l.ctx}function O(t,e,l,n){if(t[2]&&n){const o=t[2](n(l));if(e.dirty===void 0)return o;if(typeof o=="object"){const s=[],r=Math.max(e.dirty.length,o.length);for(let i=0;i<r;i+=1)s[i]=e.dirty[i]|o[i];return s}return e.dirty|o}return e.dirty}function q(t,e,l,n,o,s){if(o){const r=lt(e,l,n,s);t.p(r,o)}}function I(t){if(t.ctx.length>32){const e=[],l=t.ctx.length/32;for(let n=0;n<l;n++)e[n]=-1;return e}return-1}function re(t){const e={};for(const l in t)l[0]!=="$"&&(e[l]=t[l]);return e}function nt(t){const e={};for(const l in t)e[l]=!0;return e}function Le(t,e,l){return t.set(l),e}function w(t,e){t.appendChild(e)}function H(t,e,l){t.insertBefore(e,l||null)}function P(t){t.parentNode&&t.parentNode.removeChild(t)}function st(t,e){for(let l=0;l<t.length;l+=1)t[l]&&t[l].d(e)}function L(t){return document.createElement(t)}function ae(t){return document.createElementNS("http://www.w3.org/2000/svg",t)}function M(t){return document.createTextNode(t)}function z(){return M(" ")}function ht(){return M("")}function se(t,e,l,n){return t.addEventListener(e,l,n),()=>t.removeEventListener(e,l,n)}function d(t,e,l){l==null?t.removeAttribute(e):t.getAttribute(e)!==l&&t.setAttribute(e,l)}function mt(t){return Array.from(t.childNodes)}function oe(t,e){e=""+e,t.wholeText!==e&&(t.data=e)}function _t(t,e,{bubbles:l=!1,cancelable:n=!1}={}){const o=document.createEvent("CustomEvent");return o.initCustomEvent(t,l,n,e),o}class gt{constructor(e=!1){this.is_svg=!1,this.is_svg=e,this.e=this.n=null}c(e){this.h(e)}m(e,l,n=null){this.e||(this.is_svg?this.e=ae(l.nodeName):this.e=L(l.nodeName),this.t=l,this.c(e)),this.i(n)}h(e){this.e.innerHTML=e,this.n=Array.from(this.e.childNodes)}i(e){for(let l=0;l<this.n.length;l+=1)H(this.t,this.n[l],e)}p(e){this.d(),this.h(e),this.i(this.a)}d(){this.n.forEach(P)}}let fe;function ie(t){fe=t}function rt(){if(!fe)throw new Error("Function called outside component initialization");return fe}function bt(t){rt().$$.on_mount.push(t)}function pt(){const t=rt();return(e,l,{cancelable:n=!1}={})=>{const o=t.$$.callbacks[e];if(o){const s=_t(e,l,{cancelable:n});return o.slice().forEach(r=>{r.call(t,s)}),!s.defaultPrevented}return!0}}function Se(t,e){const l=t.$$.callbacks[e.type];l&&l.slice().forEach(n=>n.call(this,e))}const ne=[],Ee=[],ue=[],Pe=[],kt=Promise.resolve();let _e=!1;function wt(){_e||(_e=!0,kt.then(ot))}function ge(t){ue.push(t)}const he=new Set;let te=0;function ot(){if(te!==0)return;const t=fe;do{try{for(;te<ne.length;){const e=ne[te];te++,ie(e),yt(e.$$)}}catch(e){throw ne.length=0,te=0,e}for(ie(null),ne.length=0,te=0;Ee.length;)Ee.pop()();for(let e=0;e<ue.length;e+=1){const l=ue[e];he.has(l)||(he.add(l),l())}ue.length=0}while(ne.length);for(;Pe.length;)Pe.pop()();_e=!1,he.clear(),ie(t)}function yt(t){if(t.fragment!==null){t.update(),$(t.before_update);const e=t.dirty;t.dirty=[-1],t.fragment&&t.fragment.p(t.ctx,e),t.after_update.forEach(ge)}}const de=new Set;let Z;function j(){Z={r:0,c:[],p:Z}}function U(){Z.r||$(Z.c),Z=Z.p}function k(t,e){t&&t.i&&(de.delete(t),t.i(e))}function v(t,e,l,n){if(t&&t.o){if(de.has(t))return;de.add(t),Z.c.push(()=>{de.delete(t),n&&(l&&t.d(1),n())}),t.o(e)}else n&&n()}const it=typeof window<"u"?window:typeof globalThis<"u"?globalThis:global;function Q(t){t&&t.c()}function K(t,e,l,n){const{fragment:o,after_update:s}=t.$$;o&&o.m(e,l),n||ge(()=>{const r=t.$$.on_mount.map($e).filter(et);t.$$.on_destroy?t.$$.on_destroy.push(...r):$(r),t.$$.on_mount=[]}),s.forEach(ge)}function V(t,e){const l=t.$$;l.fragment!==null&&($(l.on_destroy),l.fragment&&l.fragment.d(e),l.on_destroy=l.fragment=null,l.ctx=[])}function vt(t,e){t.$$.dirty[0]===-1&&(ne.push(t),wt(),t.$$.dirty.fill(0)),t.$$.dirty[e/31|0]|=1<<e%31}function X(t,e,l,n,o,s,r,i=[-1]){const a=fe;ie(t);const f=t.$$={fragment:null,ctx:[],props:s,update:R,not_equal:o,bound:ye(),on_mount:[],on_destroy:[],on_disconnect:[],before_update:[],after_update:[],context:new Map(e.context||(a?a.$$.context:[])),callbacks:ye(),dirty:i,skip_bound:!1,root:e.target||a.$$.root};r&&r(f.root);let h=!1;if(f.ctx=l?l(t,e.props||{},(c,m,..._)=>{const b=_.length?_[0]:m;return f.ctx&&o(f.ctx[c],f.ctx[c]=b)&&(!f.skip_bound&&f.bound[c]&&f.bound[c](b),h&&vt(t,c)),m}):[],f.update(),h=!0,$(f.before_update),f.fragment=n?n(f.ctx):!1,e.target){if(e.hydrate){const c=mt(e.target);f.fragment&&f.fragment.l(c),c.forEach(P)}else f.fragment&&f.fragment.c();e.intro&&k(t.$$.fragment),K(t,e.target,e.anchor,e.customElement),ot()}ie(a)}class Y{$destroy(){V(this,1),this.$destroy=R}$on(e,l){if(!et(l))return R;const n=this.$$.callbacks[e]||(this.$$.callbacks[e]=[]);return n.push(l),()=>{const o=n.indexOf(l);o!==-1&&n.splice(o,1)}}$set(e){this.$$set&&!ut(e)&&(this.$$.skip_bound=!0,this.$$set(e),this.$$.skip_bound=!1)}}const le=[];function Lt(t,e=R){let l;const n=new Set;function o(i){if(J(t,i)&&(t=i,l)){const a=!le.length;for(const f of n)f[1](),le.push(f,t);if(a){for(let f=0;f<le.length;f+=2)le[f][0](le[f+1]);le.length=0}}}function s(i){o(i(t))}function r(i,a=R){const f=[i,a];return n.add(f),n.size===1&&(l=e(o)||R),i(t),()=>{n.delete(f),n.size===0&&(l(),l=null)}}return{set:o,update:s,subscribe:r}}const me={};function Ce(t){return t==="local"?localStorage:sessionStorage}function at(t,e,l){const n=(l==null?void 0:l.serializer)??JSON,o=(l==null?void 0:l.storage)??"local",s=typeof window<"u"&&typeof document<"u";function r(i,a){s&&Ce(o).setItem(i,n.stringify(a))}if(!me[t]){const i=Lt(e,h=>{const c=s?Ce(o).getItem(t):null;if(c&&h(n.parse(c)),s){const m=_=>{_.key===t&&h(_.newValue?n.parse(_.newValue):null)};return window.addEventListener("storage",m),()=>window.removeEventListener("storage",m)}}),{subscribe:a,set:f}=i;me[t]={set(h){r(t,h),f(h)},update(h){const c=h(dt(i));r(t,c),f(c)},subscribe:a}}return me[t]}const He=at("storePrefersDarkScheme",!1),ce=at("storeLightSwitch",void 0);const St=t=>({}),Te=t=>({}),Et=t=>({}),Fe=t=>({});function Re(t){let e,l,n;const o=t[16].lead,s=B(o,t,t[15],Fe);return{c(){e=L("div"),s&&s.c(),d(e,"class",l="app-bar-lead "+t[4])},m(r,i){H(r,e,i),s&&s.m(e,null),n=!0},p(r,i){s&&s.p&&(!n||i&32768)&&q(s,o,r,r[15],n?O(o,r[15],i,Et):I(r[15]),Fe),(!n||i&16&&l!==(l="app-bar-lead "+r[4]))&&d(e,"class",l)},i(r){n||(k(s,r),n=!0)},o(r){v(s,r),n=!1},d(r){r&&P(e),s&&s.d(r)}}}function De(t){let e,l,n;const o=t[16].trail,s=B(o,t,t[15],Te);return{c(){e=L("div"),s&&s.c(),d(e,"class",l="app-bar-trail "+t[2])},m(r,i){H(r,e,i),s&&s.m(e,null),n=!0},p(r,i){s&&s.p&&(!n||i&32768)&&q(s,o,r,r[15],n?O(o,r[15],i,St):I(r[15]),Te),(!n||i&4&&l!==(l="app-bar-trail "+r[2]))&&d(e,"class",l)},i(r){n||(k(s,r),n=!0)},o(r){v(s,r),n=!1},d(r){r&&P(e),s&&s.d(r)}}}function Pt(t){let e,l,n,o,s,r,i,a=t[6].lead&&Re(t);const f=t[16].default,h=B(f,t,t[15],null);let c=t[6].trail&&De(t);return{c(){e=L("div"),a&&a.c(),l=z(),n=L("div"),h&&h.c(),s=z(),c&&c.c(),d(n,"class",o="app-bar-center "+t[3]),d(e,"class",r="app-bar "+t[5]),d(e,"data-testid","app-bar"),d(e,"role","toolbar"),d(e,"aria-label",t[0]),d(e,"aria-labelledby",t[1])},m(m,_){H(m,e,_),a&&a.m(e,null),w(e,l),w(e,n),h&&h.m(n,null),w(e,s),c&&c.m(e,null),i=!0},p(m,[_]){m[6].lead?a?(a.p(m,_),_&64&&k(a,1)):(a=Re(m),a.c(),k(a,1),a.m(e,l)):a&&(j(),v(a,1,1,()=>{a=null}),U()),h&&h.p&&(!i||_&32768)&&q(h,f,m,m[15],i?O(f,m[15],_,null):I(m[15]),null),(!i||_&8&&o!==(o="app-bar-center "+m[3]))&&d(n,"class",o),m[6].trail?c?(c.p(m,_),_&64&&k(c,1)):(c=De(m),c.c(),k(c,1),c.m(e,null)):c&&(j(),v(c,1,1,()=>{c=null}),U()),(!i||_&32&&r!==(r="app-bar "+m[5]))&&d(e,"class",r),(!i||_&1)&&d(e,"aria-label",m[0]),(!i||_&2)&&d(e,"aria-labelledby",m[1])},i(m){i||(k(a),k(h,m),k(c),i=!0)},o(m){v(a),v(h,m),v(c),i=!1},d(m){m&&P(e),a&&a.d(),h&&h.d(m),c&&c.d()}}}const Ct="flex items-center",Ht="flex-none flex justify-between items-center",Tt="flex-auto",Ft="flex-none flex items-center space-x-4";function Rt(t,e,l){let n,o,s,r,{$$slots:i={},$$scope:a}=e;const f=nt(i);let{background:h="bg-surface-100-800-token"}=e,{border:c=""}=e,{padding:m="p-4"}=e,{shadow:_="shadow-lg"}=e,{space:b="space-x-4"}=e,{slotLead:u=""}=e,{slotDefault:S=""}=e,{slotTrail:C=""}=e,{label:y=""}=e,{labelledby:E=""}=e;return t.$$set=p=>{l(17,e=x(x({},e),re(p))),"background"in p&&l(7,h=p.background),"border"in p&&l(8,c=p.border),"padding"in p&&l(9,m=p.padding),"shadow"in p&&l(10,_=p.shadow),"space"in p&&l(11,b=p.space),"slotLead"in p&&l(12,u=p.slotLead),"slotDefault"in p&&l(13,S=p.slotDefault),"slotTrail"in p&&l(14,C=p.slotTrail),"label"in p&&l(0,y=p.label),"labelledby"in p&&l(1,E=p.labelledby),"$$scope"in p&&l(15,a=p.$$scope)},t.$$.update=()=>{l(5,n=`${Ct} ${h} ${c} ${m} ${_} ${b} ${e.class??""}`),t.$$.dirty&4096&&l(4,o=`${Ht} ${u}`),t.$$.dirty&8192&&l(3,s=`${Tt} ${S}`),t.$$.dirty&16384&&l(2,r=`${Ft} ${C}`)},e=re(e),[y,E,r,s,o,n,f,h,c,m,_,b,u,S,C,a,i]}class ft extends Y{constructor(e){super(),X(this,e,Rt,Pt,J,{background:7,border:8,padding:9,shadow:10,space:11,slotLead:12,slotDefault:13,slotTrail:14,label:0,labelledby:1})}}const Dt=t=>({}),Ne=t=>({}),Nt=t=>({}),Me=t=>({}),Mt=t=>({}),ze=t=>({}),zt=t=>({}),Ae=t=>({}),At=t=>({}),Be=t=>({}),Bt=t=>({}),Oe=t=>({});function qe(t){let e,l,n;const o=t[17].header,s=B(o,t,t[16],Oe);return{c(){e=L("header"),s&&s.c(),d(e,"id","shell-header"),d(e,"class",l="flex-none "+t[6])},m(r,i){H(r,e,i),s&&s.m(e,null),n=!0},p(r,i){s&&s.p&&(!n||i&65536)&&q(s,o,r,r[16],n?O(o,r[16],i,Bt):I(r[16]),Oe),(!n||i&64&&l!==(l="flex-none "+r[6]))&&d(e,"class",l)},i(r){n||(k(s,r),n=!0)},o(r){v(s,r),n=!1},d(r){r&&P(e),s&&s.d(r)}}}function Ie(t){let e,l;const n=t[17].sidebarLeft,o=B(n,t,t[16],Be);return{c(){e=L("aside"),o&&o.c(),d(e,"id","sidebar-left"),d(e,"class",t[5])},m(s,r){H(s,e,r),o&&o.m(e,null),l=!0},p(s,r){o&&o.p&&(!l||r&65536)&&q(o,n,s,s[16],l?O(n,s[16],r,At):I(s[16]),Be),(!l||r&32)&&d(e,"class",s[5])},i(s){l||(k(o,s),l=!0)},o(s){v(o,s),l=!1},d(s){s&&P(e),o&&o.d(s)}}}function je(t){let e,l,n;const o=t[17].pageHeader,s=B(o,t,t[16],Ae),r=s||Ot();return{c(){e=L("header"),r&&r.c(),d(e,"id","page-header"),d(e,"class",l="flex-none "+t[3])},m(i,a){H(i,e,a),r&&r.m(e,null),n=!0},p(i,a){s&&s.p&&(!n||a&65536)&&q(s,o,i,i[16],n?O(o,i[16],a,zt):I(i[16]),Ae),(!n||a&8&&l!==(l="flex-none "+i[3]))&&d(e,"class",l)},i(i){n||(k(r,i),n=!0)},o(i){v(r,i),n=!1},d(i){i&&P(e),r&&r.d(i)}}}function Ot(t){let e;return{c(){e=M("(slot:header)")},m(l,n){H(l,e,n)},d(l){l&&P(e)}}}function Ue(t){let e,l,n;const o=t[17].pageFooter,s=B(o,t,t[16],ze),r=s||qt();return{c(){e=L("footer"),r&&r.c(),d(e,"id","page-footer"),d(e,"class",l="flex-none "+t[1])},m(i,a){H(i,e,a),r&&r.m(e,null),n=!0},p(i,a){s&&s.p&&(!n||a&65536)&&q(s,o,i,i[16],n?O(o,i[16],a,Mt):I(i[16]),ze),(!n||a&2&&l!==(l="flex-none "+i[1]))&&d(e,"class",l)},i(i){n||(k(r,i),n=!0)},o(i){v(r,i),n=!1},d(i){i&&P(e),r&&r.d(i)}}}function qt(t){let e;return{c(){e=M("(slot:footer)")},m(l,n){H(l,e,n)},d(l){l&&P(e)}}}function Je(t){let e,l;const n=t[17].sidebarRight,o=B(n,t,t[16],Me);return{c(){e=L("aside"),o&&o.c(),d(e,"id","sidebar-right"),d(e,"class",t[4])},m(s,r){H(s,e,r),o&&o.m(e,null),l=!0},p(s,r){o&&o.p&&(!l||r&65536)&&q(o,n,s,s[16],l?O(n,s[16],r,Nt):I(s[16]),Me),(!l||r&16)&&d(e,"class",s[4])},i(s){l||(k(o,s),l=!0)},o(s){v(o,s),l=!1},d(s){s&&P(e),o&&o.d(s)}}}function Ke(t){let e,l,n;const o=t[17].footer,s=B(o,t,t[16],Ne);return{c(){e=L("footer"),s&&s.c(),d(e,"id","shell-footer"),d(e,"class",l="flex-none "+t[0])},m(r,i){H(r,e,i),s&&s.m(e,null),n=!0},p(r,i){s&&s.p&&(!n||i&65536)&&q(s,o,r,r[16],n?O(o,r[16],i,Dt):I(r[16]),Ne),(!n||i&1&&l!==(l="flex-none "+r[0]))&&d(e,"class",l)},i(r){n||(k(s,r),n=!0)},o(r){v(s,r),n=!1},d(r){r&&P(e),s&&s.d(r)}}}function It(t){let e,l,n,o,s,r,i,a,f,h,c,m,_=t[8].header&&qe(t),b=t[8].sidebarLeft&&Ie(t),u=t[8].pageHeader&&je(t);const S=t[17].default,C=B(S,t,t[16],null);let y=t[8].pageFooter&&Ue(t),E=t[8].sidebarRight&&Je(t),p=t[8].footer&&Ke(t);return{c(){e=L("div"),_&&_.c(),l=z(),n=L("div"),b&&b.c(),o=z(),s=L("div"),u&&u.c(),r=z(),i=L("main"),C&&C.c(),f=z(),y&&y.c(),h=z(),E&&E.c(),c=z(),p&&p.c(),d(i,"id","page-content"),d(i,"class",a="flex-auto "+t[2]),d(s,"id","page"),d(s,"class",Jt),d(n,"class","flex-auto "+Ut),d(e,"id","appShell"),d(e,"class",t[7]),d(e,"data-testid","app-shell")},m(g,T){H(g,e,T),_&&_.m(e,null),w(e,l),w(e,n),b&&b.m(n,null),w(n,o),w(n,s),u&&u.m(s,null),w(s,r),w(s,i),C&&C.m(i,null),w(s,f),y&&y.m(s,null),w(n,h),E&&E.m(n,null),w(e,c),p&&p.m(e,null),m=!0},p(g,[T]){g[8].header?_?(_.p(g,T),T&256&&k(_,1)):(_=qe(g),_.c(),k(_,1),_.m(e,l)):_&&(j(),v(_,1,1,()=>{_=null}),U()),g[8].sidebarLeft?b?(b.p(g,T),T&256&&k(b,1)):(b=Ie(g),b.c(),k(b,1),b.m(n,o)):b&&(j(),v(b,1,1,()=>{b=null}),U()),g[8].pageHeader?u?(u.p(g,T),T&256&&k(u,1)):(u=je(g),u.c(),k(u,1),u.m(s,r)):u&&(j(),v(u,1,1,()=>{u=null}),U()),C&&C.p&&(!m||T&65536)&&q(C,S,g,g[16],m?O(S,g[16],T,null):I(g[16]),null),(!m||T&4&&a!==(a="flex-auto "+g[2]))&&d(i,"class",a),g[8].pageFooter?y?(y.p(g,T),T&256&&k(y,1)):(y=Ue(g),y.c(),k(y,1),y.m(s,null)):y&&(j(),v(y,1,1,()=>{y=null}),U()),g[8].sidebarRight?E?(E.p(g,T),T&256&&k(E,1)):(E=Je(g),E.c(),k(E,1),E.m(n,null)):E&&(j(),v(E,1,1,()=>{E=null}),U()),g[8].footer?p?(p.p(g,T),T&256&&k(p,1)):(p=Ke(g),p.c(),k(p,1),p.m(e,null)):p&&(j(),v(p,1,1,()=>{p=null}),U()),(!m||T&128)&&d(e,"class",g[7])},i(g){m||(k(_),k(b),k(u),k(C,g),k(y),k(E),k(p),m=!0)},o(g){v(_),v(b),v(u),v(C,g),v(y),v(E),v(p),m=!1},d(g){g&&P(e),_&&_.d(),b&&b.d(),u&&u.d(),C&&C.d(g),y&&y.d(),E&&E.d(),p&&p.d()}}}const jt="w-full h-full flex flex-col overflow-hidden",Ut="w-full h-full flex overflow-hidden",Jt="flex-1 overflow-x-hidden overflow-y-auto flex flex-col",Kt="flex-none overflow-x-hidden overflow-y-auto",Vt="flex-none overflow-x-hidden overflow-y-auto";function Wt(t,e,l){let n,o,s,r,i,a,f,h,{$$slots:c={},$$scope:m}=e;const _=nt(c);let{slotHeader:b="z-10"}=e,{slotSidebarLeft:u="w-auto"}=e,{slotSidebarRight:S="w-auto"}=e,{slotPageHeader:C=""}=e,{slotPageContent:y=""}=e,{slotPageFooter:E=""}=e,{slotFooter:p=""}=e;return t.$$set=g=>{l(18,e=x(x({},e),re(g))),"slotHeader"in g&&l(9,b=g.slotHeader),"slotSidebarLeft"in g&&l(10,u=g.slotSidebarLeft),"slotSidebarRight"in g&&l(11,S=g.slotSidebarRight),"slotPageHeader"in g&&l(12,C=g.slotPageHeader),"slotPageContent"in g&&l(13,y=g.slotPageContent),"slotPageFooter"in g&&l(14,E=g.slotPageFooter),"slotFooter"in g&&l(15,p=g.slotFooter),"$$scope"in g&&l(16,m=g.$$scope)},t.$$.update=()=>{l(7,n=`${jt} ${e.class??""}`),t.$$.dirty&512&&l(6,o=`${b}`),t.$$.dirty&1024&&l(5,s=`${Kt} ${u}`),t.$$.dirty&2048&&l(4,r=`${Vt} ${S}`),t.$$.dirty&4096&&l(3,i=`${C}`),t.$$.dirty&8192&&l(2,a=`${y}`),t.$$.dirty&16384&&l(1,f=`${E}`),t.$$.dirty&32768&&l(0,h=`${p}`)},e=re(e),[h,f,a,i,r,s,o,n,_,b,u,S,C,y,E,p,m,c]}class Gt extends Y{constructor(e){super(),X(this,e,Wt,It,J,{slotHeader:9,slotSidebarLeft:10,slotSidebarRight:11,slotPageHeader:12,slotPageContent:13,slotPageFooter:14,slotFooter:15})}}const{document:Ve}=it;function Qt(t){let e,l;return{c(){e=ae("svg"),l=ae("path"),d(l,"d","M223.5 32C100 32 0 132.3 0 256S100 480 223.5 480c60.6 0 115.5-24.2 155.8-63.4c5-4.9 6.3-12.5 3.1-18.7s-10.1-9.7-17-8.5c-9.8 1.7-19.8 2.6-30.1 2.6c-96.9 0-175.5-78.8-175.5-176c0-65.8 36-123.1 89.3-153.3c6.1-3.5 9.2-10.5 7.7-17.3s-7.3-11.9-14.3-12.5c-6.3-.5-12.6-.8-19-.8z"),d(e,"class","lightswitch-icon fill-white "+ct),d(e,"xmlns","http://www.w3.org/2000/svg"),d(e,"viewBox","0 0 384 512")},m(n,o){H(n,e,o),w(e,l)},p:R,d(n){n&&P(e)}}}function Xt(t){let e,l;return{c(){e=ae("svg"),l=ae("path"),d(l,"d","M361.5 1.2c5 2.1 8.6 6.6 9.6 11.9L391 121l107.9 19.8c5.3 1 9.8 4.6 11.9 9.6s1.5 10.7-1.6 15.2L446.9 256l62.3 90.3c3.1 4.5 3.7 10.2 1.6 15.2s-6.6 8.6-11.9 9.6L391 391 371.1 498.9c-1 5.3-4.6 9.8-9.6 11.9s-10.7 1.5-15.2-1.6L256 446.9l-90.3 62.3c-4.5 3.1-10.2 3.7-15.2 1.6s-8.6-6.6-9.6-11.9L121 391 13.1 371.1c-5.3-1-9.8-4.6-11.9-9.6s-1.5-10.7 1.6-15.2L65.1 256 2.8 165.7c-3.1-4.5-3.7-10.2-1.6-15.2s6.6-8.6 11.9-9.6L121 121 140.9 13.1c1-5.3 4.6-9.8 9.6-11.9s10.7-1.5 15.2 1.6L256 65.1 346.3 2.8c4.5-3.1 10.2-3.7 15.2-1.6zM352 256c0 53-43 96-96 96s-96-43-96-96s43-96 96-96s96 43 96 96zm32 0c0-70.7-57.3-128-128-128s-128 57.3-128 128s57.3 128 128 128s128-57.3 128-128z"),d(e,"class","lightswitch-icon fill-black "+ct),d(e,"xmlns","http://www.w3.org/2000/svg"),d(e,"viewBox","0 0 512 512")},m(n,o){H(n,e,o),w(e,l)},p:R,d(n){n&&P(e)}}}function Yt(t){let e,l=`<script>${$t.toString()} setColorScheme();<\/script>`,n,o,s,r,i,a,f,h,c;function m(u,S){return u[0]===!1?Xt:Qt}let _=m(t),b=_(t);return{c(){e=new gt(!1),n=ht(),o=z(),s=L("div"),r=L("div"),b.c(),e.a=n,d(r,"class",i="lightswitch-thumb "+t[1]),d(s,"class",a="lightswitch "+t[2]),d(s,"role","switch"),d(s,"aria-label","Light Switch"),d(s,"aria-checked",t[0]),d(s,"title",f="Toggle "+(t[0]?"Light":"Dark")+" Mode"),d(s,"tabindex","0")},m(u,S){e.m(l,Ve.head),w(Ve.head,n),H(u,o,S),H(u,s,S),w(s,r),b.m(r,null),h||(c=[se(s,"click",t[3]),se(s,"keydown",t[4]),se(s,"keyup",t[6]),se(s,"keypress",t[7])],h=!0)},p(u,[S]){_===(_=m(u))&&b?b.p(u,S):(b.d(1),b=_(u),b&&(b.c(),b.m(r,null))),S&2&&i!==(i="lightswitch-thumb "+u[1])&&d(r,"class",i),S&4&&a!==(a="lightswitch "+u[2])&&d(s,"class",a),S&1&&d(s,"aria-checked",u[0]),S&1&&f!==(f="Toggle "+(u[0]?"Light":"Dark")+" Mode")&&d(s,"title",f)},i:R,o:R,d(u){P(n),u&&e.d(),u&&P(o),u&&P(s),b.d(),h=!1,$(c)}}}const Zt="inline-block bg-surface-200-700-token ring-[1px] ring-surface-300-600-token ring-inset w-12 h-6 rounded-full cursor-pointer transition-all duration-[100ms]",xt="bg-white dark:bg-black fill-white dark:fill-black w-6 h-6 flex justify-center items-center rounded-full shadow-lg transition-all duration-[100ms] scale-90",ct="block w-4 h-4";function $t(){localStorage.getItem("storeLightSwitch")==="true"||!("storeLightSwitch"in localStorage)&&window.matchMedia("(prefers-color-scheme: dark)").matches?document.documentElement.classList.add("dark"):document.documentElement.classList.remove("dark")}function el(t,e,l){let n,o,s,r,i;ve(t,ce,u=>l(0,r=u)),ve(t,He,u=>l(8,i=u));const a=pt();function f(){const u=window.matchMedia("(prefers-color-scheme: dark)").matches;He.set(u),r===void 0&&Le(ce,r=i,r)}function h(){const u=document.documentElement.classList;r?u.add("dark"):u.remove("dark")}function c(u){ce.set(Le(ce,r=!r,r)),h(),a("click",u)}function m(u){["Enter","Space"].includes(u.code)&&(u.preventDefault(),u.currentTarget.click()),a("keydown",u)}bt(()=>{f()});function _(u){Se.call(this,t,u)}function b(u){Se.call(this,t,u)}return t.$$set=u=>{l(12,e=x(x({},e),re(u)))},t.$$.update=()=>{t.$$.dirty&1&&l(5,n=r?"translate-x-full":"translate-x-0"),l(2,o=`${Zt} ${e.class??""}`),t.$$.dirty&32&&l(1,s=`${xt} ${n}`)},e=re(e),[r,s,o,c,m,n,_,b]}class tl extends Y{constructor(e){super(),X(this,e,el,Yt,J,{})}}function ll(t){let e;return{c(){e=L("h1"),e.textContent="Lobby",d(e,"class","text-center")},m(l,n){H(l,e,n)},p:R,d(l){l&&P(e)}}}function nl(t){let e,l,n,o;return{c(){e=L("div"),l=L("p"),l.textContent=`Olá "${t[0]??"clebinho157"}"`,n=z(),o=L("a"),o.textContent="Trocar de nick",d(o,"href","/"),d(o,"class","btn btn-sm ring-2 ring-secondary-500 ring-inset"),d(e,"class","")},m(s,r){H(s,e,r),w(e,l),w(e,n),w(e,o)},p:R,d(s){s&&P(e)}}}function sl(t){let e,l;return e=new tl({}),{c(){Q(e.$$.fragment)},m(n,o){K(e,n,o),l=!0},i(n){l||(k(e.$$.fragment,n),l=!0)},o(n){v(e.$$.fragment,n),l=!1},d(n){V(e,n)}}}function rl(t){let e,l;return e=new ft({props:{$$slots:{trail:[sl],lead:[nl],default:[ll]},$$scope:{ctx:t}}}),{c(){Q(e.$$.fragment)},m(n,o){K(e,n,o),l=!0},p(n,[o]){const s={};o&2&&(s.$$scope={dirty:o,ctx:n}),e.$set(s)},i(n){l||(k(e.$$.fragment,n),l=!0)},o(n){v(e.$$.fragment,n),l=!1},d(n){V(e,n)}}}function ol(t){var e="; "+document.cookie,l=e.split("; "+t+"=");if(l.length==2)return l.pop().split(";").shift()}function il(t){return[ol("name")]}class al extends Y{constructor(e){super(),X(this,e,il,rl,J,{})}}function fl(t){let e;return{c(){e=L("div"),e.innerHTML=`<h2>Insert a new room</h2> 
    <hr/> 
    <form method="post" class="flex flex-col gap-4"><label for="name"><span>Rooms name</span> 
            <input type="text" id="name" name="name" value="Bongo 1"/></label> 
        <label for="number_of_players"><span>Max numbers of players:</span> 
            <input type="number" id="number_of_players" name="number_of_players" min="2" max="16" value="8"/></label> 
        
        <input type="submit" value="New room" class="btn btn-ringed-primary btn-base"/></form>`,d(e,"class","gambiarra svelte-rz8ztr")},m(l,n){H(l,e,n)},p:R,d(l){l&&P(e)}}}function cl(t){let e,l;return e=new ft({props:{$$slots:{default:[fl]},$$scope:{ctx:t}}}),{c(){Q(e.$$.fragment)},m(n,o){K(e,n,o),l=!0},p(n,[o]){const s={};o&1&&(s.$$scope={dirty:o,ctx:n}),e.$set(s)},i(n){l||(k(e.$$.fragment,n),l=!0)},o(n){v(e.$$.fragment,n),l=!1},d(n){V(e,n)}}}class ul extends Y{constructor(e){super(),X(this,e,null,cl,J,{})}}function We(t,e,l){const n=t.slice();return n[6]=e[l],n}function Ge(t){let e;return{c(){e=M("Error")},m(l,n){H(l,e,n)},d(l){l&&P(e)}}}function Qe(t){var o;let e,l=((o=t[6])==null?void 0:o.name)+"",n;return{c(){e=L("li"),n=M(l)},m(s,r){H(s,e,r),w(e,n)},p(s,r){var i;r&4&&l!==(l=((i=s[6])==null?void 0:i.name)+"")&&oe(n,l)},d(s){s&&P(e)}}}function dl(t){var be,pe;let e,l,n,o,s,r,i,a=((be=t[3])==null?void 0:be.name)+"",f,h,c,m,_=((pe=t[2])==null?void 0:pe.length)+"",b,u,S,C,y,E,p,g,T,W=t[2],D=[];for(let F=0;F<W.length;F+=1)D[F]=Qe(We(t,W,F));let N=null;return W.length||(N=Ge()),{c(){e=L("div"),l=L("h2"),n=M(t[0]),o=z(),s=L("div"),r=L("header"),i=M("Sala do "),f=M(a),h=z(),c=L("div"),m=M("Quantidade de players:"),b=M(_),u=M("/"),S=M(t[1]),C=z(),y=L("ol");for(let F=0;F<D.length;F+=1)D[F].c();N&&N.c(),E=z(),p=L("footer"),g=L("a"),T=M("Entrar"),d(r,"class","card-header"),d(c,"class","p-4"),d(y,"class","p-4"),d(g,"href",t[4]),d(g,"class","btn btn-filled-secondary btn-base"),d(p,"class","card-footer"),d(s,"class","card card-glass-secondary"),d(e,"class","cont-card-room")},m(F,G){H(F,e,G),w(e,l),w(l,n),w(e,o),w(e,s),w(s,r),w(r,i),w(r,f),w(s,h),w(s,c),w(c,m),w(c,b),w(c,u),w(c,S),w(s,C),w(s,y);for(let ee=0;ee<D.length;ee+=1)D[ee].m(y,null);N&&N.m(y,null),w(s,E),w(s,p),w(p,g),w(g,T)},p(F,[G]){var ee,ke;if(G&1&&oe(n,F[0]),G&8&&a!==(a=((ee=F[3])==null?void 0:ee.name)+"")&&oe(f,a),G&4&&_!==(_=((ke=F[2])==null?void 0:ke.length)+"")&&oe(b,_),G&2&&oe(S,F[1]),G&4){W=F[2];let A;for(A=0;A<W.length;A+=1){const we=We(F,W,A);D[A]?D[A].p(we,G):(D[A]=Qe(we),D[A].c(),D[A].m(y,null))}for(;A<D.length;A+=1)D[A].d(1);D.length=W.length,W.length?N&&(N.d(1),N=null):N||(N=Ge(),N.c(),N.m(y,null))}},i:R,o:R,d(F){F&&P(e),st(D,F),N&&N.d()}}}function hl(t,e,l){let n,{nome_sala:o="default name"}=e,{qtd_players_max:s=1}=e,{path:r="lobby"}=e,{users:i=[]}=e,a=`${window.location.href}/${r}`;return t.$$set=f=>{"nome_sala"in f&&l(0,o=f.nome_sala),"qtd_players_max"in f&&l(1,s=f.qtd_players_max),"path"in f&&l(5,r=f.path),"users"in f&&l(2,i=f.users)},t.$$.update=()=>{t.$$.dirty&4&&l(3,n=i==null?void 0:i.find(f=>f.is_admin==!0))},[o,s,i,n,a,r]}class ml extends Y{constructor(e){super(),X(this,e,hl,dl,J,{nome_sala:0,qtd_players_max:1,path:5,users:2})}}const{window:Xe}=it;function Ye(t,e,l){const n=t.slice();return n[8]=e[l],n}function Ze(t){let e;return{c(){e=L("h1"),e.textContent="Nenhuma sala disponivel no momento"},m(l,n){H(l,e,n)},p:R,d(l){l&&P(e)}}}function xe(t){var o,s,r,i;let e,l;function n(...a){return t[4](t[8],...a)}return e=new ml({props:{nome_sala:(o=t[8])==null?void 0:o.name,qtd_players_max:(s=t[8])==null?void 0:s.max_number_players,path:(r=t[8])==null?void 0:r.id,users:(i=t[0])==null?void 0:i.filter(n)}}),{c(){Q(e.$$.fragment)},m(a,f){K(e,a,f),l=!0},p(a,f){var c,m,_,b;t=a;const h={};f&2&&(h.nome_sala=(c=t[8])==null?void 0:c.name),f&2&&(h.qtd_players_max=(m=t[8])==null?void 0:m.max_number_players),f&2&&(h.path=(_=t[8])==null?void 0:_.id),f&3&&(h.users=(b=t[0])==null?void 0:b.filter(n)),e.$set(h)},i(a){l||(k(e.$$.fragment,a),l=!0)},o(a){v(e.$$.fragment,a),l=!1},d(a){V(e,a)}}}function _l(t){let e,l,n,o,s=t[1],r=[];for(let f=0;f<s.length;f+=1)r[f]=xe(Ye(t,s,f));const i=f=>v(r[f],1,1,()=>{r[f]=null});let a=null;return s.length||(a=Ze()),{c(){e=L("div");for(let f=0;f<r.length;f+=1)r[f].c();a&&a.c(),d(e,"class","grid gap-4 grid-cols-4 m-6")},m(f,h){H(f,e,h);for(let c=0;c<r.length;c+=1)r[c].m(e,null);a&&a.m(e,null),l=!0,n||(o=[se(Xe,"pagehide",t[2]),se(Xe,"pageshow",t[3])],n=!0)},p(f,[h]){if(h&3){s=f[1];let c;for(c=0;c<s.length;c+=1){const m=Ye(f,s,c);r[c]?(r[c].p(m,h),k(r[c],1)):(r[c]=xe(m),r[c].c(),k(r[c],1),r[c].m(e,null))}for(j(),c=s.length;c<r.length;c+=1)i(c);U(),!s.length&&a?a.p(f,h):s.length?a&&(a.d(1),a=null):(a=Ze(),a.c(),a.m(e,null))}},i(f){if(!l){for(let h=0;h<s.length;h+=1)k(r[h]);l=!0}},o(f){r=r.filter(Boolean);for(let h=0;h<r.length;h+=1)v(r[h]);l=!1},d(f){f&&P(e),st(r,f),a&&a.d(),n=!1,$(o)}}}function gl(t,e,l){let n,o;var s=null;function r(){i();const{location:c}=window,_=`${c.protocol.startsWith("https")?"wss":"ws"}://${c.host}${c.pathname}/ws`;s=new WebSocket(_),s.onopen=()=>{console.log("Eu entrei")},s.onmessage=b=>{var S,C;let u=JSON.parse(b.data);switch(console.log(u),u.msg_type){case"Update":switch(u.action){case"Add":l(1,n=[...n,u.room.Room]),l(0,o=[...o,u.user.Connection]);break;case"Delete":l(1,n=n==null?void 0:n.filter(y=>y.id!=u.room.Uuid)),l(0,o=o==null?void 0:o.filter(y=>y.room_id!=u.room.Uuid));break}break;case"Initial":l(1,n=(S=u.room)==null?void 0:S.Rooms),l(0,o=(C=u.user)==null?void 0:C.Connections);break}},s.onclose=()=>{s=null}}function i(){s&&(s.close(),s=null)}r();function a(){s.close()}function f(c){c.persisted?(console.log("This page was restored from the bfcache."),r()):console.log("This page was loaded normally.")}const h=(c,m)=>m.room_id==(c==null?void 0:c.id);return t.$$.update=()=>{t.$$.dirty&2&&console.log(n),t.$$.dirty&1&&console.log(o)},l(1,n=[]),l(0,o=[]),[o,n,a,f,h]}class bl extends Y{constructor(e){super(),X(this,e,gl,_l,J,{})}}function pl(t){let e,l;return e=new bl({}),{c(){Q(e.$$.fragment)},m(n,o){K(e,n,o),l=!0},i(n){l||(k(e.$$.fragment,n),l=!0)},o(n){v(e.$$.fragment,n),l=!1},d(n){V(e,n)}}}function kl(t){let e,l;return e=new al({}),{c(){Q(e.$$.fragment)},m(n,o){K(e,n,o),l=!0},i(n){l||(k(e.$$.fragment,n),l=!0)},o(n){v(e.$$.fragment,n),l=!1},d(n){V(e,n)}}}function wl(t){let e,l;return e=new ul({}),{c(){Q(e.$$.fragment)},m(n,o){K(e,n,o),l=!0},i(n){l||(k(e.$$.fragment,n),l=!0)},o(n){v(e.$$.fragment,n),l=!1},d(n){V(e,n)}}}function yl(t){let e,l;return e=new Gt({props:{$$slots:{sidebarLeft:[wl],header:[kl],default:[pl]},$$scope:{ctx:t}}}),{c(){Q(e.$$.fragment)},m(n,o){K(e,n,o),l=!0},p(n,[o]){const s={};o&1&&(s.$$scope={dirty:o,ctx:n}),e.$set(s)},i(n){l||(k(e.$$.fragment,n),l=!0)},o(n){v(e.$$.fragment,n),l=!1},d(n){V(e,n)}}}class vl extends Y{constructor(e){super(),X(this,e,null,yl,J,{})}}new vl({target:document.getElementById("app")});
