(() => { if (/Generating will use/.test(document.body.innerText)) return 'open'; document.querySelector('[aria-label="Settings trigger"]').click(); return 'opened'; })()
