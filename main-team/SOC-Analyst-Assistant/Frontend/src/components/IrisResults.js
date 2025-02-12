// IrisResults.jsx
import React from "react";
import PropTypes from "prop-types";

const IrisResults = ({ data }) => {
  if (!data) {
    return <div>No Iris results available</div>;
  }

  // The data might look like { risk_score: number, dt_link: string }
  const { risk_score, dt_link } = data;

  return (
    <div className="iris-results">
      <h3>Iris Investigate Analysis</h3>
      <ul>
        <li><strong>Risk Score:</strong> {risk_score}</li>
      </ul>
      {/* dt_link is optional; if you have it in the payload, you could display it or not */}
    </div>
  );
};

IrisResults.propTypes = {
  data: PropTypes.shape({
    risk_score: PropTypes.number,
    dt_link: PropTypes.string,
  }),
};

export default IrisResults;
