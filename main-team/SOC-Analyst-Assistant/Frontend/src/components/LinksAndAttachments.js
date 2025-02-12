import React from "react";

const LinksAndAttachments = ({ links, attachments }) => {
  return (
    <section>
      <details open>
        {/* The summary itself is accessible by default. We can keep it as is or use a heading. */}
        <summary><strong>4. Links &amp; Attachments</strong></summary>

        <div style={{ color: "red" }}>
          TODO: List links like bullets as above and optionally disable
          them from being clickable for safety. Or provide a context menu.
        </div>

        {/* Table with a caption and scope for column headers */}
        <h4>Links (URLs)</h4>
        <table>
          <caption className="sr-only">
            List of link URLs, their analysis status, and recommended actions
          </caption>
          <thead>
            <tr>
              <th scope="col">URL</th>
              <th scope="col">Analysis</th>
              <th scope="col">Action</th>
            </tr>
          </thead>
          <tbody>
            {links.map((link, index) => (
              <tr key={index}>
                {/* 
                  If you want to keep the link clickable, 
                  consider making the link text more descriptive 
                  than just the raw URL for screen readers. 
                */}
                <td>
                  <a
                    href={link.url}
                    aria-label={`Link to ${link.url}. ${link.analysis || "No analysis."}`}
                  >
                    {link.url}
                  </a>
                </td>
                <td>{link.analysis}</td>
                <td>
                  {/* 
                    Provide a descriptive button label. 
                    For example: 
                  */}
                  <button aria-label={`Perform action: ${link.action}`}>
                    {link.action}
                  </button>
                </td>
              </tr>
            ))}
          </tbody>
        </table>

        <h4>Attachments</h4>
        {attachments.length === 0 ? (
          <p>No Attachments Detected</p>
        ) : (
          <ul>
            {attachments.map((attachment, index) => (
              <li key={index}>{attachment}</li>
            ))}
          </ul>
        )}
      </details>
    </section>
  );
};

export default LinksAndAttachments;
