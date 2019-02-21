import('./pkg/lorenz_attractor')
    .then(wasm => {
        const canvas = document.getElementById('drawing');
        const ctx = canvas.getContext('2d');

        const xInput = document.getElementById('startx');
        const yInput = document.getElementById('starty');
        const zInput = document.getElementById('startz');
        const renderBtn = document.getElementById('render');

        renderBtn.addEventListener('click', () => {
            const x = parseFloat(xInput.value) || 0;
            const y = parseFloat(yInput.value) || 0;
            const z = parseFloat(zInput.value) || 0;
            wasm.draw(ctx, 600, 600, x, y, z);
        });

        wasm.draw(ctx, 600, 600, 0, 0, 0);
    })
    .catch(console.error);
