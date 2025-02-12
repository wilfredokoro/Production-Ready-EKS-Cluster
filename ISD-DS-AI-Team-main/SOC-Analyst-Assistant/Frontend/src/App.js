import React, { useState, useRef, useEffect } from "react";
import HeaderSummary from "./components/HeaderSummary";
import EmailHeaderDetails from "./components/EmailHeaderDetails";
import EmailContentPreview from "./components/EmailContentPreview";
import LinksAndAttachments from "./components/LinksAndAttachments";
import RemediationActions from "./components/RemediationActions";
import SupplementalDetails from "./components/SupplementalDetails";
import AuditLog from "./components/AuditLog";
import ErrorBoundary from "./ErrorBoundary";
import "./App.css";

const App = () => {
  useEffect(() => {
    document.documentElement.style.height = "100%";
    document.documentElement.style.width = "100%";
    document.body.style.height = "100%";
    document.body.style.width = "100%";
    document.body.style.margin = "0";
    document.body.style.padding = "0";
    // document.body.style.overflow = "hidden"; // If needed, but may affect accessibility if you trap scrolling
  }, []);

  const [emailData, setEmailData] = useState(null);
  const [error, setError] = useState(null);
  const [isLoading, setIsLoading] = useState(false);
  const fileInputRef = useRef(null);
  const [responseText, setResponseText] = useState("");
  const uploadEndpoint = "http://localhost:5000/api/emails";

  const handleFileUpload = async (file) => {
    setIsLoading(true);
    try {
      const formData = new FormData();
      formData.append("file", file);

      const response = await fetch(uploadEndpoint, {
        method: "POST",
        body: formData,
      });

      if (!response.ok) {
        const text = await response.text();
        setResponseText(text);
        console.log(text);
        throw new Error(text);
      }

      const jsonResponse = await response.json();
      setEmailData(jsonResponse);
      setError(null);
    } catch (error) {
      if (error.message === "Failed to fetch") {
        setError("Connection refused: Is SOC Assist Server running?");
      } else {
        setError(error.message);
      }
      setEmailData(null);
    } finally {
      setIsLoading(false);
    }
  };

  const handleDrop = async (event) => {
    event.preventDefault();
    const file = event.dataTransfer.files[0];
    if (file) {
      await handleFileUpload(file);
    }
  };

  const handleDragOver = (event) => {
    event.preventDefault();
    event.dataTransfer.dropEffect = "copy";
  };

  // Trigger file input when the user clicks or keys 'Enter' or ' ' on the drop zone
  const triggerFileInput = () => {
    if (fileInputRef.current) {
      fileInputRef.current.click();
    }
  };

  const onDropZoneKeyDown = (e) => {
    if (e.key === "Enter" || e.key === " ") {
      e.preventDefault();
      triggerFileInput();
    }
  };

  const dividerPosition = 25; // Fixed position at one-fourth

  return (
    <div
      className="app-container"
      style={{
        display: "flex",
        flexDirection: "column",
        height: "100vh",
        width: "100vw",
        minHeight: "100vh", 
        minWidth: "100vw",  
        boxSizing: "border-box",
      }}
    >
      {/* Top Component */}
      <div
        id="drop-zone"
        className="drop-zone"
        onDragOver={handleDragOver}
        onDrop={handleDrop}
        // Make this div keyboard accessible
        tabIndex={0}
        role="button"
        aria-label="Upload file by clicking or dragging and dropping"
        onKeyDown={onDropZoneKeyDown}
        onClick={triggerFileInput}
        style={{
          padding: "20px",
          border: "2px dashed #333", 
          margin: "1rem",
          textAlign: "center",
          cursor: "pointer",
        }}
      >
        <h1>Spam Email Remediation Report</h1>
        <p>Drag and drop a file here or press Enter/Space or click to select a file.</p>
        <input
          type="file"
          ref={fileInputRef}
          id="file-upload"
          style={{ display: "none" }}
          aria-hidden="true"
          onChange={(e) => {
            if (e.target.files[0]) handleFileUpload(e.target.files[0]);
          }}
        />
      </div>

      {isLoading && <p>Loading...</p>}
      {error && <p style={{ color: "red" }}>{error}</p>}

      {/* Bottom Section */}
      <div style={{ display: "flex", flex: 1 }}>
        {/* Left Component */}
        <div
          style={{
            flex: `0 0 ${dividerPosition}%`,
            backgroundColor: "#f0f0f0",
            padding: "10px",
            overflow: "auto",
          }}
        >
          <h2>Emails Submitted for Analysis</h2>
          {/* Possibly populate with data, or placeholders */}
        </div>

        {/* Divider */}
        <div
          style={{
            flex: "0 0 5px",
            backgroundColor: "#ccc",
          }}
        />

        {/* Right Component */}
        {emailData && (
          <div
            className="email-report"
            style={{
              overflowX: "auto", 
              overflowY: "auto", 
            }}
          >
            <ErrorBoundary>
              <HeaderSummary
                response={emailData || "No summary available"}
              />
            </ErrorBoundary>
            <ErrorBoundary>
              <EmailHeaderDetails
                response={emailData || "No JSON response"}
              />
            </ErrorBoundary>
            <ErrorBoundary>
              {emailData.emailContent && (
                <EmailContentPreview emailContent={emailData.emailContent} />
              )}
            </ErrorBoundary>
            <ErrorBoundary>
              {emailData.linksAndAttachments && (
                <LinksAndAttachments
                  links={emailData.linksAndAttachments.links || []}
                  attachments={emailData.linksAndAttachments.attachments || []}
                />
              )}
            </ErrorBoundary>
            <ErrorBoundary>
              {emailData.remediationActions && (
                <RemediationActions actions={emailData.remediationActions || []} />
              )}
            </ErrorBoundary>
            <ErrorBoundary>
              {emailData && (
                <SupplementalDetails
                  userInteractions={emailData.supplementalDetails.userInteractions || []}
                  threatIntelligence={emailData.supplementalDetails.threatIntelligence || {}}
                  mitre={emailData.supplementalDetails.MITRE || {}}
                  incidentData={emailData}
                  links={emailData.linksAndAttachments.links || []}
                />
              )}
            </ErrorBoundary>
            <ErrorBoundary>
              {emailData.auditLog && Array.isArray(emailData.auditLog) ? (
                <AuditLog logs={emailData.auditLog} />
              ) : (
                <p>No audit logs available.</p>
              )}
            </ErrorBoundary>
          </div>
        )}
      </div>
    </div>
  );
};

export default App;
