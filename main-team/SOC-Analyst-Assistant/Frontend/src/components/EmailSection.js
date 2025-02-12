import React, { useState } from "react";

const EmailSection = ({ email }) => {
  const [isHeaderOpen, setIsHeaderOpen] = useState(true);
  const [isContentOpen, setIsContentOpen] = useState(true);
  const [isLinksOpen, setIsLinksOpen] = useState(true);
  const [isActionsOpen, setIsActionsOpen] = useState(true);
  const [isDetailsOpen, setIsDetailsOpen] = useState(true);

  return (
    <div className="email-section">
      {/* Header / Summary */}
      <section>
        {/* Use h2 or h3 based on hierarchy in the overall page */}
        <h2
          tabIndex={0}
          role="button"
          aria-expanded={isHeaderOpen}
          aria-controls="header-summary-content"
          onClick={() => setIsHeaderOpen(!isHeaderOpen)}
          onKeyDown={(e) => {
            if (e.key === "Enter" || e.key === " ") {
              e.preventDefault();
              setIsHeaderOpen(!isHeaderOpen);
            }
          }}
        >
          1. Header / Summary
        </h2>
        {isHeaderOpen && (
          <div id="header-summary-content">
            <p><strong>Incident ID:</strong> {email.incidentId}</p>
            <p><strong>Detection Timestamp:</strong> {email.detectionTimestamp}</p>
            <p><strong>Status:</strong> {email.status}</p>
            <p><strong>Email Subject:</strong> {email.emailSubject}</p>
            <p><strong>Sender:</strong> {email.sender.displayName} ({email.sender.emailAddress})</p>
            <p><strong>Risk Rating:</strong> {email.riskRating}</p>
            <p><strong>Recipient(s):</strong> {email.recipients.join(", ")}</p>
            <p><strong>Last Updated:</strong> {email.lastUpdated}</p>
          </div>
        )}
      </section>

      {/* Email Header Details */}
      <section>
        <h2
          tabIndex={0}
          role="button"
          aria-expanded={isContentOpen}
          aria-controls="header-details-content"
          onClick={() => setIsContentOpen(!isContentOpen)}
          onKeyDown={(e) => {
            if (e.key === "Enter" || e.key === " ") {
              e.preventDefault();
              setIsContentOpen(!isContentOpen);
            }
          }}
        >
          2. Email Header Details
        </h2>
        {isContentOpen && (
          <div id="header-details-content">
            <pre>{JSON.stringify(email.headerDetails, null, 2)}</pre>
          </div>
        )}
      </section>

      {/* Email Content Preview */}
      <section>
        <h2
          tabIndex={0}
          role="button"
          aria-expanded={isLinksOpen}
          aria-controls="email-content-preview"
          onClick={() => setIsLinksOpen(!isLinksOpen)}
          onKeyDown={(e) => {
            if (e.key === "Enter" || e.key === " ") {
              e.preventDefault();
              setIsLinksOpen(!isLinksOpen);
            }
          }}
        >
          3. Email Content Preview
        </h2>
        {isLinksOpen && (
          <div id="email-content-preview">
            {/* Avoid using dangerouslySetInnerHTML unless absolutely necessary,
                or at least sanitize the content. */}
            <p dangerouslySetInnerHTML={{ __html: email.emailContent.html }} />
            <p>
              <strong>Suspicious Elements:</strong>{" "}
              {email.emailContent.suspiciousElements.join(", ")}
            </p>
          </div>
        )}
      </section>

      {/* Links & Attachments */}
      <section>
        <h2
          tabIndex={0}
          role="button"
          aria-expanded={isActionsOpen}
          aria-controls="links-attachments"
          onClick={() => setIsActionsOpen(!isActionsOpen)}
          onKeyDown={(e) => {
            if (e.key === "Enter" || e.key === " ") {
              e.preventDefault();
              setIsActionsOpen(!isActionsOpen);
            }
          }}
        >
          4. Links &amp; Attachments
        </h2>
        {isActionsOpen && (
          <div id="links-attachments">
            <h3>Links (URLs)</h3>
            <ul>
              {email.linksAndAttachments.links.map((link, index) => (
                <li key={index}>
                  <a
                    href={link.url}
                    aria-label={`Link to ${link.url}. ${link.analysis}`}
                  >
                    {link.url}
                  </a>
                  <p>{link.analysis}</p>
                  <button aria-label={`Perform action: ${link.action}`}>
                    {link.action}
                  </button>
                </li>
              ))}
            </ul>

            <h3>Attachments</h3>
            <p>
              {email.linksAndAttachments.attachments.length === 0
                ? "No Attachments Detected"
                : ""}
            </p>
            {/* If there are attachments, you might also want to list them explicitly */}
          </div>
        )}
      </section>

      {/* Remediation Actions */}
      <section>
        <h2
          tabIndex={0}
          role="button"
          aria-expanded={isDetailsOpen}
          aria-controls="remediation-actions"
          onClick={() => setIsDetailsOpen(!isDetailsOpen)}
          onKeyDown={(e) => {
            if (e.key === "Enter" || e.key === " ") {
              e.preventDefault();
              setIsDetailsOpen(!isDetailsOpen);
            }
          }}
        >
          5. Remediation Actions
        </h2>
        {isDetailsOpen && (
          <div id="remediation-actions">
            <ul>
              {email.remediationActions.map((action, index) => (
                <li key={index}>{action}</li>
              ))}
            </ul>
          </div>
        )}
      </section>
    </div>
  );
};

export default EmailSection;
