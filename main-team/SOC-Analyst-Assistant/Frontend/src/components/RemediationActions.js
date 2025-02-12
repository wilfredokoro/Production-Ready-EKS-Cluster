import React from "react";

const RemediationActions = ({ actions }) => {
  return (
    <section>
        <details open>
            <summary><strong>5. Remediation Actions</strong></summary>
            <div style={{color: "red"}}>TODO: Need content!"
            </div>
            <ul>
                {actions.map((action, index) => (
                    <li key={index}>{action}</li>
                ))}
            </ul>
        </details>
    </section>
  );
};

export default RemediationActions;
