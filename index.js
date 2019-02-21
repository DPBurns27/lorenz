import('./pkg/lorenz_attractor')
    .then(wasm => {
        const canvas = document.getElementById('drawing');
        const ctx = canvas.getContext('2d');

        const realInput = document.getElementById('real');
        const imaginaryInput = document.getElementById('imaginary');
        const renderBtn = document.getElementById('render');

//        renderBtn.addEventListener('click', () => {
//            const real = parseFloat(realInput.value) || 0;
//            const imaginary = parseFloat(imaginaryInput.value) || 0;
//            wasm.draw(ctx, 600, 600, real, imaginary);
//        });

        wasm.draw(ctx, 600, 600, 0.0, 0.0, 0.0);
    })
    .catch(console.error);