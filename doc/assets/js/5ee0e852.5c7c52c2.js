"use strict";(self.webpackChunkmithril_doc=self.webpackChunkmithril_doc||[]).push([[2332],{36563:(e,t,n)=>{n.r(t),n.d(t,{assets:()=>d,contentTitle:()=>r,default:()=>u,frontMatter:()=>i,metadata:()=>l,toc:()=>c});var s=n(74848),o=n(28453);const i={slug:2,title:"2. Use simple structured logging\n",authors:[{name:"Mithril Team"}],tags:["Superseded"],date:new Date("2022-04-24T00:00:00.000Z")},r=void 0,l={permalink:"/doc/adr/2",source:"@site/adr/002-use-structured-logging.md",title:"2. Use simple structured logging\n",description:"Status",date:"2022-04-24T00:00:00.000Z",tags:[{inline:!0,label:"Superseded",permalink:"/doc/adr/tags/superseded"}],readingTime:.68,hasTruncateMarker:!1,authors:[{name:"Mithril Team"}],frontMatter:{slug:"2",title:"2. Use simple structured logging\n",authors:[{name:"Mithril Team"}],tags:["Superseded"],date:"2022-04-24T00:00:00.000Z"},unlisted:!1,prevItem:{title:"3. Release process and versioning\n",permalink:"/doc/adr/3"},nextItem:{title:"1. Record Architecture Decisions\n",permalink:"/doc/adr/1"}},d={authorsImageUrls:[void 0]},c=[{value:"Status",id:"status",level:2},{value:"Context",id:"context",level:2},{value:"Decision",id:"decision",level:2},{value:"Consequences",id:"consequences",level:2}];function a(e){const t={a:"a",code:"code",em:"em",h2:"h2",li:"li",p:"p",ul:"ul",...(0,o.R)(),...e.components};return(0,s.jsxs)(s.Fragment,{children:[(0,s.jsx)(t.h2,{id:"status",children:"Status"}),"\n",(0,s.jsxs)(t.p,{children:["Superseded by ",(0,s.jsx)(t.a,{href:"/adr/7",children:"ADR 7"})]}),"\n",(0,s.jsx)(t.h2,{id:"context",children:"Context"}),"\n",(0,s.jsxs)(t.ul,{children:["\n",(0,s.jsxs)(t.li,{children:["Logs are a critical tool for operating any software system, enabling ",(0,s.jsx)(t.a,{href:"https://cloud.google.com/architecture/devops/devops-measurement-monitoring-and-observability",children:"observability"})," of the system."]}),"\n",(0,s.jsxs)(t.li,{children:["Following ",(0,s.jsx)(t.a,{href:"https://12factor.net/logs",children:"12 Factor Apps"})," principles, providing the needed components and tools to be able to configure logging and monitoring should not be the responsibility of the software components"]}),"\n"]}),"\n",(0,s.jsx)(t.h2,{id:"decision",children:"Decision"}),"\n",(0,s.jsx)(t.p,{children:(0,s.jsx)(t.em,{children:"Therefore"})}),"\n",(0,s.jsxs)(t.ul,{children:["\n",(0,s.jsxs)(t.li,{children:["Each component of the system use ",(0,s.jsx)(t.a,{href:"https://www.sumologic.com/glossary/structured-logging/",children:"Structured logging"})," using documented and standardised JSON format for its logs"]}),"\n",(0,s.jsxs)(t.li,{children:["Logs are always emitted to ",(0,s.jsx)(t.code,{children:"stdout"})," of the process the component is part of"]}),"\n"]}),"\n",(0,s.jsx)(t.h2,{id:"consequences",children:"Consequences"}),"\n",(0,s.jsxs)(t.ul,{children:["\n",(0,s.jsx)(t.li,{children:"The schema of the logged items should be properly documented in a JSON schema"}),"\n",(0,s.jsx)(t.li,{children:"It is the responsibility of the node operator to consume the logs and process them"}),"\n",(0,s.jsxs)(t.li,{children:["We use existing libraries to provide needed log infrastructure, like ",(0,s.jsx)(t.a,{href:"https://zsiciarz.github.io/24daysofrust/book/vol2/day4.html",children:"slog"})," for Rust"]}),"\n"]})]})}function u(e={}){const{wrapper:t}={...(0,o.R)(),...e.components};return t?(0,s.jsx)(t,{...e,children:(0,s.jsx)(a,{...e})}):a(e)}},28453:(e,t,n)=>{n.d(t,{R:()=>r,x:()=>l});var s=n(96540);const o={},i=s.createContext(o);function r(e){const t=s.useContext(i);return s.useMemo((function(){return"function"==typeof e?e(t):{...t,...e}}),[t,e])}function l(e){let t;return t=e.disableParentContext?"function"==typeof e.components?e.components(o):e.components||o:r(e.components),s.createElement(i.Provider,{value:t},e.children)}}}]);