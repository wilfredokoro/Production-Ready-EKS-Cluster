/**
 * Base64-encode the entire URL without padding, then build
 * the VirusTotal "URL" page link:
 *    https://www.virustotal.com/gui/url/<base64>
 */
const generateVirusTotalLink = (rawUrl) => {
    // 1. Base64-encode the URL. Use btoa in the browser:
    let encoded = btoa(rawUrl);
  
    // 2. Remove any trailing '=' signs (no padding).
    encoded = encoded.replace(/=+$/, "");
  
    // 3. Return the final link for a URL resource in VirusTotal
    return `https://www.virustotal.com/gui/url/${encoded}`;
  };
  
  export default generateVirusTotalLink;