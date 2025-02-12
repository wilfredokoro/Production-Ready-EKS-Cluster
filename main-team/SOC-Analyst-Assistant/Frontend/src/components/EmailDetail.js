import React from "react";

function EmailDetail({ email }) {
  return (
    <div className="email-detail">
      <h2>Email Detail</h2>
      <p><strong>Subject:</strong> {email.subject}</p>
      <p><strong>From:</strong> {email.sender}</p>
      <p><strong>Body:</strong> {email.body}</p>
      {email.attachments && (
        <div>
          <h3>Attachments:</h3>
          <ul>
            {email.attachments.map((attachment, index) => (
              <li key={index}>{attachment}</li>
            ))}
          </ul>
        </div>
      )}
    </div>
  );
}

export default EmailDetail;
