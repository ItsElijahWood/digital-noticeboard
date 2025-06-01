pub fn login_page() -> String {
    r#"
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1.0" />
        <title>Notice Board Login</title>
        <link rel="icon" type="image/png" href="public/favicon.png" />
        <style>
            body {
                margin: 0;
                font-family: "Publicsans", sans-serif;
                background-color: #4a6da7;
                display: flex;
                flex-direction: column;
                align-items: center;
                justify-content: center;
                height: 100vh;
            }

            @font-face {
                font-family: "Publicsans";
                src: url(public/public-sans.ttf);
            }

            .header {
                position: absolute;
                top: 20px;
                left: 40px;
                font-size: 48px;
                color: #fff;
                font-weight: bold;
            }

            .leave-link {
                position: absolute;
                top: 30px;
                right: 40px;
                color: #fff;
                text-decoration: none;
                font-size: 18px;
                cursor: pointer;
                transition: opacity 0.2s;
            }

            .leave-link:hover {
                opacity: 0.7;
            }

            .login-container {
                background-color: rgba(255, 255, 255, 0.1);
                padding: 40px;
                border-radius: 12px;
                box-shadow: 0 0 20px rgba(0, 0, 0, 0.2);
                width: 300px;
                display: flex;
                flex-direction: column;
                gap: 20px;
            }

            .login-container label {
                color: #fff;
                font-size: 20px;
            }

            .login-container input {
                padding: 10px;
                border: none;
                border-radius: 6px;
                font-size: 16px;
                font-family: "Publicsans", sans-serif;
            }

            .login-container input:focus {
                outline: 2px solid #fff;
                background-color: #f0f0f0;
            }

            .submit-button {
                padding: 12px;
                border: none;
                border-radius: 8px;
                background-color: #2a4d8f;
                color: white;
                font-size: 18px;
                font-weight: bold;
                cursor: pointer;
                transition: background-color 0.3s ease;
            }

            .submit-button:hover {
                background-color: #1d3562;
            }
        </style>
    </head>
    <body>
        <div class="header">Noticeboard Login</div>

        <form class="login-container" id="loginForm">
            <label for="username">Congregation</label>
            <input type="text" id="username" name="username" placeholder="Enter congregation" required autocomplete="username"/>

            <label for="password">Password</label>
            <input type="password" id="password" name="password" placeholder="Enter password" required autocomplete="current-password"/>

            <button id="submit" type="submit" class="submit-button">Login</button>
        </form>

        <script>
            document.getElementById('loginForm').addEventListener('submit', async function(event) {
                event.preventDefault();

                const username = document.getElementById('username').value.trim();
                const password = document.getElementById('password').value;

                if (!username || !password) {
                    alert('Required fields not filled.');
                    return;
                }

                try {
                    const response = await fetch('/api/login', {
                        method: 'POST',
                        headers: {
                            'Content-Type': 'application/json'
                        },
                        body: JSON.stringify({ username, password })
                    });

                    if (response.ok) {
                        window.location.href = '/dashboard'; 
                    } else {
                        const data = await response.json();
                        alert(`${data.error}`);
                    }
                } catch (error) {
                    alert('Network error: ' + error.message);
                }
            });
        </script>
    </body>
    </html>
    "#.to_string()
}

