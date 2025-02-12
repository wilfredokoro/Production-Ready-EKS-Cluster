import React from "react";

// todo can this tree structure be generated automatically from the json?

const HeaderSummary = ({response}) => {
    // TODO figure out what goes into the Header / Summary and only display that stuff
    return (
        <section>
            <details open>
                <summary><strong>1. Header / Summary</strong></summary>
                <div style={{color: "red"}}>TODO: Figure out what should goe into the Header / Summary .</div>
                <div>
                    <ul>
                        <li>Incident ID: <lidata>{response.requestId}</lidata></li>
                        <li>Detection Timestamp: <lidata>{response.detectionTimestamp}</lidata></li>
                        <li>Status: <lidata>{response.status}</lidata></li>
                        <li>Email Subject: <lidata>{response.emailSubject}</lidata></li>
                        <li>Sender:</li>
                        <ul>
                            {/*TODO this is causing errors*/}
                            <li>Display Name: <lidata>{response.sender.displayName}</lidata></li>
                            <li>Email Address: <lidata>{response.sender.emailAddress}</lidata></li>

                        </ul>
                        <li>Risk Rating: <lidata>{response.riskRating}</lidata></li>
                        <li>Recipient(s):</li>
                        <ul>
                            {response.recipients.map((recipient, index) => (
                                <li key={index}>
                                    <lidata>{recipient}</lidata>
                                </li>
                            ))}
                        </ul>
                        <li>Last Updated: <lidata>{response.lastUpdated}</lidata></li>
                    </ul>
                </div>
            </details>
        </section>
    );
};

export default HeaderSummary;
