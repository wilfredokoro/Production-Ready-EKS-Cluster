import React from "react";

function EmailList({ emails, onEmailSelect }) {
  return (
    <div className="email-list">
      <h2>Email List</h2>
      {emails.map((email) => (
        <div
          key={email.id}
          className="email-item"
          // Make sure it's accessible via keyboard
          tabIndex={0}
          role="button"
          aria-label={`Select email with subject: ${email.subject}`}
          onClick={() => onEmailSelect(email)}
          onKeyDown={(e) => {
            if (e.key === "Enter" || e.key === " ") {
              e.preventDefault();
              onEmailSelect(email);
            }
          }}
          style={{
            cursor: "pointer",
            border: "1px solid #ccc",
            padding: "8px",
            marginBottom: "8px",
          }}
        >
          <h3>{email.subject}</h3>
          <p>From: {email.sender}</p>
        </div>
      ))}
    </div>
  );
}

export default EmailList;
