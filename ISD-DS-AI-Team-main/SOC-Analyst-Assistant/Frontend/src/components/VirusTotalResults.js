// VirusTotal.jsx
import React from 'react';
import PropTypes from 'prop-types';

const VirusTotal = ({ data }) => {
  if (!data) {
    return <div>No analysis results available</div>;
  }

  // data is presumably { stats: { malicious, suspicious, ... }, vt_link: string }
  const { stats, vt_link } = data;
  if (!stats) {
    return <div>No stats in analysis results</div>;
  }

  const {
    malicious = 0,
    suspicious = 0,
    undetected = 0,
    harmless = 0,
    timeout = 0,
  } = stats;

  return (
    <div className="virus-total">
      <h3>VirusTotal Analysis</h3>
      <ul>
        <li><strong>Malicious:</strong> {malicious}</li>
        <li><strong>Suspicious:</strong> {suspicious}</li>
        <li><strong>Undetected:</strong> {undetected}</li>
        <li><strong>Harmless:</strong> {harmless}</li>
        <li><strong>Timeout:</strong> {timeout}</li>
      </ul>
    </div>
  );
};

VirusTotal.propTypes = {
  data: PropTypes.shape({
    stats: PropTypes.shape({
      malicious: PropTypes.number,
      suspicious: PropTypes.number,
      undetected: PropTypes.number,
      harmless: PropTypes.number,
      timeout: PropTypes.number,
    }),
    vt_link: PropTypes.string,
  }),
};

export default VirusTotal;
