import('./pkg/lorenz_attractor')
    .then(wasm => {
        const canvas = document.getElementById('drawing');
        const ctx = canvas.getContext('2d');

        const renderBtn = document.getElementById('render');

        renderBtn.addEventListener('click', () => {
            wasm.draw(ctx, 600, 600, 0, 0, 0);
        });

        wasm.draw(ctx, 600, 600, 0, 0, 0);
    })
    .catch(console.error);
