(() => { const re=new RegExp(__ARG__); let hit=null;
 const walk=(root)=>{ for (const e of root.querySelectorAll('*')) { if (hit) return; if (e.shadowRoot) walk(e.shadowRoot); if (e.children.length===0 && re.test((e.textContent||'').trim())) hit=e; } };
 walk(document); if(!hit) return 'missing'; const c=hit.closest('button,a,[role=button],[role=option],li,mat-list-item')||hit; c.click(); return 'clicked '+c.tagName; })()
