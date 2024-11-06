# Url parser
This rust project provides functionality for parsing URL-addresses and its parts (like protocol, domains, parameters etc.) using rust-peg library.

# Description of the parsing process
A URL (Uniform Resource Locator) is the address of a unique resource on the internet. It is one of the key mechanisms used by browsers to retrieve published resources, such as HTML pages, CSS documents, images, and so on. A URL-address contains such parts:
![image info](./images/image.png)

Scheme (protocol)
:The scheme identifies the protocol to be used to access the resource on the Internet. It can be HTTP (without SSL), HTTPS (with SSL),
 FTP and so on. The list of official IANA-registered schemes:
