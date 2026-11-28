(() => { const m = document.body.innerText.match(/Generating will use[^\n]*/); return m ? m[0] : null; })()
