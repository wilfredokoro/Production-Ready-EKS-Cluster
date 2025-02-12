import React from "react";

const AuditLog = ({ logs }) => {
  return (
    <section>
      <details open>
        <summary><strong>7. Audit Log</strong></summary>
        <table>
          <thead>
            <tr>
              <th>Timestamp</th>
              <th>User</th>
              <th>Action</th>
              <th>Notes</th>
            </tr>
          </thead>
          <tbody>
            {logs.map((log, index) => (
              <tr key={index}>
                <td>{log.timestamp}</td>
                <td>{log.user}</td>
                <td>{log.action}</td>
                <td>{log.notes}</td>
              </tr>
            ))}
          </tbody>
        </table>
      </details>
    </section>
  );
};

export default AuditLog;
