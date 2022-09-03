# Simple Static File Server

A static file distribution server for development using ActixWeb.
It is a binary file that runs on intel64 Linux environment.
It is simple to start, just place the fs binary in the root directory of the static file you want to distribute, and pass the RUN_PORT environment variable to start it.

```
$ RUN_PORT=3000 ./fs
```

After booting, you can access http://localhost:3000/index.html, etc.

```
$ tree
.
├── fs
├── index.html
├── css
│   └── app.css
├── js
│    └── app.js
├── images
│   ├── 1.jpg
│   ├── 2.jpg
│   └── 3.png

```

NOTE: Use only for development purposes. As it was developed for my own personal use, I assume no responsibility for its use.

# Build

```
$ cargo build --release
```

# SSL

* certs/key.pem (pricate key file)
* certs/cert.pem (certificate chain file)

```
$ RUN_PORT=3000 SSL_MODE=true ./fs
```
