import React from "react";

const EmailContentPreview = ({ emailContent }) => {
  return (
    <section>
        <details open>
            <summary><strong>3. Email Content Preview</strong></summary>
            <div style={{color: "red"}}>TODO: Purify and display the contents as HTML. Note: the purified HTML will come from SAA</div>

            <div dangerouslySetInnerHTML={{__html: emailContent.html}}/>
            <strong>Suspicious Elements:</strong>
            <ul>
                {emailContent.suspiciousElements.map((element, index) => (
                    <li key={index}>{element}</li>
                ))}
            </ul>
        </details>
    </section>
  );
};

export default EmailContentPreview;
