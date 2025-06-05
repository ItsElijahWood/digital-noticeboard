pub fn dashboard_page() -> String {
    r#"
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>Dashboard</title>
        <link rel="icon" type="image/png" href="public/favicon.png">
        <style>
            body {
                margin: 0;
                font-family: 'Arial', sans-serif;
                background-color: #4a6da7;
            }

            .header {
                top: 20px;
                left: 40px;
                font-size: 48px;
                color: #fff;
                font-weight: bold;
                padding: 30px;
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

            input[type="file"],
            input[type="text"] {
                display: block;
                margin: 10px 0;
                padding: 10px;
                font-size: 16px;
                border: 1px solid #ccc;
                border-radius: 6px;
                background-color: #fff;
                color: #333;
                width: 300px;
            }

            button {
                padding: 10px 20px;
                font-size: 16px;
                border: none;
                border-radius: 6px;
                background-color: #2e5ea3;
                color: white;
                cursor: pointer;
                transition: background-color 0.2s;
            }

            button:hover {
                background-color: #234a82;
            }
        </style>
    </head>
    <body>
        <div class="header">Dashboard</div>
        <p style="padding-left: 2rem; color: #fff; font-size: 40px; font-family: 'Arial', sans-serif;">Upload files</p>
       <div id="container_upload" style="display: flex; flex-wrap: wrap;">
        <div style="padding-left: 2rem;">
            <p style="color: #fff; font-size: 25px; font-family: 'Arial', sans-serif;">Shared Noticeboad</p>
            <input type="file" accept="image/png" id="imgUpload" />
            <button type="button" onclick="uploadPngSharedNoticeboard()">Upload PNG</button>
        </div>
      </div> 
    </body>
    <script>
        fetch('/api/protected', {
            method: 'GET',
            credentials: 'include'
        }).then(async response => {
            const data = await response.json();
            
            if (!response.ok) {
                window.location.href='/login';
            }
        });

        fetch('/api/fetch_dashboard_congregation', {
    method: 'POST',
    credentials: 'include' 
}).then(async response => await response.json())
.then(data => {
    if (data === "scotby") {
        const container = document.getElementById("container_upload");

        // Check if it's already added
        if (!document.getElementById("scotbyUploadDiv")) {
            const scotbyDiv = document.createElement("div");
            scotbyDiv.id = "scotbyUploadDiv";
            scotbyDiv.style.paddingLeft = "2rem";

            const paragraph = document.createElement("p");
            paragraph.textContent = "Scotby Noticeboad";
            paragraph.style.color = '#fff';
            paragraph.style.fontSize = "25px";
            paragraph.style.fontFamily = "'Arial', sans-serif";

            const fileInput = document.createElement("input");
            fileInput.type = "file";
            fileInput.accept = "image/png";
            fileInput.id = "imgUploadSC";

            const button = document.createElement("button");
            button.type = "button";
            button.textContent = "Upload PNG";
            button.onclick = function() {
                uploadPngScotbyNoticeboard();
            };

            scotbyDiv.appendChild(paragraph);
            scotbyDiv.appendChild(fileInput);
            scotbyDiv.appendChild(button);

            container.appendChild(scotbyDiv);
        }
    } else {
        console.error(`error check failed: ${data}`);
    }

    if (data === "brampton") {
        const container = document.getElementById("container_upload");

if (!document.getElementById("bramptonUploadDiv")) {
    const bramptonDiv = document.createElement("div");
    bramptonDiv.id = "bramptonUploadDiv";
    bramptonDiv.style.paddingLeft = "2rem";

    const paragraph = document.createElement("p");
    paragraph.textContent = "Brampton Noticeboad";
    paragraph.style.color = '#fff';
    paragraph.style.fontSize = "25px";
    paragraph.style.fontFamily = "'Arial', sans-serif";

    const fileInput = document.createElement("input");
    fileInput.type = "file";
    fileInput.accept = "image/png";
    fileInput.id = "imgUploadBC";

    const button = document.createElement("button");
    button.type = "button";
    button.textContent = "Upload PNG";
    button.onclick = function() {
        uploadPngBramptonNoticeboard();
    };

    bramptonDiv.appendChild(paragraph);
    bramptonDiv.appendChild(fileInput);
    bramptonDiv.appendChild(button);

    container.appendChild(bramptonDiv);
}
    }

    if (data === "moorhouse") {
        const container = document.getElementById("container_upload");

if (!document.getElementById("moorhouseUploadDiv")) {
    const moorhouseDiv = document.createElement("div");
    moorhouseDiv.id = "moorhouseUploadDiv";
    moorhouseDiv.style.paddingLeft = "2rem";

    const paragraph = document.createElement("p");
    paragraph.textContent = "Moorhouse Noticeboad";
    paragraph.style.color = '#fff';
    paragraph.style.fontSize = "25px";
    paragraph.style.fontFamily = "'Arial', sans-serif";

    const fileInput = document.createElement("input");
    fileInput.type = "file";
    fileInput.accept = "image/png";
    fileInput.id = "imgUploadMC";

    const button = document.createElement("button");
    button.type = "button";
    button.textContent = "Upload PNG";
    button.onclick = function() {
        uploadPngMoorhouseNoticeboard();
    };

    moorhouseDiv.appendChild(paragraph);
    moorhouseDiv.appendChild(fileInput);
    moorhouseDiv.appendChild(button);

    container.appendChild(moorhouseDiv);
}
    }
});
 

        function uploadPngSharedNoticeboard() {
            const input = document.getElementById('imgUpload');

            if (input.files.length === 0) {
                alert('Select a PNG file first.');
                return;
            }
            const file = input.files[0];
            
            if (file.type !== 'image/png') {
                alert('Only PNG files are allowed.');
                return;
            }

            const formData = new FormData();
            formData.append('img', file);
            formData.append('type', 'shared_notice_board');

            fetch('/api/add_img', {
                method: "POST",
                credentials: "include",
                body: formData 
            })
            .then(response => {
                if (!response.ok) {
                    throw new Error('Upload failed');
                }
                alert('Upload successful!');
                window.location.href = '/dashboard';
            })
            .catch(error => {
                alert('Error uploading file.');
                console.error(error);
            }); 
        }

        function uploadPngBramptonNoticeboard() {
            const input = document.getElementById('imgUploadBC');

            if (input.files.length === 0) {
                alert('Select a PNG file first.');
                return;
            }
            const file = input.files[0];
            
            if (file.type !== 'image/png') {
                alert('Only PNG files are allowed.');
                return;
            }

            const formData = new FormData();
            formData.append('img', file);
            formData.append('type', 'brampton_notice_board');

            fetch('/api/add_img', {
                method: "POST",
                credentials: "include",
                body: formData 
            })
            .then(response => {
                if (!response.ok) {
                    throw new Error('Upload failed');
                }
                alert('Upload successful!');
                window.location.href = '/dashboard';
            })
            .catch(error => {
                alert('Error uploading file.');
                console.error(error);
            }); 
        }

        function uploadPngMoorhouseNoticeboard() {
            const input = document.getElementById('imgUploadMC');

            if (input.files.length === 0) {
                alert('Select a PNG file first.');
                return;
            }
            const file = input.files[0];
            
            if (file.type !== 'image/png') {
                alert('Only PNG files are allowed.');
                return;
            }

            const formData = new FormData();
            formData.append('img', file);
            formData.append('type', 'moorhouse_notice_board');

            fetch('/api/add_img', {
                method: "POST",
                credentials: "include",
                body: formData 
            })
            .then(response => {
                if (!response.ok) {
                    throw new Error('Upload failed');
                }
                alert('Upload successful!');
                window.location.href = '/dashboard';
            })
            .catch(error => {
                alert('Error uploading file.');
                console.error(error);
            }); 
        }

        function uploadPngScotbyNoticeboard() {
            const input = document.getElementById('imgUploadSC');

            if (input.files.length === 0) {
                alert('Select a PNG file first.');
                return;
            }
            const file = input.files[0];
            
            if (file.type !== 'image/png') {
                alert('Only PNG files are allowed.');
                return;
            }

            const formData = new FormData();
            formData.append('img', file);
            formData.append('type', 'scotby_notice_board');

            fetch('/api/add_img', {
                method: "POST",
                credentials: "include",
                body: formData 
            })
            .then(response => {
                if (!response.ok) {
                    throw new Error('Upload failed');
                }
                alert('Upload successful!');
                // window.location.href = '/dashboard';
            })
            .catch(error => {
                alert('Error uploading file.');
                console.error(error);
            }); 
        }
    </script>
    </html>
   "#
    .to_string()
}

