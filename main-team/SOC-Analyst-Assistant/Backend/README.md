# SOC Analyst Assistant

The **SOC Analyst Assistant** is a tool designed to assist Security Operations Center (SOC) analysts in analyzing uploaded files via a user-friendly web interface or CLI. It supports file analysis, drag-and-drop functionality, and can be run as a local server or a standalone command-line application.

---

## Features

- **Simple Interface **: Drag-and-Drop or Click to Upload files for analysis. 
- **REST API**: Host the application as a server to process file uploads via an API.
- **CLI Support**: Analyze files locally using command-line commands.
- **Live Analysis Feedback**: Provides real-time updates and results of file processing.

---

## Installation
- **docker compose up**

### Prerequisites
- **Docker** 
- **Web Access**

---

## Usage
```
Usage: serve_scan "<COMMAND>"

Commands:
serve  Start a server to process files via a REST API
scan   Scan a file locally
help   Print this message or the help of the given subcommand(s)

Options:
-h, --help     Print help
-V, --version  Print version
```
### **Server Function**
```
Usage: serve_scan serve [OPTIONS]

Options:
-u, --url <URL>    url of the REST API [default: 0.0.0.0]
-p, --port <PORT>  Port to run the server on [default: 8080]
-h, --help         Print help
```
### **Scan Function**
```
Usage: serve_scan scan [OPTIONS] --input-file <INPUT_FILE>

Options:
-i, --input-file <INPUT_FILE>    Path to the file to scan
-o, --output-file <OUTPUT_FILE>  Path to the output file to scan [default: output_file.txt]
-h, --help                       Print help
```

### notes

#### Email headers background
As you  know, an email has headers.  
Some of these headers originate with the email... things like to:, from:, cc:, etc.,
Other headers are added as the email is routed through the email system.  These are called transport headers. Transport headers are essential for system diagnostics, debugging, etc.
When forwarding an email for analysis you should always use the option: "forward as attachment". This ensures that all Transport Headers are included.
Transport headers are crucial for cybersecurity analysis.
Any system along the transport route can add headers, and many do, mostly it's things like: "time received"... 
One transport header is called: "Authentication-Results" which is added by email servers along the route.
The system that reports "spam" emails to the USCISSPAM mailbox does indeed "forward as attachement". It must. That's why we see the actual spam as .msg file attachments.
However, some system is adding transport headers that are not in the attached (.msg) email, but are attached to the actual forwarded email itself.


 


Notice here how "sender" is a scaler person but the "to" field is as vector of persons... there can be multiple "to"s
```
    sender: Person {
        name: "DollarSprout",
        email: "hello@dollarsprout.com",
    },
    to: [
        Person {
            name: "Hopkins, Sherona A",
            email: "sherona.a.hopkins@uscis.dhs.gov",
        },
    ],

```

### Example of how to post an email
```
curl --location --request POST 'localhost:5000/api/emails' \
--header 'Content-Type: multipart/form-data' \
--form 'file=@"/Users/thomasconley/tests/one.msg"' 
```
