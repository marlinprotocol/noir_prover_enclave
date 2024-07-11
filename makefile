PROJECTS := kalypso-listener generator-client 
all: $(PROJECTS)

kalypso-listener:
	@echo "Building kalypso-listener... "
	@cd dependencies/kalypso-unified && cargo build --target x86_64-unknown-linux-gnu --release -p listener
	@cp dependencies/kalypso-unified/target/x86_64-unknown-linux-gnu/release/listener kalypso-listener

generator-client:
	@echo "Building generator-client... "
	@cd dependencies/kalypso-unified && cargo build --target x86_64-unknown-linux-gnu --release -p generator_client
	@cp dependencies/kalypso-unified/target/x86_64-unknown-linux-gnu/release/generator-client generator-client


.PHONY: clone-repos
clone-repos:
	@echo "Cloning Repo"
	@mkdir -p dependencies 
	@cd dependencies && git clone https://github.com/marlinprotocol/kalypso-unified.git

.PHONY: pull-repo
pull-repo:   
	@echo "Pulling Repo"
	@cd dependencies/kalypso-generator && git pull
	@cd dependencies/kalypso-unified && git pull

.PHONY: clean
clean:
	@rm -f generator-client kalypso-generator
	@rm -rf dependencies
