
import React, { useState } from 'react';
// import './EmailAnalysis.css';

const EmailAnalysis = () => {
    const [output, setOutput] = useState('');
    const [emailSubject, setEmailSubject] = useState('');
    const [loading, setLoading] = useState(false);

    const uploadEndpoint = 'http://localhost:5000/api/emails';

    const handleFileUpload = async (file) => {
        setLoading(true);
        try {
            const formData = new FormData();
            formData.append('file', file);

            const response = await fetch(uploadEndpoint, {
                method: 'POST',
                body: formData,
            });

            if (!response.ok) {
                // todo Retrieve the response body as text
                // todo currently errors are text/html?
                // todo do we prefer json?
                const text = await response.text();
                setResponseText(text);
                throw new Error(text);
            }
            // if (!response.ok) {
            //     throw new Error(`Upload failed: ${response.statusText}`);
            // }

            const jsonResponse = await response.json();

            setEmailSubject(jsonResponse.emailSubject || '[No Subject]');
            setOutput(JSON.stringify(jsonResponse, null, 2));
        } catch (error) {
            console.error(error);
            setOutput(`Error: ${error.message}`);
        } finally {
            setLoading(false);
        }
    };

    const handleDrop = (event) => {
        event.preventDefault();
        const files = event.dataTransfer.files;
        if (files.length > 0) {
            handleFileUpload(files[0]);
        }
    };

    const handleClick = () => {
        const fileInput = document.createElement('input');
        fileInput.type = 'file';
        fileInput.onchange = () => {
            if (fileInput.files.length > 0) {
                handleFileUpload(fileInput.files[0]);
            }
        };
        fileInput.click();
    };

    return (
        <div className="email-analysis">
            <h2>Email Analysis</h2>
            <div
                id="drag-drop-area"
                onDragOver={(e) => e.preventDefault()}
                onDrop={handleDrop}
                onClick={handleClick}
                style={{ border: '2px dashed #007BFF', padding: '20px', textAlign: 'center', cursor: 'pointer' }}
            >
                Drag and drop a file here or click to select
            </div>
            {loading && <div id="spinner-container">Loading...</div>}
            <h3>Email Subject: {emailSubject}</h3>
            <pre id="output">{output}</pre>
        </div>
    );
};

export default EmailAnalysis;
