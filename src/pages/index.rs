/// Return HTML to content variable
pub fn index_page() -> String {
   r#"
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>Congregation Notice Board</title>
        <link rel="icon" type="image/png" href="public/favicon.png">
        <style>
            body {
                margin: 0;
                font-family: "Publicsans", sans-serif;
                background-color: #4a6da7;
            }

            @font-face {
                font-family: "Publicsans";
                src: url(public/public-sans.ttf);
            }

            .header {
                padding: 40px;
                display: flex;
                margin: 0;
                font-size: 48px;
                justify-content: space-between;
                color: #fff;
                font-weight: bold;
            }

            .login-link {
                position: absolute;
                top: 30px;
                right: 40px;
                color: #fff;
                text-decoration: none;
                font-size: 18px;
                cursor: pointer;
                transition: opacity 0.2s;
            }

            .login-link:hover {
                opacity: 0.7;
            }
        </style>
    </head>
    <body>
        <div class="header">Shared Noticeboard
            <p style="border: 2px solid #fff; margin: 0; user-select: none; width: 120px; padding: 5px;">&nbsp;JW<br> .ORG</p>
        </div>
    </body>
    <script>
        fetch('/api/protected', {
            method: 'GET',
            credentials: 'include'
        }).then(response => {
            const data = response.json();
            
            if (!response.ok) {
                window.location.href='/login';
            }
        });
    </script>
    </html>
   "#.to_string() 
}

