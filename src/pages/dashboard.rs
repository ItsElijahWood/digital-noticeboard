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
                font-family: "Publicsans";
                background-color: #4a6da7;
            }

            @font-face {
                font-family: "Publicsans";
                src: url(public/public-sans.ttf);
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
        <div style="padding-left: 2rem;">
            <p style="color: #fff; font-size: 25px; font-family: 'Publicsans';">Shared Noticeboad</p>
            <input type="file" accept="application/pdf" id="pdfUpload" />
            <input type="text" value="shared_notice_board" id="shared_notice_board" readonly />
            <button type="button" onclick="uploadPdf()">Upload PDF</button>
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

        function pdf_pass_token() {
            fetch('/api/protected_pdf_pass', {
                method: 'GET',
                credentials: 'include'
            }).then(response => {
                const data = response.json();
            
                if (!response.ok) {
                    window.location.href='/login';
                }
            });
        }

        function uploadPdf() {
            const input = document.getElementById('pdfUpload');
            const type = document.getElementById('shared_notice_board').value;

            if (input.files.length === 0) {
                alert('Select a PDF file first.');
                return;
            }
            const file = input.files[0];
            
            if (file.type !== 'application/pdf') {
                alert('Only PDF files are allowed.');
                return;
            }

            const formData = new FormData();
            formData.append('pdf', file);
            formData.append('type', type);

            pdf_pass_token();

            fetch('/api/add_pdf', {
                method: "POST",
                body: formData 
            })
            .then(response => {
                if (!response.ok) {
                    throw new Error('Upload failed');
                }
                alert('Upload successful!');
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

