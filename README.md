### MAGMA: A CLI Tool For Automating Spec Creation

## Installation & Usage

1. Copy the repo
2. Export GO path into working directory:
```bash
export GOPATH=$(go env GOPATH) 
export GOBIN=$GOPATH/bin 
export PATH="/opt/homebrew/opt/libpq/bin:$GOPATH:$GOBIN:$PATH" 
export DOCKER_DEFAULT_PLATFORM=linux/amd64
```

2. Install dependencies:
```bash
go install
```

3. Run CLI command with sample input file:
```bash
specautomator genspec input
```

4. Generated spec will be written into `output.json` file 



