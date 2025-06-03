/// Return HTML to content variable
pub fn moorhouse_map_page() -> String {
   r#"
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>Moorhouse Territory Map</title>
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
                font-size: 135px;
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
        <div class="header">&nbsp;&nbsp;Moorhouse Territory Map
            <div style="border: 2px solid #fff; user-select: none; width: 260px; padding: 5px;">
                <p style="margin: 0; font-size: 140px;">&nbsp;JW</p>
                <p style="margin: 0; font-size: 85px; padding-bottom: 10px;">&nbsp;.ORG</p>
            </div>
        </div>
        <div style="display: flex;">
        <div id="docs_container" style="padding: 100px; width: 90%; padding-top: 0 !important; display: flex; flex-wrap: wrap; gap: 40px; overflow-y: auto; height: 1600px; scrollbar-width: none;">
            <!-- <img src="public/storage/moorhouse/dummy_map.png" width="90%" /> -->
        </div>
        <div style="width: 300px; position: absolute; right: 0; display: flex; align-content: center; padding-right: 40px; text-align: center; gap: 40px; justify-content: center; flex-direction: column; height: 70%;">
         <a style="color: #fff; font-size: 100px; display: flex; align-items: center; height: 275px; border: 2px solid #fff; cursor: pointer; justify-content: center; text-decoration: none;" href="/moorhouse">Back</a> 
        </div>
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

