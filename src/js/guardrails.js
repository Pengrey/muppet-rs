(() => {
    return new Promise((resolve) => {
        const e = new Image();
        // The resource URL for the Keeper extension.
        e.src = 'chrome-extension://bfogiafebfohielmmehodmfbbebbbpei/images/ico-field-fill-lock-disabled.svg';

        // If it loads, the extension is present.
        e.onload = () => resolve(true);

        // If it fails, the extension is not present.
        e.onerror = () => resolve(false);
    });
})()
