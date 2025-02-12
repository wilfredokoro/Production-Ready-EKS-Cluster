import React, { useEffect, useState } from "react";
import VirusTotal from "./VirusTotalResults";
import generateVirusTotalLink from "./GenerateVirusTotalLink";

const SupplementalDetails = ({ incidentData }) => {
  const [virusTotalData, setVirusTotalData] = useState({});
  const [errorUrls, setErrorUrls] = useState([]);
  const [isLoading, setIsLoading] = useState(false);

  console.log("Incident data received:", incidentData);

  const extractLinks = (data) => {
    if (!data) {
      console.error("No incident data provided.");
      return [];
    }
    return data.linksAndAttachments?.links || [];
  };

  const links = extractLinks(incidentData);

  console.log("Extracted Links:", links);

  useEffect(() => {
    const processLinksSequentially = async (linksArray) => {
      if (linksArray.length === 0) {
        console.error("No valid links provided for analysis");
        return;
      }

      setIsLoading(true);
      for (const link of linksArray) {
        if (!link.url) {
          console.warn("Skipping invalid link object without URL:", link);
          continue;
        }

        try {
          const response = await fetch('http://localhost:5000/api/virustotal', {
            method: 'POST',
            headers: {
              'Content-Type': 'application/json',
            },
            body: JSON.stringify({ url: link.url }),
          });

          if (!response.ok) {
            const errorText = await response.text();
            throw new Error(errorText || "Unknown error");
          }

          const data = await response.json();
          setVirusTotalData((prevData) => ({
            ...prevData,
            [link.url]: data,
          }));
        } catch (error) {
          console.error(`Failed to fetch VirusTotal data for ${link.url}:`, error);
          setErrorUrls((prev) => [...prev, link.url]);
          setIsLoading(false);
          return;
        }
      }
      setIsLoading(false);
    };

    processLinksSequentially(links);
  }, [links]);

  return (
    <section>
      <details open>
        <summary tabIndex="0">
          <strong>6. Supplemental Details</strong>
        </summary>
        <div>
          <h2 id="supplemental-details">Supplemental Details</h2>
          {isLoading && (
            <p aria-live="polite" aria-busy="true">Loading analysis...</p>
          )}
          {links.length > 0 ? (
            links.map((link, index) => (
              <div key={index} role="group" aria-labelledby={`link-${index}`}>
                <h4 id={`link-${index}`}>Analysis for {link.url}</h4>

                {/* Accessible VirusTotal results or error messages */}
                {virusTotalData[link.url] ? (
                  <VirusTotal data={virusTotalData[link.url]} />
                ) : errorUrls.includes(link.url) ? (
                  <p role="alert" style={{ color: "#D9534F" }}>
                    ⚠️ Failed to analyze this URL. Please check the server logs.
                  </p>
                ) : (
                  <p>Awaiting analysis...</p>
                )}

                {/* Accessible "View on VirusTotal" button */}
                <a
                  href={generateVirusTotalLink(link.url)}
                  target="_blank"
                  rel="noopener noreferrer"
                  className="virus-total-link"
                  aria-label={`View ${link.url} on VirusTotal`}
                >
                  View on VirusTotal
                </a>
              </div>
            ))
          ) : (
            <p>No links available for analysis.</p>
          )}
        </div>
      </details>

      {/* CSS for accessibility */}
      <style>
        {`
          .virus-total-link {
            display: inline-block;
            margin-top: 10px;
            padding: 8px 12px;
            background-color: #005a9c;
            color: #ffffff;
            text-decoration: none;
            border-radius: 4px;
            font-size: 16px;
            font-weight: bold;
          }

          .virus-total-link:focus, .virus-total-link:hover {
            background-color: #004280;
            outline: 2px solid #FFD700;
          }
        `}
      </style>
    </section>
  );
};

export default SupplementalDetails;
