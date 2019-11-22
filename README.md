# D4G-tools

## Build all the plateform like prod
After cloning the repository you can run
```
./prod_kickinstance.sh
```

It should launch the whole stack as in production if you have Docker up and running


## Build the whole stack via Docker

Clone the repository then run `docker build -t d4g .` in the root directory.

After the first build (wich may be slow) you can do
`docker run -p 80:8080 d4g` and then you will have the front at `http://localhost/`

## Build only some parts

Will be written later, maybe during D4G :P
