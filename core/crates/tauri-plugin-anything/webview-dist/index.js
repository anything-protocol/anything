function t(t,n,e,r){return new(e||(e=Promise))((function(o,i){function u(t){try{s(r.next(t))}catch(t){i(t)}}function c(t){try{s(r.throw(t))}catch(t){i(t)}}function s(t){var n;t.done?o(t.value):(n=t.value,n instanceof e?n:new e((function(t){t(n)}))).then(u,c)}s((r=r.apply(t,n||[])).next())}))}function n(t,n){var e,r,o,i,u={label:0,sent:function(){if(1&o[0])throw o[1];return o[1]},trys:[],ops:[]};return i={next:c(0),throw:c(1),return:c(2)},"function"==typeof Symbol&&(i[Symbol.iterator]=function(){return this}),i;function c(c){return function(s){return function(c){if(e)throw new TypeError("Generator is already executing.");for(;i&&(i=0,c[0]&&(u=0)),u;)try{if(e=1,r&&(o=2&c[0]?r.return:c[0]?r.throw||((o=r.return)&&o.call(r),0):r.next)&&!(o=o.call(r,c[1])).done)return o;switch(r=0,o&&(c=[2&c[0],o.value]),c[0]){case 0:case 1:o=c;break;case 4:return u.label++,{value:c[1],done:!1};case 5:u.label++,r=c[1],c=[0];continue;case 7:c=u.ops.pop(),u.trys.pop();continue;default:if(!(o=u.trys,(o=o.length>0&&o[o.length-1])||6!==c[0]&&2!==c[0])){u=0;continue}if(3===c[0]&&(!o||c[1]>o[0]&&c[1]<o[3])){u.label=c[1];break}if(6===c[0]&&u.label<o[1]){u.label=o[1],o=c;break}if(o&&u.label<o[2]){u.label=o[2],u.ops.push(c);break}o[2]&&u.ops.pop(),u.trys.pop();continue}c=n.call(t,u)}catch(t){c=[6,t],r=0}finally{e=o=0}if(5&c[0])throw c[1];return{value:c[0]?c[1]:void 0,done:!0}}([c,s])}}}function e(t,n=!1){const e=window.crypto.getRandomValues(new Uint32Array(1))[0],r=`_${e}`;return Object.defineProperty(window,r,{value:e=>(n&&Reflect.deleteProperty(window,r),null==t?void 0:t(e)),writable:!1,configurable:!0}),e}async function r(t,n={}){return new Promise(((r,o)=>{const i=e((t=>{r(t),Reflect.deleteProperty(window,`_${u}`)}),!0),u=e((t=>{o(t),Reflect.deleteProperty(window,`_${i}`)}),!0);window.__TAURI_IPC__({cmd:t,callback:i,error:u,...n})}))}"function"==typeof SuppressedError&&SuppressedError;var o=function(){function e(t){this.path=t}return e.prototype.stop=function(){return t(this,void 0,void 0,(function(){return n(this,(function(t){switch(t.label){case 0:return[4,r("plugin:anything|stop",{})];case 1:return[2,t.sent()]}}))}))},e.prototype.getFlows=function(){return t(this,void 0,void 0,(function(){return n(this,(function(t){switch(t.label){case 0:return[4,r("plugin:anything|get_flows",{path:this.path})];case 1:return[2,t.sent()]}}))}))},e.prototype.getFlowByName=function(e){return t(this,void 0,void 0,(function(){return n(this,(function(t){switch(t.label){case 0:return[4,r("plugin:anything|get_flow_by_name",{flowName:e})];case 1:return[2,t.sent()]}}))}))},e.prototype.createFlow=function(e){return t(this,void 0,void 0,(function(){return n(this,(function(t){switch(t.label){case 0:return[4,r("plugin:anything|create_flow",{flowName:e})];case 1:return[2,t.sent()]}}))}))},e.prototype.CreateFlowVersion=function(e,o){return t(this,void 0,void 0,(function(){return n(this,(function(t){switch(t.label){case 0:return[4,r("plugin:anything|create_flow_version",{flowName:e,createFlowVersion:o})];case 1:return[2,t.sent()]}}))}))},e.prototype.updateFlow=function(e,o){return t(this,void 0,void 0,(function(){return n(this,(function(t){switch(t.label){case 0:return[4,r("plugin:anything|update_flow",{flowId:e,args:o})];case 1:return[2,t.sent()]}}))}))},e.prototype.deleteFlow=function(e){return t(this,void 0,void 0,(function(){return n(this,(function(t){switch(t.label){case 0:return[4,r("plugin:anything|delete_flow",{flowId:e})];case 1:return[2,t.sent()]}}))}))},e.prototype.updateFlowVersion=function(e,o,i){return t(this,void 0,void 0,(function(){return n(this,(function(t){switch(t.label){case 0:return[4,r("plugin:anything|update_flow_version",{flowId:e,flowVersionId:o,updateFlow:i})];case 1:return[2,t.sent()]}}))}))},e.prototype.executeFlow=function(e,o,i,u){return t(this,void 0,void 0,(function(){return n(this,(function(t){switch(t.label){case 0:return[4,r("plugin:anything|execute_flow",{flowId:e,flowVersionId:o,sessionId:i,stage:u})];case 1:return[2,t.sent()]}}))}))},e.prototype.fetchSessionEvents=function(e){return t(this,void 0,void 0,(function(){return n(this,(function(t){switch(t.label){case 0:return[4,r("plugin:anything|fetch_session_events",{sessionId:e})];case 1:return[2,t.sent()]}}))}))},e.prototype.getEvent=function(e){return t(this,void 0,void 0,(function(){return n(this,(function(t){switch(t.label){case 0:return[4,r("plugin:anything|get_event",{eventId:e})];case 1:return[2,t.sent()]}}))}))},e}();export{o as Anything};