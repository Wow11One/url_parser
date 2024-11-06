# Url parser
This Rust project provides functionality for parsing URLs and their components (such as protocol, domains, parameters, etc.) using the rust-peg library. It is designed for analyzing parts of a URL.

## Description of the parsing process
A URL (Uniform Resource Locator) is the address of a unique resource on the internet. It is one of the key mechanisms used by browsers to retrieve resources such as HTML pages, CSS documents, images, and more.
This library parses URL parts step-by-step, so it is crucial to understand the structure of a URL:

<img src="./images/url-structure.webp" alt="url-structure" style="border-radius: 25px;" />

<dl> 
<dt>
Scheme (protocol)
</dt>
<dd>
The scheme identifies the protocol to be used to access the resource on the Internet. It can be HTTP (without SSL), HTTPS (with SSL),
FTP and so on. 
The list of official IANA-registered schemes:
URI schemes registered with the IANA, both provisional and fully approved, are listed in its registry for Uniform Resource Identifier (URI) Schemes. These include well known ones like:

* file - File URI scheme
* ftp – File Transfer Protocol
* http – Hypertext Transfer Protocol
* https – Hypertext Transfer Protocol Secure
* imap – Internet Message Access Protocol
* irc – Internet Relay Chat
* nntp – Network News Transfer Protocol

As well as many lesser known schemes like:

* acap – Application Configuration Access Protocol
* icap – Internet Content Adaptation Protocol
* mtqp – Message Tracking Query Protocol (RFC3887)
* wss – Encrypted WebSocket connections

A scheme is required for a URL address.

For parsing purposes, created an enum of all official protocols and method that returns their string value.
The first rule of this parser checks whether a provided string is a valid URL scheme.
</dd>

<dt>
Domain name
</dt>
<dd>
: The domain name is the key part of the website address that tells you who is responsible for the content 
and ultimately whether it is legitimate or not. The domain name can have a list of subdomains, separated by a point.
A domain can have up to 500 subdomains. Each subdomain can be up to 255 characters long, but for multi level subdomains, each level can only be 63 characters long.
A domain name is required for a URL address.
The examples of a domain name: google.com, distedu.ukma.edu.ua.
</dd>

<dt>
Port
</dt>
<dd>
: A port number is a means of identifying an application or service on a server to which a message that comes over a network is to be forwarded.
A port number is a 16-bit unsigned integer that ranges from 0 to 65535. So this is how it is parsed.
A port value separated from a domain value by a colon.
A port can be optional.
The URL address with port example: http:://localhost:80.
</dd>

<dt>
Path
</dt>
<dd>
: The URL path represents the portion of the web address after the top-level domain. It includes the subdirectory and slug(s), giving the specific location of a page within the website's hierarchy.
The path can be optional.
Example: /page/new/3
</dd>

<dt>
Parameters
</dt>
<dd>
: (also known as “query strings”) are a way to structure additional information for a given URL. Parameters are added to the end of a URL after a ‘?’ symbol, and multiple parameters can be included when separated by the ‘&’ symbol.
Parameters can be optional.
Example: ?color=red&page=2
</dd>

<dt>
Fragment
</dt>
<dd>
: a string of characters that refers to a resource that is subordinate to another, primary resource. The primary resource is identified by a Uniform Resource Identifier (URI), and the fragment identifier points to the subordinate resource.
Fragment can be optional. The example of fragment: #fragment
</dd>
</dl>
The task is to create a parser that can understand all these optional and required parts of a URL and store them in a specific Rust structure.

