Rust Web Server
Team Members: Jimmy Ly and Arulselvan Madhavan

Build & Installation:
1. Go to https://github.com/brson/multirust to install multirust.
   Ideally, the following command should work:
   curl -sf https://raw.githubusercontent.com/brson/multirust/master/blastoff.sh | sh
   multirust essentially allows you to have several versions of Rust installed at
   once. In order to use the concurrent hash map library, we needed to use a nightly
   build of Rust.
2. Once multirust is installed, type the following commands in the terminal:
   multirust update nightly
   multirust default nightly
3. To build the Rust web server, run:
   cargo build
4. To run the Rust web server, run:
   cargo run [# of scheduler threads] [# of worker threads] [# of logger threads] [# cache threads]

   For example, to run with 2 scheduler threads, 5 worker threads, 1 logger thread, and 1 cache thread, you would run:
   cargo run 2 5 1 1
5. The server will be listening on localhost port 8080, so for example you can
   open the Chrome browser and go to localhost:8080/foo.html, and you should
   see a Hello, World! web page displayed.
   Note that it worked in the FireFox browser for OSX, but for some reason it does
   not seem to display correctly in the FireFox browser in an Ubuntu VM. Chrome
   seems to work in the Ubuntu VM, however. All browsers seemed to work on OSX.

Other notes:
The source code is in the /src directory. The /examplefiles directory
contains the files of various sizes used for benchmarking. The Cargo.toml
file contains configurations for the cargo package manager, including
dependencies of the project. The /target directory is the output directory
as usual.
