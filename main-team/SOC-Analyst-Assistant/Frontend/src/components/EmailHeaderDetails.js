import React from "react";

const EmailHeaderDetails = ({ response }) => {
  // TODO figure out what goes into the Analysis / Summary and only display that stuff
  return (
  <section>
    <details open>
      <summary><strong>2. Analysis / Summary</strong></summary>
      <div>
        <ul>
          <li><strong style={{color: "red"}}>TODO: Figure out what should goe into the Analysis / Summary.</strong></li>
          <li>Incident ID: <lidata>{response.requestId}</lidata></li>
          <li>Detection Timestamp: <lidata>{response.detectionTimestamp}</lidata></li>
          <li>Status: <lidata>{response.status}</lidata></li>
          {/*<li>Email Subject: <lidata>{summary.emailSubject}</lidata></li>*/}
          {/*<li>Sender:</li>*/}
          <li>Risk Rating: <lidata>{response.riskRating}</lidata></li>
          <li>Observation(s):</li>
          <ul>
            {response.observations.map((observation, index) => (
                <li key={index}>
                  <lidata>{observation}</lidata>
                </li>
            ))}
          </ul>
          <li>Last Updated: <lidata>{response.lastUpdated}</lidata></li>
        </ul>
      </div>

      {/*<pre>*/}
      {/*{`*/}
      {/*From: ${headerDetails.from}*/}
      {/*To: ${headerDetails.to}*/}
      {/*CC: ${headerDetails.cc}*/}
        {/*Subject: ${headerDetails.subject}*/}
        {/*Date: ${headerDetails.date}*/}
        {/*Message-ID: ${headerDetails.messageId}*/}
        {/*Reply-To: ${headerDetails.replyTo}*/}
        {/*Return-Path: ${headerDetails.returnPath}*/}
        {/*Received: ${headerDetails.received}*/}
        {/*Authentication-Results: SPF=${headerDetails.authenticationResults.SPF}, DKIM=${headerDetails.authenticationResults.DKIM}, DMARC=${headerDetails.authenticationResults.DMARC}*/}
        {/*`}*/}
        {/*        </pre>*/}
        {/*<strong>Key Observations:</strong>*/}
        {/*<ul>*/}
        {/*  {response.observations.map((observation, index) => (*/}
        {/*      <li key={index}>*/}
        {/*        <lidata>{observation}</lidata>*/}
        {/*      </li>*/}
        {/*  ))}*/}
        {/*  /!*{headerDetails.observations.map((obs, index) => (*!/*/}
        {/*  /!*  <li key={index}>{obs}</li>*!/*/}
        {/*  /!*))}*!/*/}
        {/*</ul>*/}
      </details>
    </section>
  );
};

export default EmailHeaderDetails;
