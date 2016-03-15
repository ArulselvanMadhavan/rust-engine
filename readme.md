This project is our attempt to build a web server in Rust that acts as a contemporary to nginx,apache,caddy.

A web server built in Rust would have many advantages including safety, high performance, and easy concurrency over existing web app frameworks. While many high performing applications are written in C and C++ to take advantage of the manual memory manipulation, the program is vulnerable to many memory-related issues involving segfaults, buffer overruns, and memory leaks. Writing applications in other languages, such as Java or Python, rely on built-in memory management that is often not optimized for the purpose of your program.
Building a web server in Rust would provide the advantages of both scenarios. Using Rust’s built-in system of smart pointers and references, many bugs that would be found (and difficult to debug) at runtime with a C or C++ program can be found and fixed at compile-time. This potentially makes building the system more extensible as the code base grows larger and more complicated since it can be less prone to newly introduced bugs.
Project Vision: Convert this web server into a web application framework.  

