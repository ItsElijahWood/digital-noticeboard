/// Return HTML to content variable
pub fn scotby_page() -> String {
   r#"
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>Scotby Notice Board</title>
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
        <div class="header">&nbsp;&nbsp;Scotby Noticeboard
            <div style="border: 2px solid #fff; user-select: none; width: 260px; padding: 5px;">
                <p style="margin: 0; font-size: 140px;">&nbsp;JW</p>
                <p style="margin: 0; font-size: 85px; padding-bottom: 10px;">&nbsp;.ORG</p>
            </div>
        </div>
        <div style="display: flex;">
        <div id="docs_container" style="padding: 100px; width: 90%; padding-top: 0 !important; display: flex; flex-wrap: wrap; gap: 40px; overflow-y: auto; height: 1600px; scrollbar-width: none;">
        </div>
        <div style="width: 300px; position: absolute; right: 0; display: flex; align-content: center; padding-right: 40px; text-align: center; gap: 40px; justify-content: center; flex-direction: column; height: 70%;">
            <a style="color: #fff; font-size: 100px; display: flex; align-items: center; height: 275px; border: 2px solid #fff; cursor: pointer; justify-content: center; text-decoration: none;" href="/">Back</a> 
            <img src="public/map_icon.png" style="border: 2px solid #fff; text-decoration: none; cursor: pointer; background-color: black;" width="300px" height="275px" onclick="window.location.href='/scotby_map'" />
        </div>
        </div>
    </body>
    <script>
        const container = document.getElementById("docs_container");

        fetch('/api/protected', {
            method: 'GET',
            credentials: 'include'
        }).then(response => {
            const data = response.json();
            
            if (!response.ok) {
                window.location.href='/login';
            }
        });

        fetch('/api/protected_img_fetch', {
            method: 'GET',
            credentials: 'include'
        }).then(response => {
            const data = response.json();
        });

       const previewDiv = document.createElement("div");
previewDiv.id = "imagePreview";
previewDiv.style.position = "fixed";
previewDiv.style.top = "50%";
previewDiv.style.left = "50%";
previewDiv.style.transform = "translate(-50%, -50%)";
previewDiv.style.backgroundColor = "rgba(0, 0, 0, 0.8)";
previewDiv.style.padding = "20px";
previewDiv.style.borderRadius = "10px";
previewDiv.style.display = "none";
previewDiv.style.zIndex = "9999";

const previewImg = document.createElement("img");
previewImg.style.maxWidth = "90vw";
previewImg.style.maxHeight = "90vh";
previewImg.style.borderRadius = "8px";

previewDiv.appendChild(previewImg);
document.body.appendChild(previewDiv);

previewDiv.addEventListener("click", () => {
    previewDiv.style.display = "none";
});

fetch('/api/fetch', {
    method: 'POST',
})
.then(response => response.json())
.then(data => {
    let num = 0;

    data.forEach(row => {
        const [file, type, congregation] = row;

        if (type === "scotby_notice_board") {
            const container = document.getElementById("docs_container");
            num += 1;

            const img = document.createElement("img");
            img.src = file;
            img.width = 570;
            img.height = 800;
            img.style.cursor = "pointer";

            img.onclick = () => {
                previewImg.src = file;
                previewDiv.style.display = "block";
            };

            container.appendChild(img);
        }
            }); 
        })
        .catch(error => {
            console.error("Fetch error:", error);
        }); 
    </script>
    </html>
   "#.to_string() 
}

